use actix_web::{web, HttpResponse};
use crate::services::auth_service::{register_user, login_user};
use crate::models::user::{User, LoginRequest};

pub async fn register_user_controller(user: web::Json<User>) -> HttpResponse {
    match register_user(user.into_inner()).await {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn login_user_controller(login_req: web::Json<LoginRequest>) -> HttpResponse {
    match login_user(login_req.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
