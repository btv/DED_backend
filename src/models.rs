use serde::Serialize;
use std::time::SystemTime;

use crate::schema::sets;

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
    pub completed_value: String
}
