use actix_web::web;

mod controllers;
mod dtos;
mod services;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .service(controllers::retrieve_product_by_id_controller)
            .service(controllers::retrieve_all_products_controller)
            .service(controllers::delete_product_by_id_controller)
            .service(controllers::update_product_by_id_controller)
            .service(controllers::create_product_controller),
    );
}
