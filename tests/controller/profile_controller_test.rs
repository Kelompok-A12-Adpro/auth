extern crate auth;

use actix_web::{http::StatusCode, test, web, App};
use auth::{
    controller::profile_controller::{get_profile_controller, upsert_profile_controller},
    factory::connection_factory::ConnectionFactory,
};
use dotenv::dotenv;
use serde_json::json;

#[actix_rt::test]
async fn test_profile_endpoints() {
    dotenv().ok();
    let pool = ConnectionFactory::new().get_pool();
    let srv = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .route("/profile", web::post().to(upsert_profile_controller))
            .route("/profile/{id}", web::get().to(get_profile_controller)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/profile")
        .append_header(("Content-Type", "application/json"))
        .set_json(json!({
            "name": "Controller Test",
            "phone": "0811111111",
            "bio": "Testing"
        }))
        .to_request();
    let resp = test::call_service(&srv, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get().uri("/profile/1").to_request();
    let resp = test::call_service(&srv, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
