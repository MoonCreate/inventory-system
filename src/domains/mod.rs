use actix_web::{middleware::Logger, web};

mod home;
mod product;
mod product_variant;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Logger::default())
            .configure(user::config)
            .configure(home::config)
            .configure(product::config)
            .configure(product_variant::config),
    );
}
