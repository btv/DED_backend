#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use diesel::{PgConnection,RunQueryDsl};
use crate::schema::exercises;
use diesel::query_dsl::filter_dsl::FindDsl;

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
#[table_name = "exercises"]
pub struct NewExercise {
    pub origin_id: i32,
    pub set_id: i32,
    pub fname: String,
    pub exercise_type: i32,
    pub description: String,
    pub notes: String,
}


impl NewExercise {
    pub fn create(&self, connection: &PgConnection) -> Result<Exercise, diesel::result::Error> {
        diesel::insert_into(exercises::table)
            .values(self)
            .get_result(connection)
    }
}

impl Exercise {
    pub fn get_exercise_by_id(id: i32, conn: &PgConnection) -> Result<Exercise, diesel::result::Error> {
        exercises::table.find(id).get_result(conn)
    }
}


#[cfg(test)]
mod tests {
    use std::time::{SystemTime, Duration};
    use crate::models::exercises::Exercise;

    #[test]
    fn test_new_exercise_structure() {
        let t_id = -1;
        let t_origin_id = 10;
        let t_set_id = 1000;
        let t_fname = "Test exercise";
        let t_exercise_type = 1;
        let t_description = "Quad burner";
        let t_notes = "Put your right foot in. Put your left foot out.";
        let t_create_time = SystemTime::now();
        let t_complete_time = t_create_time + Duration::new(600, 0);
        let t_create_id = 10;
        let t_completed_id = 200;


        let t_exercise = Exercise {
            id: t_id,
            origin_id: t_origin_id,
            set_id: t_set_id,
            fname: t_fname.to_string(),
            exercise_type: t_exercise_type,
            description: t_description.to_string(),
            notes: t_notes.to_string(),
            create_time: t_create_time,
            complete_time: t_complete_time,
            create_id: t_create_id,
            completed_id: t_completed_id,
        };


        assert_eq!(t_id, t_exercise.id);
        assert_eq!(t_origin_id, t_exercise.origin_id);
        assert_eq!(t_set_id, t_exercise.set_id);
        assert_eq!(t_exercise_type, t_exercise.exercise_type);
        assert_eq!(t_description, t_exercise.description);
        assert_eq!(t_notes, t_exercise.notes);
        assert_eq!(t_create_time, t_exercise.create_time);
        assert_eq!(t_complete_time, t_exercise.complete_time);
        assert_eq!(t_create_time, t_exercise.create_time);
        assert_eq!(t_completed_id, t_exercise.completed_id);
    }
}
