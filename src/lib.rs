#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate argonautica;
extern crate actix_web;
extern crate actix_identity;
extern crate actix_cors;



#[cfg(test)]
extern crate mockall;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };


pub mod appconfig;
pub mod handlers;
pub mod models;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
                      .expect("DATABASE_URL must be set");

    init_pool(&db_url).expect("Failed to create pool")
}

pub fn hash_secret_key() -> String {
    std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
}
