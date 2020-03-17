#![allow(non_snake_case)]

extern crate diesel;
extern crate dotenv;

mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::workouts::{Workout, NewWorkout};

    use diesel::RunQueryDsl;
    use std::time::SystemTime;
    use std::time::Duration;
    use term::terminfo::parm::Param::Words;

    // to run all tests sequentially    cargo test -- --test-threads=1
    #[test]
    fn test_db_workout_insert_and_find() {
        let conn = establish_connection();

        let _xxx = diesel::delete(DED_backend::schema::workouts::dsl::workouts)
            .execute(&conn);


        let t_id = 1000;
        let t_origin_id = 9999;
        let t_exercise = 32;
        let t_fname = "Foot Tap";
        let t_description = "put your right foot left";
        let t_notes = "put your left foot out";
        let t_created_time = SystemTime::now();
        let t_completed_time = t_created_time + Duration::new(7500, 0);
        let t_create_id = 777;
        let t_completed_id = 666;

        let t_workout = NewWorkout {
            id: t_id,
            origin_id: t_origin_id,
            exercise: t_exercise,
            fname: t_fname.to_string(),
            description: t_description.to_string(),
            notes: t_notes.to_string(),
            created_time: t_created_time,
            completed_time: t_completed_time,
            create_id: t_create_id,
            completed_id: t_completed_id,
        };


        let res = t_workout.create();
        match res {
            Ok(r) => {}
            Err(E) => {}

        }
    }
}


