import psycopg2
import os
import sys
from zipfile import ZipFile

DBURL = 'DATABASE_URL'
DATA_FILE = 'DED_EXERCISE_FILE'
EX_TYP = {}
BAD_LIST = []


def get_latest_update(cursor, file_path):
    query = """select blob, create_time from rawdata order by create_time desc limit 1 """
    blob = None
    try:
        cursor.execute(query)
        blob = cursor.fetchone()
        cursor.close()
    except (Exception, psycopg2.DatabaseError) as error:
        print(error)

    tmpFile = file_path + '/tmp.zip'
    if (blob == None):
        print("No data in database exiting\n")
        exit(-1)

    open(tmpFile, 'wb').write(blob[0])

    data = None
    create_time = blob[1]

    with ZipFile(tmpFile, 'r') as zip:
        filename = zip.filelist[0].filename
        data = str(zip.read(filename)).split('"')
    if (len(data) > 1):
        return [data[1].split('\\n'), filename, create_time]
    return None


def get_exercise_type(e_type):
    if e_type not in EX_TYP:
        EX_TYP[e_type] = len(EX_TYP)
    return EX_TYP[e_type]


def get_lines():
    DF = os.getenv(DATA_FILE)
    if (DF == None):
        print(f"Environment Variable {DATA_FILE}  is not set.  Exiting\n")
        sys.exit(-1)
    with open(DF, 'r') as f:
        read_data = f.readlines()
    return read_data


def update(conn, cursor, data, cntr):
    ins = """ INSERT INTO EXERCISES (ID, ORIGIN_ID, WORKOUT_ID, name, exercise_type, description, notes)  VALUES (%s, %s, %s, %s, %s, %s, %s)"""

    ln = data
    insert_data = None
    try:
        dta = ln.split(', ')
        if len(dta) > 1:
            ext = dta[1].rstrip()
            exercise_type = 'strength'
            origin_id = 99999
            id = workout_id = cntr
            name = dta[0]
            description = f"{ext}: {dta[0]}"
            notes = " "
            insert_data = (id, origin_id, workout_id, name, exercise_type, description, notes)
            cursor.execute(ins, insert_data)
            conn.commit()

    except (Exception, psycopg2.Error) as error:
        if (conn):
            print(f"Cannot insert  {insert_data} record error is {error} \n")
            BAD_LIST.append(insert_data)
            conn.rollback()


def main():
    cntr = 0
    loginsert = """INSERT INTO PROCESSLOG (create_time, LOG) values (%s, %s)"""
    PSQL_ENV = os.getenv(DBURL)
    if (PSQL_ENV == None):
        print(f"Environment variable {DBURL} is not set exiting")
        sys.exit(-1)

    tmp = PSQL_ENV.split('://')
    dbhost = tmp[0]
    tmp1 = tmp[1].split(':@')
    uname = tmp1[0]
    tmp2 = tmp1[1].split('/')
    hostname = tmp2[0]
    dbname = tmp2[1]
    print(f"env is {PSQL_ENV}\n")
    print(f"dbhost {dbhost}\n")
    print(f"uname {uname}\n")
    print(f"hostname {hostname}\n")
    print(f"dbname {dbname}\n")

    DF = os.getenv(DATA_FILE)

    data_file_elements = DF.split('/')

    data_file_path = '/'.join(data_file_elements[0:len(data_file_elements) - 1])

    print(data_file_path)
    filename = None
    create_time = None
    conn = None

    try:
        conn = psycopg2.connect(user=uname, host=hostname, database=dbname)
        cursor = conn.cursor()
        [data, filename, create_time] = get_latest_update(cursor, data_file_path)
        cursor = conn.cursor()
        for line in data:
            update(conn, cursor, line, cntr)
            cntr += 1

    except (Exception, psycopg2.Error) as error:
        print(f"Cannot connect is {error} \n")

    if conn is not None and len(BAD_LIST) == 0:
        sql = 'SELECT count(*) from exercises ;'
        cursor = conn.cursor()
        cursor.execute(sql)
        count = cursor.fetchone()
        cursor.close()
        print(f"wrote {count[0]} lines\n")
        if filename:
            logdata = f"processed {filename}, from {DF}  at {create_time} wrote {count[0]} lines "
            print(f"{logdata}\n")
            try:
                cursor = conn.cursor()
                ins_data = (create_time, logdata)
                cursor.execute(loginsert, ins_data)
                conn.commit();
                cursor.close()
            except (Exception, psycopg2.Error) as error:
                print(f"cannot insert into processlog {error} \n")
    else:
        if filename:
            logdata = f"processed  {len(BAD_LIST)} errors from {filename}, from {DF}  at {create_time} check console log for more data "
            print(f"{logdata}\n")
            try:
                cursor = conn.cursor()
                ins_data = (create_time, logdata)
                cursor.execute(loginsert, ins_data)
                conn.commit();
                cursor.close()
            except (Exception, psycopg2.Error) as error:
                print(f"cannot insert into processlog {error} \n")

        conn.close()


if __name__ == '__main__':
    main()
