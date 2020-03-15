use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::exercises;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="exercises"]
pub struct NewExercise {
    pub id: i32,
    pub origin_id: i32,
    pub fname: String,
    pub exercise_type: i32,
    pub description: String,
    pub notes: String,
    pub create_time: SystemTime,
    pub complete_time: SystemTime,
    pub create_id: i32,
    pub completed_id: i32,
}

impl NewExercise {
    pub fn create(&self) -> Result<Exercise, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::establish_connection;

       let connection = establish_connection();
        diesel::insert_into(exercises::table)
            .values(self)
            .get_result(&connection)
    }
}
