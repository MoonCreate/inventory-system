use actix_web::web;

mod controllers;
mod services;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").service(controllers::home));
}
