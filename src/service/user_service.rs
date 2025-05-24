// src/service/user_service.rs - Complete corrected version
use crate::repository::user_repository::{create_user, find_user_by_email};
use crate::model::user::{RegisterRequest, LoginRequest, NewUser};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn register_user(req: RegisterRequest) -> Result<String, String> {
    // Trim and validate inputs
    let trimmed_name = req.name.trim();
    let trimmed_phone = req.phone.trim();
    let trimmed_email = req.email.trim();
    
    if trimmed_name.is_empty() || trimmed_phone.is_empty() {
        return Err("Name and phone cannot be empty".to_string());
    }
    
    if trimmed_email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    // Check if user already exists
    if let Some(_) = find_user_by_email(trimmed_email).await {
        return Err("Email already registered".to_string());
    }

    // Hash password
    let hashed_password = hash_password(&req.password)?;

    // Create new user with trimmed values
    let new_user = NewUser {
        email: trimmed_email,
        password: &hashed_password,
        name: trimmed_name,
        phone: trimmed_phone,
        is_admin: false,
    };

    create_user(new_user).await?;
    Ok("User registered successfully".to_string())
}

pub async fn login_user(req: LoginRequest) -> Result<String, String> {
    let trimmed_email = req.email.trim();
    
    if trimmed_email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    
    match find_user_by_email(trimmed_email).await {
        Some(user) => {
            // ⚠️ CRITICAL FIX: Remove the ? operator here since verify_password returns bool
            if verify_password(&req.password, &user.password) {
                create_jwt_token(&user.email)
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
}

// Fixed helper functions
fn hash_password(password: &str) -> Result<String, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    
    hash(password, 12).map_err(|e| format!("Error hashing password: {}", e))
}

// IMPORTANT: Keep this as bool return, don't change to Result<bool, String>
fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

fn create_jwt_token(user_email: &str) -> Result<String, String> {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }

    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("System time error: {}", e))?
        .as_secs() + 3600;

    let claims = Claims {
        sub: user_email.to_string(),
        exp: expiration_time as usize,
    };

    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable not set".to_string())?;
    
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
        .map_err(|e| format!("Error creating JWT token: {}", e))
}
