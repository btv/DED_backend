//! Handler for all the of the workout database interactions
//! For more infomation on these endpoints, please go [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-workouts-landingpage)


use actix_web::{web, Responder, HttpResponse};
use crate::establish_connection;
use crate::models::workouts::{NewWorkout, Workout, CompleteWorkout, WorkoutList};

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

/// Get a json object of a Workout from its ID.
///
/// More information [here]()
pub async fn find_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Workout::get_workout_by_id(*id, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
///
/// Delete a workout from the database based on the primary key.
///
/// More information [here]()
pub async fn delete(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Workout::delete(*id, &conn)
        .map(|_| HttpResponse::Ok())
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn update_by_id(id: web::Path<i32>, new_set: web::Json<NewWorkout>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Workout::update(*id,&new_set, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn complete(id: web::Path<i32>, new_set: Option<web::Json<CompleteWorkout>>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    let local_set = match new_set {
        Some(i) => i,
        None => web::Json(CompleteWorkout{ notes: None})
    };

    Workout::complete(*id,&local_set, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn find_by_origin_id(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    WorkoutList::get_workouts_by_origin_id(*id, &conn)
        .map(|setlist| HttpResponse::Ok().json(setlist))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
