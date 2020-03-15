use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Queryable)]
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

#[derive(Insertable)]
#[table_name="sets"]
pub struct NewSet<'a>{
    pub id: &'a i32,
    pub exercise_id: &'a i32,
    pub style: &'a String,
    pub unit: &'a String,
    pub goal_reps: &'a i16,
    pub goal_value: &'a String,
    pub description: &'a String,
    pub created_or_completed: &'a SystemTime,
    pub completed_reps: &'a i16,
    pub completed_value: &'a String,
}
