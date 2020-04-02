//! Handler for all the of the index based urls.
//!
//! For more infomation on these endpoints, please go [here](https://github.com/coloradocollective/DED_Backend/wiki/index-landingpage)

use actix_web::{web, Responder, HttpResponse};
use crate::{establish_connection, hash_secret_key};
use crate::models::users::{NewUser, User, SlimUser, AuthData};
use actix_identity::Identity;
use argonautica::{Hasher, Verifier};



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
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

/// Endpoint for login a user to the system.
///
/// More information [here]()
pub async fn login (
    auth_data: web::Json<AuthData>,
    id: Identity
) -> impl Responder {
    use diesel::prelude::*;
    use crate::schema::users::dsl::{users,username};
    let conn = establish_connection().get().unwrap();

    let user_to_auth = users
        .filter(username.eq(&auth_data.username))
        .first::<User>(&conn)
        .unwrap();

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(&user_to_auth.passwd)
        .with_password(&auth_data.password)
        .with_secret_key(hash_secret_key().as_str())
        .verify()
        .unwrap();

    if is_valid {
        let slim_user = SlimUser::from(user_to_auth);
        id.remember(
            serde_json::to_string(&slim_user).unwrap()
        );
        HttpResponse::Ok().json(slim_user)
    } else {
        HttpResponse::Unauthorized().json("Unauthorized")
    }
}

/// Endpoint for logout a user to the system.
///
/// More information [here]()
pub async fn logout (id: Identity) -> impl Responder {
    if let Some(_i) = id.identity() { id.forget() }
    HttpResponse::Ok().finish()
}
