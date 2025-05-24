use actix_web::web;
use crate::controller::user_controller::{register_user_controller, login_user_controller};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/auth/register")
                .route(web::post().to(register_user_controller))
        )
        .service(
            web::resource("/auth/login")
                .route(web::post().to(login_user_controller))
        );
}
