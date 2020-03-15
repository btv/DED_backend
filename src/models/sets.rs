use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::sets;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="sets"]
pub struct NewSet {
    pub exercise_id: i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: String,
    pub description: String,
    pub completed_reps: i16,
    pub completed_value: String,
}

impl NewSet {
    pub fn create(&self) -> Result<Set, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::establish_connection;

       let connection = establish_connection();
        diesel::insert_into(sets::table)
            .values(self)
            .get_result(&connection)
    }
}
