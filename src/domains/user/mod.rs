use actix_web::web;

mod controllers;
mod dtos;
mod services;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(controllers::user_get_controller)
            .service(controllers::user_get_by_id_controller)
            .service(controllers::user_create_controller)
            .service(controllers::user_delete_controller)
            .service(controllers::update_user_controller),
    );
}
