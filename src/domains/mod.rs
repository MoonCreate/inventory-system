use actix_web::web;

mod home;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(user::config).configure(home::config);
}
