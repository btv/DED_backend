use serde::{Deserialize, Serialize};
use std::time::SystemTime;


use crate::schema::set;

#[derive(DbEnum)]
pub enum Type {
    Normal,
    Warmup,
    Cooldown,
    Drop,
    Fail
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Set {
    pub id: i32,
    pub exercise_id: i32,
    pub type: Type,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: String,
    pub description: String,
    pub created_or_completed: SystemTime,
    pub completed_reps: i16,
    pub completed_value: String
}
