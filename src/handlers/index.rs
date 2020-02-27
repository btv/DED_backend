//! Handler for all the of the index based urls.
//!
//! This library is mostly provided as an example of both layout of
//! how the future modules should be done and how url's within thos modules
//! should look.
//!
use actix_web::{get, web, Responder};

#[get("/{name}/index.html")]
pub async fn index_with_name(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello World!".to_string()
}
