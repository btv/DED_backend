#![allow(non_snake_case)]

extern crate actix_web;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}/index.html")]
pub async fn index_with_name(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello World!".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index_with_name)
            .service(index)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
