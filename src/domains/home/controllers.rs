use actix_web::{get, Responder};

use super::services;

#[get("/")]
pub async fn home() -> impl Responder {
    services::get_hello_world()
}
