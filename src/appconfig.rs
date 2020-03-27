use actix_web::web;

use crate::handlers::index::{index, index_with_name};
use crate::handlers::set;


/// This function is modified as needed to make sure that all of the
/// applicaitons routeing happens.
pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/sets")
                .service(
                    web::resource("/new/")
                        .route(web::post().to(set::new)
                    )
                )
                .service(
                    web::resource("/delete/{id}/")
                        .route(web::post().to(set::delete))
                        .route(web::delete().to(set::delete))
                )
                .service(
                    web::resource("/find_by_exercise_id/{ex_id}/")
                        .route(web::get().to(set::find_by_exercise_id))
                )
        )
        .route("/", web::get().to(index))
        .route("/{name}/index.html", web::get().to(index_with_name));
}
