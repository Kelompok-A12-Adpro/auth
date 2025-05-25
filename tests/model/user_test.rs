use auth::model::user::{User, NewUser, LoginRequest};
use serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serde() {
        let user = User {
            id: 1,
            email: "test@example.com".to_string(),
            password: "hashed_password".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
            is_admin: false,
            bio: "This is a test user".to_string(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize User");
        
        let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize User");
        
        assert_eq!(deserialized.id, user.id);
        assert_eq!(deserialized.email, user.email);
        assert_eq!(deserialized.password, user.password);
        assert_eq!(deserialized.name, user.name);
        assert_eq!(deserialized.phone, user.phone);
        assert_eq!(deserialized.is_admin, user.is_admin);
        assert_eq!(deserialized.bio, user.bio);
    }
    
    #[test]
    fn test_new_user_creation() {
        let new_user = NewUser {
            email: "test@example.com",
            password: "password123",
            name: "Test User",
            phone: "1234567890",
            is_admin: false,
        };
        
        assert_eq!(new_user.email, "test@example.com");
        assert_eq!(new_user.password, "password123");
        assert_eq!(new_user.name, "Test User");
        assert_eq!(new_user.phone, "1234567890");
        assert_eq!(new_user.is_admin, false);
    }
    
    #[test]
    fn test_login_request_serde() {
        let login_req = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
       
        let json = serde_json::to_string(&login_req).expect("Failed to serialize LoginRequest");
        
      
        let deserialized: LoginRequest = serde_json::from_str(&json).expect("Failed to deserialize LoginRequest");
        
       
        assert_eq!(deserialized.email, login_req.email);
        assert_eq!(deserialized.password, login_req.password);
    }
    
    #[test]
    fn test_json_format_validation() {
        let valid_json = r#"{"email":"test@example.com","password":"password123","name":"Test User","phone":"1234567890","id":1,"is_admin":false,"bio":"This is a test user"}"#;
        let user_result: Result<User, _> = serde_json::from_str(valid_json);
        assert!(user_result.is_ok());
        
        let invalid_json = r#"{"email":"test@example.com","password":"password123"}"#;
        let user_result: Result<User, _> = serde_json::from_str(invalid_json);
        assert!(user_result.is_err());
        
        let valid_login_json = r#"{"email":"test@example.com","password":"password123"}"#;
        let login_result: Result<LoginRequest, _> = serde_json::from_str(valid_login_json);
        assert!(login_result.is_ok());

        let invalid_login_json = r#"{"email":"test@example.com"}"#;
        let login_result: Result<LoginRequest, _> = serde_json::from_str(invalid_login_json);
        assert!(login_result.is_err());
    }
}