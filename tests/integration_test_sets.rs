#![allow(non_snake_case)]



extern crate diesel;
extern crate dotenv;
mod tests {
    use DED_backend::{establish_connection};
    use DED_backend::models::sets::{Set, NewSet};
    use diesel::RunQueryDsl;
    use std::time::SystemTime;

    #[test]
    fn test_db_set_insert_and_find() {
        let conn = establish_connection();
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
            style: t_style.to_string(),
            unit: t_unit.to_string(),
            goal_reps: t_goal_reps,
            goal_value: t_goal_value.to_string(),
            description: t_description.to_string(),
            completed_reps: 0,
            completed_value: "".to_string()
        };

        let result = ns.create();

        match result {
            Ok(u) => {
                assert_eq!(u.unit, t_unit);
            }
            Err(E) => {
                assert_eq!(1, 5);
                print!("got error {}", E);
                // assert_eq!(E, diesel::ConnectionError::CouldntSetupConfiguration);     Need a proper error for this
            }
        }
    }
}