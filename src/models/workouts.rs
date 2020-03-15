use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::*;

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
