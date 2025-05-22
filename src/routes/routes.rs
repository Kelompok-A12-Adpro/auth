use actix_web::web;
use crate::controller::user_controller::{register_user_controller, login_user_controller};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("auth/register", web::post().to(register_user_controller))
        .route("auth/login", web::post().to(login_user_controller));
}