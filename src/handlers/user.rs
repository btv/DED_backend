

use actix_web::{web, Responder};

use crate::models::users::{NewUser};

pub async fn create_user (
        in_user: web::Path<NewUser>
    ) -> impl Responder {

    format!("{:?}", &in_user)
}
