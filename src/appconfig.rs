use actix_web::web;

use crate::handlers::index::{index, index_with_name};


/// This function is modified as needed to make sure that all of the
/// applicaitons routeing happens.
pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .route("/{name}/index.html", web::get().to(index_with_name));
}
