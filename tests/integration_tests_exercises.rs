#![allow(non_snake_case)]

extern crate diesel;
extern crate dotenv;


mod tests {
    use DED_backend::{establish_connection};
    use DED_backend::models::exercises::{Exercise, NewExercise};
    use diesel::RunQueryDsl;
    
    use std::time::{SystemTime,UNIX_EPOCH};
    use std::time::Duration;
    
    #[test]
    fn test_db_exercise_insert_and_find(){
        let conn = establish_connection();

        // delete all entries in the database
        let _xxx = diesel::delete(DED_backend::schema::exercises::dsl::exercises)
            .execute(&conn);

        let t_id= 100;
        let t_origin_id= 200;
        let t_set_id= 300;
        let t_fname= "SuperJock";
        let t_exercise_type= 22;
        let t_description= "Pecs! Pecs! and more Pecs!";
        let t_notes= "never enouhg reps";
        let t_create_time= SystemTime::now();
        let t_complete_time= t_create_time + Duration::new(1200,0);
        let t_create_id=100;
        let t_completed_id= 240;
        
        let t_exercise = Exercise{
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
            completed_id: t_completed_id
        };


        let new_ex = NewExercise{
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
            completed_id: t_completed_id
        };


        let result = new_ex.create();

        match result{
            Ok(r_ex) =>{
                assert_eq!(r_ex.id, t_exercise.id);
                assert_eq!(r_ex.origin_id, t_exercise.origin_id);
                assert_eq!(r_ex.set_id, t_exercise.set_id);
                assert_eq!(r_ex.fname, t_exercise.fname);
                assert_eq!(r_ex.exercise_type, t_exercise.exercise_type);
                assert_eq!(r_ex.description, t_exercise.description);
                assert_eq!(r_ex.notes, t_exercise.notes);


                let mut sec_original = t_create_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                let mut sec_saves = r_ex.create_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);


                sec_original = t_complete_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                sec_saves = r_ex.complete_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);

                assert_eq!(r_ex.create_id,t_exercise.create_id);
                assert_eq!(r_ex.completed_id, t_exercise.completed_id);
            }
            Err(E) =>{// todo:   Need to fix this
                assert_eq!(1,5);
                print!("got error {}",E);
                // assert_eq!(E, diesel::ConnectionError::CouldntSetupConfiguration);
            }
        }

        let result = Exercise::get_exercise_by_id(t_id);
        match result {
            Err(E) =>{
                assert_eq!(E,diesel::NotFound );
            }
            Ok(ret)=> {
                assert_eq!(ret.id,t_exercise.id);
                assert_eq!(ret.origin_id,     t_exercise.origin_id);
                assert_eq!(ret.set_id, t_set_id);
                assert_eq!(ret.fname, t_fname);
                assert_eq!(ret.exercise_type,t_exercise_type);
                assert_eq!(ret.description, t_description);
                assert_eq!(ret.notes, t_notes);

                let mut sec_original = t_create_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                let mut sec_saves = ret.create_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);


                sec_original = t_complete_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                sec_saves = ret.complete_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);


                assert_eq!(ret.create_id,t_create_id);
                assert_eq!(ret.completed_id,t_completed_id);
            }
        }


    }
    #[test]
    fn test_db_exercises_not_found(){
        let result = Exercise::get_exercise_by_id(-123923);
        match result{ //todo: need to fix this.
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