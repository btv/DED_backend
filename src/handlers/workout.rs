
use diesel::prelude::*;
use actix_web::{web, Responder};

use crate::models::workouts::{Workout,NewWorkout};

pub async fn create_workout (
        in_workous: web::Json<NewWorkout>
    ) -> impl Responder {

    format!("{:?}", &in_workous)

}
