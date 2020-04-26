import psycopg2
import os
import sys

DBURL = 'DATABASE_URL'
DATA_FILE = 'DED_EXERCISE_FILE'


def read_and_write_blob(uname, hostname, dbname):
    DF = os.getenv(DATA_FILE)
    if (DF == None):
        print(f"Environment Variable {DATA_FILE}  is not set.  Exiting\n")
        sys.exit(-1)
    conn = None
    ins = """ INSERT INTO rawdata (blob,file_size)  VALUES (%s, %s )"""
    try:
        fd = open(DF, 'rb').read()
        conn = psycopg2.connect(user=uname, host=hostname, database=dbname)
        cursor = conn.cursor()
        insert_data = (fd, len(fd))
        cursor.execute(ins, insert_data)
        conn.commit()
        cursor.close()
    except (Exception, psycopg2.DatabaseError) as error:
        print(error)

    if (conn):
        cursor.close()
        conn.close()


def main():
    print("Testing\n")
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

    read_and_write_blob(uname, hostname, dbname)


if __name__ == '__main__':
    main()
