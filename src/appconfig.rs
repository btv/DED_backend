use actix_web::web;

use crate::handlers::index::{index, index_with_name};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(index_with_name);
}
