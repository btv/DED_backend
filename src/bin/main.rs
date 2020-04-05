#![allow(non_snake_case)]

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};

use DED_backend::appconfig::config_app;
use DED_backend::establish_connection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .configure(config_app)
        .wrap(
            IdentityService::new(
                CookieIdentityPolicy::new(&[0;32])
                    .name("ded_auth")
                    .secure(false)
                )
        )
        .wrap(
            Cors::new().supports_credentials().finish()
        )})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
