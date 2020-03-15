//! Handler for all the of the set database interactions

use diesel::prelude::*;
use actix_web::{web, Responder};

use crate::models::sets::{Set,NewSet};

pub fn create(new_set: web::Json<NewSet>) -> impl Responder {

    format!("{:?}", &new_set)
}

#[cfg(test)]
mod tests {
}
