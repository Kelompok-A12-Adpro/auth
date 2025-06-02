use actix_web::{web, HttpResponse};
use crate::model::user::{DeleteUserRequest, LoginRequest, RegisterRequest};
use crate::service::user_service::{
    delete_user_by_id, get_all_users_data, get_user_data, get_users_data, login_user, register_user
};

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

pub async fn get_recent_users() -> HttpResponse {
    match get_users_data().await {
        Ok(user_data) => HttpResponse::Ok().json(user_data),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn get_all_users() -> HttpResponse {
    match get_all_users_data().await {
        Ok(user_data) => HttpResponse::Ok().json(user_data),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn get_user_by_id(
    user_id: web::Path<i32>,
) -> HttpResponse {
    match get_user_data(user_id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn delete_user(
    data: web::Json<DeleteUserRequest>,
) -> HttpResponse {
    match delete_user_by_id(data.user_id).await {
        Ok(msg) => HttpResponse::Ok().json(msg),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}