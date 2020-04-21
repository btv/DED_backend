import psycopg2
import os
import sys

DBURL='DATABASE_URL'
DATA_FILE='DED_EXERCISE_FILE'
EX_TYP={}


def get_exercise_type(e_type):
    if e_type not in EX_TYP:
        EX_TYP[e_type] = len(EX_TYP)
    return EX_TYP[e_type]

def get_lines():
    DF = os.getenv(DATA_FILE)
    if (DF ==None):
        print(f"Environment Variable {DATA_FILE}  is not set.  Exiting\n")
        sys.exit(-1)
    with open(DF,'r') as f:
        read_data = f.readlines()
    return read_data

def main():
    PSQL_ENV = os.getenv(DBURL)
    if (PSQL_ENV == None):
        print(f"Environment variable {DBURL} is not set exiting")
        sys.exit(-1)

    tmp = PSQL_ENV.split('://')
    dbhost = tmp[0]
    tmp1 = tmp[1].split(':@')
    uname = tmp1[0]
    tmp2= tmp1[1].split('/')
    hostname = tmp2[0]
    dbname = tmp2[1]
    print(f"env is {PSQL_ENV}\n")
    print(f"dbhost {dbhost}\n")
    print(f"uname {uname}\n")
    print(f"hostname {hostname}\n")
    print(f"dbname {dbname}\n")

    ins = """ INSERT INTO EXERCISES (ID, ORIGIN_ID, WORKOUT_ID, name, exercise_type, description, notes)  VALUES (%s, %s, %s, %s, %s, %s, %s)"""

    cntr = 0

    try:
        conn = psycopg2.connect(user=uname, host=hostname, database=dbname)
        cursor = conn.cursor()
        for ln in get_lines():
            dta = ln.split(', ')
            if len(dta) >1:
                ext = dta[1].rstrip()
                exercise_type = get_exercise_type(ext)
                origin_id = 99999
                id = workout_id = cntr
                name = dta[0]
                description = f"{ext}: {dta[0]}"
                notes = " "
                insert_data = (id, origin_id, workout_id, name, exercise_type, description, notes)
                cursor.execute(ins, insert_data)
                conn.commit()
                cntr += 1

    except (Exception, psycopg2.Error) as error:
        if (conn):
            print(f"Cannot insert record error is {error} \n")

    if(conn):
        sql = 'SELECT count(*) from exercises ;'
        cursor.execute(sql);
        count = cursor.fetchone()
        print(f"wrote {count[0]} lines\n")
        cursor.close()
        conn.close()


if __name__ == '__main__':
    main()