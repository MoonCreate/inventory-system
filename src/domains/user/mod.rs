use actix_web::web;

mod controllers;
mod dtos;
mod services;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(controllers::user_get)
            .service(controllers::user_get_by_id)
            .service(controllers::user_create)
            .service(controllers::user_delete)
            .service(controllers::update_user),
    );
}
