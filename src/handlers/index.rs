use actix_web::{get, web, Responder};

#[get("/{name}/index.html")]
pub async fn index_with_name(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello World!".to_string()
}
