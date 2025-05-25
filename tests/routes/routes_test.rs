extern crate auth;

use actix_web::{http::StatusCode, test, web, App};
use auth::routes::routes::init_routes;
use dotenv::dotenv;
use auth::factory::connection_factory::ConnectionFactory;

#[cfg(test)]
mod tests {
    use super::*;

    
    fn setup_test_app() -> App<
        impl actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        dotenv().ok();

        let connection_factory = ConnectionFactory::new();
        let pool = connection_factory.get_pool();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(init_routes)
    }

    
    #[actix_rt::test]
    async fn test_routes_configured() {
        
        let app = test::init_service(setup_test_app()).await;
        
        
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "Register route not found");
        
        
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        
        assert_ne!(resp.status(), StatusCode::NOT_FOUND, "Login route not found");
    }
    
    #[actix_rt::test]
    async fn test_http_method_restrictions() {
        let app = test::init_service(setup_test_app()).await;

        let req = test::TestRequest::get()
            .uri("/auth/register")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED, "GET method should not be allowed for register");

        let req = test::TestRequest::get()
            .uri("/auth/login")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED, "GET method should not be allowed for login");
    }
}