#![allow(non_snake_case)]

extern crate actix_web;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}/index.html")]
async fn index(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/").to(|| async {"Hello World!"}))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
