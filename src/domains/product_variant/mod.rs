use actix_web::web::{self, ServiceConfig};

mod controllers;
mod dtos;
mod services;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/product_variant")
            .service(controllers::create_product_variant)
            .service(controllers::update_product_variant),
    );
}
