use actix_web::{web, HttpRequest, HttpResponse};
use crate::{
    model::profile::ProfileBuilder,
    service::profile_service::{get_profile, upsert_profile},
    service::user_service::decode_jwt,
};

pub async fn get_profile_controller(path: web::Path<i32>) -> HttpResponse {
    match get_profile(path.into_inner()).await {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

pub async fn upsert_profile_controller(
    req: HttpRequest,
    payload: web::Json<serde_json::Value>,
) -> HttpResponse {
    let uid: i32 = req
        .headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0); // or return Err if invalid

    let builder = ProfileBuilder::new(uid)
        .name(payload.get("name").and_then(|v| v.as_str()).unwrap_or_default())
        .phone(payload.get("phone").and_then(|v| v.as_str()).unwrap_or_default())
        .bio(payload.get("bio").and_then(|v| v.as_str()).unwrap_or_default());

    match upsert_profile(builder).await {
        Ok(_) => HttpResponse::Ok().json("Profile saved"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

pub async fn get_own_profile_controller(req: HttpRequest) -> HttpResponse {
    match extract_user_id(&req) {
        Ok(uid) => match get_profile(uid).await {
            Ok(p) => HttpResponse::Ok().json(p),
            Err(e) => HttpResponse::BadRequest().json(e),
        },
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

pub fn extract_user_id(req: &HttpRequest) -> Result<i32, String> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or("Missing or malformed Authorization header")?;

    let claims = decode_jwt(token)?;
    Ok(claims.sub)
}