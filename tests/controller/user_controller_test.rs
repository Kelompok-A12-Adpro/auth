extern crate auth;

use actix_web::{http::StatusCode, test, web, App};
use auth::controller::user_controller::{register_user_controller, login_user_controller};
use auth::model::user::{LoginRequest, User};
use serde_json::json;
use dotenv::dotenv;

use mockall::predicate::*;
use mockall::*;

pub trait UserService {
    fn register_user<'a>(&'a self, user: User) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + 'a>>;
    fn login_user<'a>(&'a self, login_req: LoginRequest) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + 'a>>;
}

mock! {
    pub UserService {}
    impl UserService for UserService {
        fn register_user<'a>(&'a self, user: User) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + 'a>>;
        fn login_user<'a>(&'a self, login_req: LoginRequest) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + 'a>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[actix_rt::test]
    async fn test_register_user_endpoint() {
        dotenv().ok();
        
        
        let mut mock_service = MockUserService::new();
        
        
        mock_service
            .expect_register_user()
            .returning(|_| {
                Box::pin(async { Ok("User registered successfully".to_string()) })
            });
        
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .route("/auth/register", web::post().to(register_user_controller))
        ).await;

        
        let user = User {
            id: 0, 
            email: "test_register@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
            is_admin: false,
            bio: "This is a test user".to_string(),
        };

        
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&user)
            .to_request();

        
        let resp = test::call_service(&app, req).await;

        
        assert_eq!(resp.status(), StatusCode::OK);
    }

    
    #[actix_rt::test]
    async fn test_login_user_endpoint() {
        dotenv().ok();
        
        
        let mut mock_service = MockUserService::new();
        
        
        mock_service
            .expect_login_user()
            .returning(|_| {
                Box::pin(async { Ok("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.mock.token".to_string()) })
            });
        
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_service))
                .route("/auth/login", web::post().to(login_user_controller))
        ).await;

        
        let login_req = LoginRequest {
            email: "test_login@example.com".to_string(),
            password: "password123".to_string(),
        };

        
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_req)
            .to_request();

        
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        
        let body = test::read_body(resp).await;
        assert!(!body.is_empty());
    }
}