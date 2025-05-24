use auth::model::user::User;
use auth::repository::user_repository::{create_user, find_user_by_email};
use auth::factory::connection_factory::ConnectionFactory;
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
            is_admin: false,
            bio: "Aku bersedia membagikan cintaku kepada dunia 💗".to_string(),
        };
        
        
        let new_user = auth::model::user::NewUser {
            email: &test_user.email,
            password: &test_user.password,
            name: &test_user.name,
            phone: &test_user.phone,
            is_admin: test_user.is_admin,
        };
        let result = create_user(new_user).await;
        
        
        assert!(result.is_ok(), "Failed to create user: {:?}", result);
        
        
        let found_user = find_user_by_email(&test_user.email).await;
        assert!(found_user.is_some(), "User was not found after creation");
        
        if let Some(found_user) = found_user {
            assert_eq!(found_user.email, test_user.email);
            assert_eq!(found_user.password, test_user.password);
            assert_eq!(found_user.name, test_user.name);
            assert_eq!(found_user.phone, test_user.phone);
            assert_eq!(found_user.is_admin, test_user.is_admin);
            assert_eq!(found_user.bio, test_user.bio);
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
            is_admin: false,
            bio: "This is a test user".to_string(),
        };
        
        
        let new_user = auth::model::user::NewUser {
            email: &test_user.email,
            password: &test_user.password,
            name: &test_user.name,
            phone: &test_user.phone,
            is_admin: test_user.is_admin,
        };
        create_user(new_user).await.expect("Failed to create test user");
        
        
        let found_user = find_user_by_email(&test_user.email).await;
        
        
        assert!(found_user.is_some(), "User was not found after creation");
        
        if let Some(found_user) = found_user {
            assert_eq!(found_user.email, test_user.email, "Email doesn't match");
        }
        
        
        let not_found = find_user_by_email("nonexistent@example.com").await;
        
        
        assert!(not_found.is_none(), "Found a user that shouldn't exist");
        
        
        let _ = cleanup_test_db().expect("Failed to clean up test database");
    }
}