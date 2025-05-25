use actix_web::{web, HttpResponse};
use crate::{
    model::profile::ProfileBuilder,
    service::profile_service::{get_profile, upsert_profile},
};

pub async fn get_profile_controller(path: web::Path<i32>) -> HttpResponse {
    match get_profile(path.into_inner()).await {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

pub async fn upsert_profile_controller(
    uid: web::ReqData<i32>,         
    payload: web::Json<serde_json::Value>, 
) -> HttpResponse {
    let builder = ProfileBuilder::new(*uid)
        .name(payload.get("name").and_then(|v| v.as_str()).unwrap_or_default())
        .phone(payload.get("phone").and_then(|v| v.as_str()).unwrap_or_default())
        .bio(
            payload
                .get("bio")
                .and_then(|v| v.as_str())
                .unwrap_or_default(),
        );

    match upsert_profile(builder).await {
        Ok(_) => HttpResponse::Ok().json("Profile saved"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
