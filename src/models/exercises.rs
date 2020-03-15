use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::*;

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

#[derive(Insertable)]
#[table_name="exercises"]
pub struct NewExercise<'a>{
    pub id: &'a i32,
    pub origin_id: &'a i32,
    pub fname: &'a str,
    pub exercise_type: &'a i32,
    pub description: &'a str,
    pub notes: &'a str,
    pub create_time: &'a SystemTime,
    pub complete_time: &'a SystemTime,
    pub create_id: &'a i32,
    pub completed_id: &'a i32,
}

