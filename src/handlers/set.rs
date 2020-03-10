//! Handler for all the of the set database interactions

use diesel::prelude::*;
use rand::prelude::*;
use actix_web::{web, Responder};
use std::time::SystemTime;

use crate::models::Set;

impl Set {
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
            id: rand::random(),
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



