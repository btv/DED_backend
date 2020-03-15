#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;
extern crate dotenv;


#[cfg(test)]
extern crate mockall;

use diesel::prelude::*;
use diesel::pg::{PgConnection};
use dotenv::dotenv;
use std::env;


pub mod appconfig;
pub mod handlers;
pub mod models;
pub mod schema;


//pub fn establish_connection(db_env_var: &str) -> PgConnection {
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    //let db_url = env::var(db_env_var)
    //             .expect(&format!("Environment variable {} must be set", db_env_var));
    let db_url = env::var("DATABASE_URL")
                      .expect("DATABASE_URL must be set");

    PgConnection::establish(&db_url)
                  .expect(&format!("Error connecting to {}", db_url))
}
