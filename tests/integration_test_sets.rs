#![allow(non_snake_case)]



extern crate diesel;
extern crate dotenv;


mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::sets::{Set, NewSet};
    use diesel::RunQueryDsl;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_db_set_insert_and_find() {
        let conn = establish_connection().get().unwrap();
        let _xxx = diesel::delete(DED_backend::schema::sets::dsl::sets)
            .execute(&conn);


        let t_id = 101;
        let t_exercise_id = 1020;
        let t_style = "Fancy";
        let t_unit = "steps";
        let t_goal_reps = 45;
        let t_goal_value = "To survive";
        let t_description = "Twist and shout";
        let t_created_or_completed = SystemTime::now();
        let t_completed_reps = 10;
        let t_completed_value = "Should this be a string";

        let ns = NewSet {
            exercise_id: t_exercise_id,
            style: t_style.to_string(),
            unit: t_unit.to_string(),
            goal_reps: t_goal_reps,
            goal_value: t_goal_value.to_string(),
            description: t_description.to_string(),
        };

        match ns.create(&conn) {
            Ok(u) => {
                assert_eq!(u.unit, t_unit);
            }
            Err(E) => {
                print!("got error {}", E);
                assert_eq!(1, 5);
                // assert_eq!(E, diesel::ConnectionError::CouldntSetupConfiguration);     Need a proper error for this
            }
        }

        let result = Set::get_set_by_id(t_id, &conn);

        match result{
            Ok(r_set) =>{
                assert_eq!(r_set.id, t_id);
                // Possibly address the nanosecond difference
                let sec_original = t_created_or_completed.duration_since(UNIX_EPOCH).unwrap().as_secs();
                let sec_saves = r_set.created_or_completed.duration_since(UNIX_EPOCH).unwrap().as_secs();
                assert_eq!(sec_original, sec_saves);
                assert_eq!(r_set.description, t_description);
                assert_eq!(r_set.exercise_id, t_exercise_id);
                assert_eq!(r_set.completed_value,t_completed_value);
                assert_eq!(r_set.goal_reps,t_goal_reps);
                assert_eq!(r_set.unit, t_unit);
                assert_eq!(r_set.completed_reps, t_completed_reps);
                assert_eq!(r_set.style, t_style);
                assert_eq!(r_set.goal_value, t_goal_value);
            }
            Err(E) =>{
                assert_eq!(E,diesel::NotFound);
            }
        }


    }

    #[test]
    fn test_db_set_not_found(){
        let conn = establish_connection().get().unwrap();
        match Set::get_set_by_id(-999, &conn) {
            Ok(_T) =>{
                print!("Negative value in sets ");
                assert_eq!(6,-1);
            }
            Err(E) =>{
                assert_eq!(E,diesel::NotFound);
            }
        }
    }
}
