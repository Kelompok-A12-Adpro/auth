use auth::model::user::{User, LoginRequest};
use auth::service::user_service::{register_user, login_user};
use mockall::predicate::*;
use mockall::*;
use std::env;
use dotenv::dotenv;
use bcrypt::{hash, verify};

mock! {
    pub UserRepository {}
    
    impl UserRepository {
        pub async fn find_user_by_email(email: &str) -> Option<User>;
        pub async fn create_user(user: User) -> Result<(), String>;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    fn setup_env() {
        dotenv().ok();
        
        if env::var("JWT_SECRET").is_err() {
            env::set_var("JWT_SECRET", "test_secret_key_for_jwt");
        }
    }

    
    #[actix_rt::test]
    async fn test_register_user_success() {
        setup_env();
        
        
        
        
        let test_email = "test_register_success@example.com";
        let test_password = "secure_password123";
        
        
        let user = User {
            id: 0,
            email: test_email.to_string(),
            password: test_password.to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
        };
        
        
        let result = register_user(user).await;
        
        
        
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
        
        
        let user = User {
            id: 0,
            email: test_email.to_string(),
            password: test_password.to_string(),
            name: "Login Test User".to_string(),
            phone: "1234567890".to_string(),
        };
        
        let _ = register_user(user).await; 
        
        
        let login_req = LoginRequest {
            email: test_email.to_string(),
            password: test_password.to_string(),
        };
        
        let result = login_user(login_req).await;
        
        
        assert!(result.is_ok(), "Login failed: {:?}", result.err());
        
        
        let token = result.unwrap();
        assert!(!token.is_empty(), "Token is empty");
        assert!(token.contains("."), "Token doesn't appear to be valid JWT format");
    }
    
    
    #[actix_rt::test]
    async fn test_login_user_invalid_password() {
        setup_env();
        
        
        let test_email = "test_login_invalid@example.com";
        let correct_password = "correct_password123";
        
        
        let user = User {
            id: 0,
            email: test_email.to_string(),
            password: correct_password.to_string(),
            name: "Invalid Login Test User".to_string(),
            phone: "1234567890".to_string(),
        };
        
        let _ = register_user(user).await; 
        
        
        let login_req = LoginRequest {
            email: test_email.to_string(),
            password: "wrong_password".to_string(), 
        };
        
        let result = login_user(login_req).await;
        
        
        assert!(result.is_err(), "Login should have failed with wrong password");
        assert_eq!(result.unwrap_err(), "Invalid credentials");
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