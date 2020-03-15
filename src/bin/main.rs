#![allow(non_snake_case)]

extern crate actix_web;

use actix_web::{App, HttpServer};

use DED_backend::appconfig::config_app;
use DED_backend::establish_connection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .configure(config_app)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
