//! Handler for all the of the index based urls.
//!
//! This library is mostly provided as an example of both layout of
//! how the future modules should be done and how url's within those modules
//! should look.

use actix_web::{web, Responder};

/// Expiremental page, pulling the name parameter from the URL and using
/// it for the output string.
///
/// **Arguments**:
/// * name: <pretty self descriptive
///
/// **Returns**:
/// A implementation of a `Responder` with the text `Hello [name]` to the person called on.
pub async fn index_with_name(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

/// Returns the information needed for the index page.
pub async fn index() -> impl Responder {
    "DED Backend.  Hello!".to_string()
}
