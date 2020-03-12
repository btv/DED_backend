#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{User, NewUser};
use self::models::{Workout, NewWorkout};
use std::sync::mpsc::SyncSender;
use std::time::SystemTime;


#[cfg(test)]
extern crate num_traits;

pub mod appconfig;
pub mod handlers;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn create_user<'a>(conn: &PgConnection, id: &'a i32, username: &'a str, fname: &'a str,
                       email: &'a str, salt: &'a str )->User{
    use schema::users;

    let new_user = NewUser{
        id: id,
        username: username,
        fname: fname,
        email: email,
        salt: salt,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error adding new user")
}
pub fn create_workout<'a>(conn: &PgConnection, id: &'a i32, origin_id: &'a i32, exercise: &'a i32,
                            fname: &'a str, description: &'a str, notes: &'a str,
                          created_time: &'a SystemTime, completed_time: &'a SystemTime,
                          create_id: &'a i32, completed_id: &'a i32)->Workout{
    use schema::workouts;
    let new_workout = NewWorkout{
        id: id,
        origin_id: origin_id,
        exercise: exercise,
        fname: fname,
        description: description,
        notes: notes,
        created_time: created_time,
        completed_time: completed_time,
        create_id: create_id,
        completed_id: completed_id,
    };

    diesel::insert_into( workouts::table)
        .values(&new_workout)
        .get_result(conn)
        .expect("Error adding new workout")

}
