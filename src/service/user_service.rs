use crate::repository::user_repository::{create_user, find_user_by_email};
use crate::model::user::{RegisterRequest, LoginRequest, NewUser};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn register_user(req: RegisterRequest) -> Result<String, String> {
    if req.name.trim().is_empty() || req.phone.trim().is_empty() {
        return Err("Name and phone cannot be empty".to_string());
    }

    if let Some(_) = find_user_by_email(&req.email).await {
        return Err("Email already registered".to_string());
    }

    let hashed_password = hash_password(&req.password);

    let new_user = NewUser {
        email: &req.email,
        password: &hashed_password,
        name: &req.name,
        phone: &req.phone,
        is_admin: false,
    };

    create_user(new_user).await?;
    Ok("User registered successfully".to_string())
}

pub async fn login_user(req: LoginRequest) -> Result<String, String> {
    match find_user_by_email(&req.email).await {
        Some(user) => {
            if verify_password(&req.password, &user.password) {
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

fn create_jwt_token(user_email: &str) -> String {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }

    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600;

    let claims = Claims {
        sub: user_email.to_string(),
        exp: expiration_time as usize,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key).unwrap()
}
