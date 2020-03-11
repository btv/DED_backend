#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;

#[cfg(test)]
extern crate num_traits;

pub mod appconfig;
pub mod handlers;
pub mod models;
pub mod schema;
