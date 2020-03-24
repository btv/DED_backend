//! Handler for all the of the set database interactions


use actix_web::{web, Responder, HttpResponse};
use crate::establish_connection;

use crate::models::sets::{NewSet,Set};

pub async fn new(new_set: web::Json<NewSet>) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    new_set
        .create(&conn)
        .map(|set| HttpResponse::Ok().json(set))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn delete(id: web::Path<i32>, ) -> impl Responder {
    let conn = establish_connection().get().unwrap();

    Set::delete(&id, &conn)
        .map(|_| HttpResponse::Ok())
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
