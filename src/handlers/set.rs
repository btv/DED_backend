//! Handler for all the of the set database interactions

use diesel::prelude::*;
use actix_web::{web, Responder};
use std::time::SystemTime;

use crate::models::Set;

impl Set {
    //! Function to create a new Set instance.
    fn new(
        exercise_id: i32,
        style: String,
        unit: String,
        goal_reps: i16,
        goal_value: String,
        description: String,
        completed_reps: i16,
        completed_value: String,
    ) -> Set {
        Set {
            id: rand::random::<i32>().abs(),
            exercise_id: exercise_id,
            unit: unit,
            style: style,
            goal_reps: goal_reps,
            goal_value: goal_value,
            description: description,
            created_or_completed: SystemTime::now(),
            completed_reps: completed_reps,
            completed_value: completed_value
        }
    }
}

//! function run by the new set endpoint to generate a new set within the database.
pub async fn new_set(
        exercise_id: i32,
        style: String,
        unit: String,
        goal_reps: i16,
        goal_value: String,
        description: String,
        completed_reps: i16,
        completed_value: String,
    ) -> impl Responder {

        let new_set = Set::new(
            exercise_id,
            style,
            unit,
            goal_reps,
            goal_value,
            description,
            completed_reps,
            completed_value
        );

        //diesel::insert_into(sete).values(&new_new).execute(conn)?;

        web::Json(new_set.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::sign;

    #[test]
    fn new_set_unit_test() {
        let new_test_set = Set::new(
            -1,
            "none".to_string(),
            "none".to_string(),
            10,
            "none".to_string(),
            "none".to_string(),
            0,
            "none".to_string()
        );

        // String tests
        assert_eq!(new_test_set.style, "none".to_string());
        assert_eq!(new_test_set.unit, "none".to_string());
        assert_eq!(new_test_set.description, "none".to_string());
        assert_eq!(new_test_set.goal_value, "none".to_string());
        assert_eq!(new_test_set.completed_value, "none".to_string());

        //Numerical tests
        assert_eq!(new_test_set.completed_reps, 0);
        assert_eq!(new_test_set.goal_reps, 10);
        assert_eq!(true, sign::Signed::is_positive(&new_test_set.id));
    }
}
