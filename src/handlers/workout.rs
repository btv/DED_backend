//! Handler for all the of the workout database interactions
//! For more infomation on these endpoints, please go [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-workouts-landingpage)


use actix_web::{web, Responder, HttpResponse};
use crate::establish_connection;
use crate::models::workouts::{NewWorkout};

/// Create a new workout entry in the database.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-workouts-create)
pub async fn create_workout(new_wk: web::Json<NewWorkout>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    new_wk
        .create(&conn)
        .map(|wk| HttpResponse::Ok().json(wk))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
