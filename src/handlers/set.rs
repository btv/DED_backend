//! Handler for all the of the set database interactions


use actix_web::{web, Responder};

use crate::models::sets::{NewSet};

pub async fn new(new_set: web::Json<NewSet>) -> impl Responder {

    format!("{:?}", &new_set)
}
