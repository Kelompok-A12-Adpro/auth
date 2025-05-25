extern crate auth;

use actix_web::{http::StatusCode, test, web, App};
use auth::controller::user_controller::{register_user_controller, login_user_controller};
use auth::model::user::{LoginRequest, RegisterRequest};
use auth::factory::connection_factory::ConnectionFactory;
use serde_json::json;
use dotenv::dotenv;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to set up test app with database
    async fn setup_test_app() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        dotenv().ok();
        
        let connection_factory = ConnectionFactory::new();
        let pool = connection_factory.get_pool();
        
        test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/auth/register", web::post().to(register_user_controller))
                .route("/auth/login", web::post().to(login_user_controller))
        ).await
    }

    // Helper function to cleanup test data
    fn cleanup_test_data(email_pattern: &str) -> Result<(), diesel::result::Error> {
        use diesel::connection::SimpleConnection;
        
        let connection_factory = ConnectionFactory::new();
        let conn = &mut connection_factory.get_connection();
        
        let query = format!("DELETE FROM users WHERE email LIKE '%{}%'", email_pattern);
        conn.batch_execute(&query)?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_register_user_endpoint() {
        let app = setup_test_app().await;
        
        // Clean up any existing test data
        let _ = cleanup_test_data("test_register_controller");
        
        let register_req = RegisterRequest {
            email: "test_register_controller@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&register_req)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Should return OK for successful registration
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Clean up
        let _ = cleanup_test_data("test_register_controller");
    }

    #[actix_rt::test]
    async fn test_login_user_endpoint() {
        let app = setup_test_app().await;
        
        // Clean up any existing test data
        let _ = cleanup_test_data("test_login_controller");
        
        // First register a user
        let register_req = RegisterRequest {
            email: "test_login_controller@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&register_req)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Now try to login
        let login_req = LoginRequest {
            email: "test_login_controller@example.com".to_string(),
            password: "password123".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Verify we get a token back
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).expect("Body should be UTF-8");
        
        // Remove quotes if present (JSON string response)
        let token = body_str.trim_matches('"');
        assert!(token.len() > 10, "Token should be substantial length");
        assert!(token.contains('.'), "Token should look like JWT");
        
        // Clean up
        let _ = cleanup_test_data("test_login_controller");
    }

    #[actix_rt::test]
    async fn test_login_invalid_credentials() {
        let app = setup_test_app().await;
        
        // Clean up any existing test data
        let _ = cleanup_test_data("test_invalid_login");
        
        // First register a user
        let register_req = RegisterRequest {
            email: "test_invalid_login@example.com".to_string(),
            password: "correct_password".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&register_req)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Try to login with wrong password
        let login_req = LoginRequest {
            email: "test_invalid_login@example.com".to_string(),
            password: "wrong_password".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_req)
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        
        // Clean up
        let _ = cleanup_test_data("test_invalid_login");
    }
}