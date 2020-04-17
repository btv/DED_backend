use actix_web::web;

use crate::handlers::{set, exercise, index, workout};


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
                    web::resource("/complete/{id}")
                        .route(web::post().to(set::complete_by_set_id)
                    )
                )
                .service(
                    web::resource("/{id}/")
                        .route(web::delete().to(set::delete))
                        .route(web::get().to(set::find_by_set_id))
                        .route(web::patch().to(set::update_by_set_id))
                )
                .service(
                    web::resource("/find_by_exercise_id/{ex_id}/")
                        .route(web::get().to(set::find_by_exercise_id))
                )
        )
        .service(
            web::scope("/exercises")
                .service(
                    web::resource("/create/")
                        .route(web::post().to(exercise::create_exercise)
                    )
                )
                .service(
                    web::resource("/{id}/")
                        .route(web::delete().to(exercise::delete))
                        .route(web::get().to(exercise::find_by_id))
                )
        )
        .service(
            web::scope("/workouts")
                .service(
                    web::resource("/create/")
                        .route(web::post().to(workout::create_workout))
                )
        )
        .route("/", web::get().to(index::index))
        .route("/login/", web::post().to(index::login))
        .route("/logout/", web::delete().to(index::logout))
        .route("/register/", web::post().to(index::register));
}
