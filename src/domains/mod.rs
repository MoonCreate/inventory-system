use actix_web::web;

mod home;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(home::config);
}
