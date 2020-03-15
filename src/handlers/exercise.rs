use diesel::prelude::*;
use actix_web::{web, Responder};

use crate::models::exercises::{Exercise,NewExercise};

pub async fn create_exercise (
        in_exercise: web::Path<NewExercise>
    ) -> impl Responder {

    format!("{:?}", &in_exercise)
}

