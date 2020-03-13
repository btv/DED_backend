#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{User, NewUser};
use self::models::{Workout, NewWorkout};
use self::models::{Exercise, NewExercise};
use self::models::{Set, NewSet};
use std::sync::mpsc::SyncSender;
use std::time::SystemTime;
use diesel::RunQueryDsl;




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
                       email: &'a str, salt: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
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

#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl UserList{
    pub fn get_users()->UserList{
        use crate::schema::users::dsl::*;
        let connection = establish_connection();


        let results = users.load::<User>(&connection).expect("Error retreiving users");
        return UserList(results);
    }
}

pub fn create_workout<'a>(conn: &PgConnection, id: &'a i32, origin_id: &'a i32, exercise: &'a i32,
                          fname: &'a str, description: &'a str, notes: &'a str,
                          created_time: &'a SystemTime, completed_time: &'a SystemTime,
                          create_id: &'a i32, completed_id: &'a i32) -> Workout {
    use schema::workouts;
    let new_workout = NewWorkout {
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

    diesel::insert_into(workouts::table)
        .values(&new_workout)
        .get_result(conn)
        .expect("Error adding new workout")
}

pub fn create_exercise<'a>(conn: &PgConnection, id: &'a i32, origin_id: &'a i32,
                           fname: &'a str, exercise_type: &'a i32,
                           description: &'a str, notes: &'a str,
                           create_time: &'a SystemTime,
                           complete_time: &'a SystemTime,
                           create_id: &'a i32,
                           completed_id: &'a i32, ) -> Exercise {
    use schema::exercises;
    let new_exercise = NewExercise {
        id: id,
        origin_id: origin_id,
        fname: fname,
        exercise_type: exercise_type,
        description: description,
        notes: notes,
        create_time: create_time,
        complete_time: complete_time,
        create_id: create_id,
        completed_id: completed_id,
    };

    diesel::insert_into(exercises::table)
        .values(&new_exercise)
        .get_result(conn)
        .expect("Error adding new Exercise")

}

pub fn create_set<'a>(conn: &PgConnection,     id: &'a i32,
                       exercise_id: &'a i32,
                       style: &'a String,
                       unit: &'a String,
                       goal_reps: &'a i16,
                       goal_value: &'a String,
                       description: &'a String,
                       created_or_completed: &'a SystemTime,
                       completed_reps: &'a i16,
                       completed_value: &'a String,
                      )->Set{

    use schema::sets;
    let new_set = NewSet{
        id: id,
        exercise_id: exercise_id,
        style:style,
        unit: unit,
        goal_reps:goal_reps,
        goal_value:goal_value,
        description: description,
        created_or_completed: created_or_completed,
        completed_reps: completed_reps,
        completed_value: completed_value,
    };

    diesel::insert_into(sets::table)
        .values(new_set)
        .get_result(conn)
        .expect("Error adding new set")

}
