## We are using [Diesel](http://diesel.rs/)  

Some additional command line checks


to 
To see the tables created:

diesel print-schema --database-url=$DATABASE_URL


addl commands

cargo install diesel_cli

diesel migration run

diesel setup

diesel migration redo 

diesel database reset:

Resets your database by dropping the database specified in your DATABASE_URL and then running `diesel
             database setup
             
diesel database setup:

Creates the database specified in your DATABASE_URL, and then runs any existing migrations.