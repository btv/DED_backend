use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::workouts;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="workouts"]
pub struct NewWorkout {
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
