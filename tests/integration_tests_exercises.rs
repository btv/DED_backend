#![allow(non_snake_case)]

extern crate diesel;
extern crate dotenv;


mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::exercises::{Exercise, NewExercise};
    use diesel::RunQueryDsl;

    #[test]
    fn test_db_exercise_insert_and_find(){
        let conn = establish_connection().get().unwrap();

        // delete all entries in the database
        let _xxx = diesel::delete(DED_backend::schema::exercises::dsl::exercises)
            .execute(&conn);

        let t_origin_id= 200;
        let t_set_id= 300;
        let t_fname= "SuperJock";
        let t_exercise_type= 22;
        let t_description= "Pecs! Pecs! and more Pecs!";
        let t_notes= "never enough reps";

        let new_ex = NewExercise{
            origin_id: t_origin_id,
            set_id: t_set_id,
            fname: t_fname.to_string(),
            exercise_type: t_exercise_type,
            description: t_description.to_string(),
            notes: t_notes.to_string(),
        };


        let new_ex_id = match new_ex.create(&conn) {
            Ok(r_ex) =>{
                assert_eq!(r_ex.origin_id, t_origin_id);
                assert_eq!(r_ex.set_id, t_set_id);
                assert_eq!(r_ex.fname, t_fname);
                assert_eq!(r_ex.exercise_type, t_exercise_type);
                assert_eq!(r_ex.description, t_description);
                assert_eq!(r_ex.notes, t_notes);
                r_ex.id
            }
            Err(E) =>{// todo:   Need to fix this
                assert_eq!(1,5);
                print!("got error {}",E);
                // assert_eq!(E, diesel::ConnectionError::CouldntSetupConfiguration);
                0
            }
        };

        match Exercise::get_exercise_by_id(new_ex_id, &conn) {
            Err(E) =>{
                assert_eq!(E,diesel::NotFound );
            }
            Ok(ret)=> {
                assert_eq!(ret, new_ex);
            }
        }

    }
    #[test]
    fn test_db_exercises_not_found(){
        let conn = establish_connection().get().unwrap();
        match Exercise::get_exercise_by_id(-123923, &conn) {
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
