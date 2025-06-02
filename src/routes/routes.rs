use actix_web::web;
use crate::controller::{
        profile_controller::{get_own_profile_controller, get_profile_controller, upsert_profile_controller},
        user_controller::{delete_user, get_all_users, get_recent_users, login_user_controller, register_user_controller},
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
        )

        // ---------- User Data ----------
        .service(
            web::resource("/users/recent")
                .route(web::get().to(get_recent_users)),
        )
        .service(
            web::resource("/users")
                .route(web::get().to(get_all_users))
                .route(web::delete().to(delete_user)),
        )
        .service(
            web::resource("/users/{id}")
                .route(web::get().to(get_profile_controller)),
        );
}
