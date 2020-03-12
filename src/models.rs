use serde::Serialize;
use std::time::SystemTime;

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Set {
    pub id: i32,
    pub exercise_id: i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: String,
    pub description: String,
    pub created_or_completed: SystemTime,
    pub completed_reps: i16,
    pub completed_value: String,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Exercise {
    pub id: i32,
    pub origin_id: i32,
    pub set_id: i32,
    pub fname: String,
    pub exercise_type: i32,
    pub description: String,
    pub notes: String,
    pub create_time: SystemTime,
    pub complete_time: SystemTime,
    pub create_id: i32,
    pub completed_id: i32,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fname: String,
    pub email: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a>{
    pub id: &'a i32,
    pub username: &'a str,
    pub fname: &'a str,
    pub email: &'a str,
    pub salt: &'a str,
}



#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Workout {
    pub id: i32,
    pub origin_id: i32,
    pub exercise: i32,
    pub fname: String,
    pub description: String,
    pub notes: String,
    pub created_time: SystemTime,
    pub completed_time: SystemTime,
    pub create_id: i32,
    pub completed_id: i32,
}

#[derive(Insertable)]
#[table_name="workouts"]
pub struct NewWorkout<'a>{
    pub id: &'a i32,
    pub origin_id: &'a i32,
    pub exercise: &'a i32,
    pub fname: &'a str,
    pub description: &'a str,
    pub notes: &'a str,
    pub created_time: &'a SystemTime,
    pub completed_time: &'a SystemTime,
    pub create_id: &'a i32,
    pub completed_id: &'a i32,

}