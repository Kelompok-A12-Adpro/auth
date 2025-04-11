use crate::model::user::User;
use crate::repository::user_repository::{create_user, find_user_by_email};
use crate::factory::connection_factory::ConnectionFactory;
use diesel::connection::SimpleConnection;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use std::env;


fn setup_test_db() -> Result<(), diesel::result::Error> {
    dotenv().ok();
    let connection_factory = ConnectionFactory::new();
    let conn = &mut connection_factory.get_connection();
    
    
    conn.batch_execute("DELETE FROM users WHERE email LIKE '%test%'")?;
    Ok(())
}

fn cleanup_test_db() -> Result<(), diesel::result::Error> {
    let connection_factory = ConnectionFactory::new();
    let conn = &mut connection_factory.get_connection();
    
    
    conn.batch_execute("DELETE FROM users WHERE email LIKE '%test%'")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[actix_rt::test]
    async fn test_create_user() {
        
        let _ = setup_test_db().expect("Failed to set up test database");
        
        
        let test_user = User {
            id: 0, 
            email: "test_create@example.com".to_string(),
            password: "hashed_password".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
        };
        
        
        let result = create_user(test_user.clone()).await;
        
        
        assert!(result.is_ok(), "Failed to create user: {:?}", result);
        
        
        let found_user = find_user_by_email(&test_user.email).await;
        assert!(found_user.is_some(), "User was not found after creation");
        
        if let Some(found_user) = found_user {
            assert_eq!(found_user.email, test_user.email);
            assert_eq!(found_user.password, test_user.password);
            assert_eq!(found_user.name, test_user.name);
            assert_eq!(found_user.phone, test_user.phone);
        }
        
        
        let _ = cleanup_test_db().expect("Failed to clean up test database");
    }
    
    
    #[actix_rt::test]
    async fn test_find_user_by_email() {
        
        let _ = setup_test_db().expect("Failed to set up test database");
        
        
        let test_user = User {
            id: 0,
            email: "test_find@example.com".to_string(),
            password: "hashed_password".to_string(),
            name: "Test User".to_string(),
            phone: "1234567890".to_string(),
        };
        
        
        create_user(test_user.clone()).await.expect("Failed to create test user");
        
        
        let found_user = find_user_by_email(&test_user.email).await;
        
        
        assert!(found_user.is_some(), "User was not found after creation");
        
        if let Some(found_user) = found_user {
            assert_eq!(found_user.email, test_user.email, "Email doesn't match");
        }
        
        
        let not_found = find_user_by_email("nonexistent@example.com").await;
        
        
        assert!(not_found.is_none(), "Found a user that shouldn't exist");
        
        
        let _ = cleanup_test_db().expect("Failed to clean up test database");
    }
    
    
    #[actix_rt::test]
    async fn test_unique_email_constraint() {
        
        let _ = setup_test_db().expect("Failed to set up test database");
        
        
        let user1 = User {
            id: 0,
            email: "test_unique@example.com".to_string(),
            password: "password1".to_string(),
            name: "Test User 1".to_string(),
            phone: "1234567890".to_string(),
        };
        
        let result1 = create_user(user1.clone()).await;
        assert!(result1.is_ok(), "Failed to create first user: {:?}", result1);
        
        
        let user2 = User {
            id: 0,
            email: "test_unique@example.com".to_string(), 
            password: "password2".to_string(),
            name: "Test User 2".to_string(), 
            phone: "0987654321".to_string(),
        };
        
        
        let result2 = create_user(user2).await;
        
        
        assert!(result2.is_err(), "Created user with duplicate email, which should not be possible");
        
        
        let _ = cleanup_test_db().expect("Failed to clean up test database");
    }
}