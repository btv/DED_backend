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
    pub workout_id: i32,
    pub name: String,
    pub exercise_type: i32,
    pub description: String,
    pub notes: String,
    pub create_time: SystemTime,
    pub complete_time: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "exercises"]
pub struct NewExercise {
    pub origin_id: i32,
    pub workout_id: i32,
    pub name: String,
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

    pub fn delete(in_id: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(exercises::table.find(in_id)).execute(connection)
    }

    pub fn update(
        in_id: i32,
        new_ex: &NewExercise,
        connection: &PgConnection
    ) -> Result<usize, diesel::result::Error> {

        diesel::update(exercises::table.find(in_id))
            .set(new_ex)
            .execute(connection)
    }
}

impl PartialEq<NewExercise> for Exercise {
    fn eq(&self, other:& NewExercise) -> bool {
        self.origin_id == other.origin_id &&
        self.workout_id == other.workout_id &&
        self.name == other.name &&
        self.exercise_type == other.exercise_type &&
        self.description == other.description &&
        self.notes == other.notes
    }
}

impl PartialEq<Exercise> for NewExercise {
    fn eq(&self, other:& Exercise) -> bool {
        self.origin_id == other.origin_id &&
        self.workout_id == other.workout_id &&
        self.name == other.name &&
        self.exercise_type == other.exercise_type &&
        self.description == other.description &&
        self.notes == other.notes
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
        let t_workout_id = 1000;
        let t_name = "Test exercise";
        let t_exercise_type = 1;
        let t_description = "Quad burner";
        let t_notes = "Put your right foot in. Put your left foot out.";
        let t_create_time = SystemTime::now();
        let t_complete_time = t_create_time + Duration::new(600, 0);



        let t_exercise = Exercise {
            id: t_id,
            origin_id: t_origin_id,
            workout_id: t_workout_id,
            name: t_name.to_string(),
            exercise_type: t_exercise_type,
            description: t_description.to_string(),
            notes: t_notes.to_string(),
            create_time: t_create_time,
            complete_time: t_complete_time,
        };


        assert_eq!(t_id, t_exercise.id);
        assert_eq!(t_origin_id, t_exercise.origin_id);
        assert_eq!(t_workout_id, t_exercise.workout_id);
        assert_eq!(t_exercise_type, t_exercise.exercise_type);
        assert_eq!(t_description, t_exercise.description);
        assert_eq!(t_notes, t_exercise.notes);
        assert_eq!(t_create_time, t_exercise.create_time);
        assert_eq!(t_complete_time, t_exercise.complete_time);
    }
}
