#![allow(non_snake_case)]

extern crate actix_web;

use actix_web::{App, HttpServer};

use DED_backend::appconfig::config_app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config_app)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
