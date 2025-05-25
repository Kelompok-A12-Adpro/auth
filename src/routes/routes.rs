use actix_web::web;
use crate::{
    controller::{
        profile_controller::{get_profile_controller, upsert_profile_controller, get_own_profile_controller},
        user_controller::{login_user_controller, register_user_controller},
    },
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg

        // ---------- Auth ----------
        .service(
            web::resource("/auth/register").route(web::post().to(register_user_controller)),
        )
        .service(web::resource("/auth/login").route(web::post().to(login_user_controller)))
        
        // ---------- Profile ----------
        .service(
        web::resource("/profile")
            .route(web::get().to(get_own_profile_controller)) 
            .route(web::post().to(upsert_profile_controller))
            .route(web::put().to(upsert_profile_controller)),
        )
        .service(
        web::resource("/profile/{id}")
            .route(web::get().to(get_profile_controller)),
        );
}
