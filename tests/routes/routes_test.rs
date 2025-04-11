extern crate auth;

use actix_web::{http::StatusCode, test, web, App};
use auth::routes::routes::init_routes;
use dotenv::dotenv;
use auth::factory::connection_factory::ConnectionFactory;

#[cfg(test)]
mod tests {
    use super::*;

    
    async fn setup_test_app() -> impl actix_web::dev::Service<
        actix_web::test::TestRequest,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        dotenv().ok();
        
        
        let connection_factory = ConnectionFactory::new();
        let pool = connection_factory.get_pool();
        
        test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(init_routes)
        ).await
    }

    
    #[actix_rt::test]
    async fn test_routes_configured() {
        
        let app = setup_test_app().await;
        
        
        let req = test::TestRequest::post()
            .uri("/register")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "Register route not found");
        
        
        let req = test::TestRequest::post()
            .uri("/login")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "Login route not found");
    }
    
    
    #[actix_rt::test]
    async fn test_http_method_restrictions() {
        
        let app = setup_test_app().await;
        
        
        let req = test::TestRequest::get()
            .uri("/register")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED, "GET method should not be allowed for register");
        
        
        let req = test::TestRequest::get()
            .uri("/login")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED, "GET method should not be allowed for login");
    }
}