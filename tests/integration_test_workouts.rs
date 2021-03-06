#![allow(non_snake_case)]

extern crate diesel;
extern crate dotenv;

mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::workouts::{Workout, NewWorkout};

    use diesel::RunQueryDsl;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::time::Duration;

    // to run all tests sequentially    cargo test -- --test-threads=1
    #[test]
    fn test_db_workout_insert_and_find() {
        let conn = establish_connection().get().unwrap();

        let t_id = 1000;
        let t_origin_id = 9999;
        let t_name = "Foot Tap";
        let t_description = "put your right foot left";
        let t_notes = "put your left foot out";
        let t_created_time = SystemTime::now();
        let t_completed_time = t_created_time + Duration::new(7500, 0);
        let t_user_id = 102;

        let t_workout = NewWorkout {
            origin_id: t_origin_id,
            name: t_name.to_string(),
            description: t_description.to_string(),
            notes: t_notes.to_string(),
            user_id: t_user_id
        };


        match t_workout.create(&conn) {
            Ok(r) => {
                assert_eq!(r.origin_id, t_origin_id);
                assert_eq!(r.description, t_description);
                assert_eq!(r.notes, t_notes);
                assert_eq!(r.name, t_name);
                assert_eq!(r.user_id, t_user_id);

            }
            Err(E) => {
                assert_eq!(E, diesel::NotFound);
            }
        }

        match Workout::get_workout_by_id(t_id,&conn) {
            Ok(r) => {
                assert_eq!(r.id, t_id);
                assert_eq!(r.origin_id, t_origin_id);
                assert_eq!(r.description, t_description);
                assert_eq!(r.notes, t_notes);
                let mut sec_original = t_created_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                let mut sec_saves = r.created_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);

                sec_original = t_completed_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                sec_saves = r.completed_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);


            }
            Err(E) => {
                assert_eq!(E, diesel::NotFound);
            }
        }




        match Workout::get_workout_by_user_id(t_user_id,&conn) {
            Ok(r) => {
                assert_eq!(r.user_id, t_user_id);
                assert_eq!(r.origin_id, t_origin_id);
                assert_eq!(r.description, t_description);
                assert_eq!(r.notes, t_notes);
                let mut sec_original = t_created_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                let mut sec_saves = r.created_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);

                sec_original = t_completed_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                sec_saves = r.completed_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);


            }
            Err(E) => {
                assert_eq!(E, diesel::NotFound);
            }
        }
    }

    #[test]
    fn test_db_workout_not_found(){
        let conn = establish_connection().get().unwrap();
        let res = Workout::get_workout_by_id(-99999, &conn);
        match res{//todo: need to fix this
            Err(E) =>{
                assert_eq!(E, diesel::NotFound);
            }
            Ok(_T) =>{
                print!("Negative user Id in user table");
                assert_eq!(6,-1);
            }
        }
    }

}


