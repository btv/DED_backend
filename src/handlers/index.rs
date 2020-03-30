//! Handler for all the of the index based urls.
//!
//! For more infomation on these endpoints, please go [here](https://github.com/coloradocollective/DED_Backend/wiki/index-landingpage)

use actix_web::{web, Responder, HttpResponse};
use crate::{establish_connection,hash_secret_key};
use crate::models::users::{NewUser,SlimUser,AuthData};



/// Returns the information needed for the index page.
pub async fn index() -> impl Responder {
    "DED Backend.  Hello!".to_string()
}


/// Endpoint for registering a new user to the system.
///
/// More information [here](https://github.com/coloradocollective/DED_Backend/wiki/endpoint-users-create)
pub async fn register (
        in_user: web::Json<NewUser>
    ) -> impl Responder {
    use argonautica::Hasher;
    let conn = establish_connection().get().unwrap();

    let mut new_in_user = in_user.clone();

    let mut hasher = Hasher::default();
    new_in_user.passwd = hasher
        .with_password(&in_user.passwd)
        .with_secret_key(hash_secret_key().as_str())
        .hash()
        .unwrap();


    new_in_user
        .create(&conn)
        .map(|user| HttpResponse::Ok().json(SlimUser::from(user)))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

/// Endpoint for authenticating a user to the system.
///
/// More information [here]()
pub async fn authenticate (in_data: web::Json<AuthData>) -> impl Responder {
    format!("{:?}", in_data)
}
