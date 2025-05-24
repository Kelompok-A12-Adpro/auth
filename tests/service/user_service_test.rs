use auth::model::user::{User, LoginRequest};
use auth::service::user_service::{register_user, login_user};
use mockall::predicate::*;
use mockall::*;
use std::env;
use dotenv::dotenv;
use tokio::time::{sleep, Duration};

#[cfg(test)]
mod tests {
    use auth::model::user::RegisterRequest;

    use super::*;
    
    
    fn setup_env() {
        dotenv().ok();
        
        if env::var("JWT_SECRET").is_err() {
            env::set_var("JWT_SECRET", "adprojomox123");
        }
    }

    
    #[actix_rt::test]
    async fn test_register_user_success() {
        setup_env();
        
        
        
        
        let test_email = "test_register_success@example.com";
        let test_password = "secure_password123";
        
        
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            password: test_password.to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
            
        };
        
        
        let result = register_user(register_req).await;
        
        
        
        if result.is_ok() {
            assert_eq!(result.unwrap(), "User registered successfully");
        } else {
            let err = result.unwrap_err();
            assert!(err.contains("Email already registered") || err.contains("duplicate key"), 
                   "Unexpected error: {}", err);
        }
    }
    
    
    #[actix_rt::test]
    async fn test_login_user_success() {
        setup_env();

        let test_email = "test_login_success@example.com";
        let test_password = "secure_password123";

        let register_req = RegisterRequest {
            email: test_email.to_string(),
            password: test_password.to_string(),
            name: "Login Test User".to_string(),
            phone: "1234567890".to_string(),
        };

        let register_result = register_user(register_req).await;
        assert!(register_result.is_ok(), "Register failed: {:?}", register_result.as_ref().err());

        // Tunggu agar DB commit selesai
        sleep(Duration::from_millis(100)).await;

        let login_req = LoginRequest {
            email: test_email.to_string(),
            password: test_password.to_string(),
        };

        let result = login_user(login_req).await;

        assert!(result.is_ok(), "Login failed: {:?}", result.as_ref().err());

        let token = result.unwrap();
        assert!(!token.is_empty(), "Token is empty");
        assert!(token.contains("."), "Token doesn't appear to be valid JWT format");
    }

    #[actix_rt::test]
    async fn test_login_user_invalid_password() {
        setup_env();

        let test_email = "test_login_invalid@example.com";
        let correct_password = "correct_password123";

        let register_req = RegisterRequest {
            email: test_email.to_string(),
            password: correct_password.to_string(),
            name: "Invalid Login Test User".to_string(),
            phone: "1234567890".to_string(),
        };

        let register_result = register_user(register_req).await;
        assert!(register_result.is_ok(), "Register failed: {:?}", register_result.err());

        // Tunggu agar DB commit selesai
        sleep(Duration::from_millis(100)).await;

        let login_req = LoginRequest {
            email: test_email.to_string(),
            password: "wrong_password".to_string(),
        };

        let result = login_user(login_req).await;

        assert!(result.is_err(), "Login should have failed with wrong password");

        if let Err(err_msg) = result {
            assert!(
                err_msg == "Invalid credentials" || err_msg.contains("invalid password"),
                "Unexpected error: {}", err_msg
            );
        }
    }
    
    
    #[actix_rt::test]
    async fn test_login_user_not_found() {
        setup_env();
        
        
        let login_req = LoginRequest {
            email: "nonexistent_user@example.com".to_string(), 
            password: "any_password".to_string(),
        };
        
        let result = login_user(login_req).await;
        
        
        assert!(result.is_err(), "Login should fail for non-existent user");
        assert_eq!(result.unwrap_err(), "User not found");
    }
}