//! Handler for all the of the set database interactions
//! For more infomation on these endpoints, please go [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-sets-landingpage)


use actix_web::{web, Responder, HttpResponse};
use crate::establish_connection;

use crate::models::sets::{NewSet,Set, SetList, CompleteSet};

/// Create a new set entry in the database.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-sets-create)
pub async fn new(new_set: web::Json<NewSet>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    new_set
        .create(&conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

/// Delete a set from the database based on the primary key.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-sets-delete)
pub async fn delete(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Set::delete(*id, &conn)
        .map(|_| HttpResponse::Ok())
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}


/// Get an array of json objects that include all workouts sets based on an exercise ID.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-sets-find_by_exercise_id)
pub async fn find_by_exercise_id(ex_id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    SetList::get_sets_by_exercise_id(*ex_id, &conn)
        .map(|setlist| HttpResponse::Ok().json(setlist))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

/// Get a json object of a Set from its ID.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-sets-find)
pub async fn find_by_set_id(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Set::get_set_by_id(*id, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn update_by_set_id(id: web::Path<i32>, new_set: web::Json<NewSet>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Set::update(*id,&new_set, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn complete_by_set_id(id: web::Path<i32>, new_set: web::Json<CompleteSet>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Set::complete(*id,&new_set, &conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

/// Get an array of json objects that include all workouts sets based on an origin ID.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-sets-find_by_origin_id)
pub async fn find_by_origin_id(ex_id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    SetList::get_sets_by_origin_id(*ex_id, &conn)
        .map(|setlist| HttpResponse::Ok().json(setlist))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
