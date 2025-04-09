use crate::repositories::auth_repository::{create_user, find_user_by_email};
use crate::models::user::{User, LoginRequest};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use std::env;

pub async fn register_user(user: User) -> Result<String, String> {
    if let Some(_) = find_user_by_email(&user.email).await {
        return Err("Email already registered".to_string());
    }
    
    let hashed_password = hash_password(&user.password);
    let user = User {
        password: hashed_password,
        ..user
    };

    match create_user(user).await {
        Ok(_) => Ok("User registered successfully".to_string()),
        Err(e) => Err(e),
    }
}

pub async fn login_user(login_req: LoginRequest) -> Result<String, String> {
    match find_user_by_email(&login_req.email).await {
        Some(user) => {
            if verify_password(&login_req.password, &user.password) {
                Ok(create_jwt_token(&user.email))
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
}

fn hash_password(password: &str) -> String {
    hash(password, 12).expect("Error hashing password")
}

fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

fn create_jwt_token(user: &str) -> String {
    let claims = jsonwebtoken::Claims {
        sub: user.to_string(),
        exp: 10000000000, // Set expiration time
    };

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key).unwrap()
}
