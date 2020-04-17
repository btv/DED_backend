//! Handler for all the of the exercise database interactions
//! For more infomation on these endpoints, please go [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-exercises-landingpage)

use actix_web::{web, Responder, HttpResponse};
use crate::establish_connection;

use crate::models::exercises::{NewExercise, Exercise};

/// Create a new exercise entry in the database.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-exercises-create)
pub async fn create_exercise (
        in_exercise: web::Json<NewExercise>
    ) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    in_exercise
        .create(&conn)
        .map(|ex| HttpResponse::Ok().json(ex))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}


/// Delete an exercise from the database based on the primary key.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-exercises-delete)
pub async fn delete(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Exercise::delete(*id, &conn)
        .map(|_| HttpResponse::Ok())
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

/// Get an exercise json object that include all workouts sets based on primary key.
///
/// More information [here]()
pub async fn find_by_id(ex_id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Exercise::get_exercise_by_id(*ex_id, &conn)
        .map(|setlist| HttpResponse::Ok().json(setlist))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn update_by_id(id: web::Path<i32>, new_set: web::Json<NewExercise>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Exercise::update(*id,&new_set, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
