extern crate auth;


#[cfg(test)]
mod integration_tests {
    use actix_web::{test, web, App, http::StatusCode};
    use auth::routes::routes::init_routes;
    use auth::factory::connection_factory::ConnectionFactory;
    use diesel::connection::SimpleConnection;
    use serde_json::json;
    use dotenv::dotenv;

    
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
                .configure(init_routes)
        ).await
    }

    
    fn cleanup_test_data() -> Result<(), diesel::result::Error> {
        let connection_factory = ConnectionFactory::new();
        let conn = &mut connection_factory.get_connection();
        
        
        conn.batch_execute("DELETE FROM users WHERE email LIKE '%test%' OR email LIKE '%integration%'")?;
        Ok(())
    }

    
    #[actix_rt::test]
    async fn test_user_registration_flow() {
        
        let app = setup_test_app().await;
        
        
        let _ = cleanup_test_data(); 
        
        
        let test_user = json!({
            "id": 0,
            "email": "integration_test@example.com",
            "password": "secure_password123",
            "name": "Integration Test User",
            "phone": "1234567890"
        });
        
        
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&test_user)
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        
        let login_req = json!({
            "email": "integration_test@example.com",
            "password": "secure_password123"
        });
        
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_req)
            .to_request();
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        
        let body = test::read_body(resp).await;
        let token = String::from_utf8(body.to_vec()).expect("Token should be valid UTF-8");
        
        
        assert!(token.len() > 2);
        
        
        let _ = cleanup_test_data();
    }
}