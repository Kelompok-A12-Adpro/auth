use actix_web::{web, HttpResponse};
use crate::model::user::{RegisterRequest, LoginRequest};
use crate::service::user_service::{register_user, login_user};

pub async fn register_user_controller(
    user: web::Json<RegisterRequest>,
) -> HttpResponse {
    match register_user(user.into_inner()).await {
        Ok(msg) => HttpResponse::Ok().json(msg),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub async fn login_user_controller(
    login_req: web::Json<LoginRequest>,
) -> HttpResponse {
    match login_user(login_req.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
