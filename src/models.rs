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
    pub uname: String,
    pub exercise_type: i32,
    pub description: String,
    pub notes: String,
    pub create_time: SystemTime,
    pub complete_time: SystemTime,
    pub created_id: i32,
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

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Workout {
    pub id: i32,
    pub origin_id: i32,
    pub uname: String,
    pub description: String,
    pub notes: String,
    pub created_time: SystemTime,
    pub completed_time: SystemTime,
    pub created_id: i32,
    pub completed_id: i32
}
