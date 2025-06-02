use crate::repository::user_repository::{create_user, delete_user, find_user_by_email, find_user_by_id, get_all_users, get_recent_users_and_total};
use crate::model::user::{LoginRequest, NewUser, RegisterRequest, User, UserDataResponse};
use bcrypt::{hash, verify};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn register_user(req: RegisterRequest) -> Result<String, String> {
    let trimmed_name = req.name.trim();
    let trimmed_phone = req.phone.trim();
    let trimmed_email = req.email.trim();
    
    if trimmed_name.is_empty() || trimmed_phone.is_empty() {
        return Err("Name and phone cannot be empty".to_string());
    }
    
    if trimmed_email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    if let Some(_) = find_user_by_email(trimmed_email).await {
        return Err("Email already registered".to_string());
    }

    let hashed_password = hash_password(&req.password)?;

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
            if verify_password(&req.password, &user.password) {
                create_jwt_token(&user)
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
}

pub async fn get_users_data() -> Result<UserDataResponse, String> {
    get_recent_users_and_total()
        .await
        .map_err(|e| format!("Error fetching user data: {}", e))
}

pub async fn get_all_users_data() -> Result<UserDataResponse, String> {
    get_all_users()
        .await
        .map_err(|e| format!("Error fetching all users: {}", e))
}

pub async fn get_user_data(id: i32) -> Result<User, String> {
    find_user_by_id(id)
        .await
        .ok_or_else(|| "User not found".to_string())
}

pub async fn delete_user_by_id(uid: i32) -> Result<String, String> {
    delete_user(uid).await
        .map_err(|e| format!("Error deleting user: {}", e))
        .map(|_| "User deleted successfully".to_string())
}


fn hash_password(password: &str) -> Result<String, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    
    hash(password, 12).map_err(|e| format!("Error hashing password: {}", e))
}

fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub email: String,
    pub is_admin: bool,
}

fn create_jwt_token(user: &User) -> Result<String, String> {

    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("System time error: {}", e))?
        .as_secs() + 3600;

    let claims = Claims {
        sub: user.id,
        exp: expiration_time as usize,
        email: user.email.clone(),
        is_admin: user.is_admin,
    };

    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable not set".to_string())?;

    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
        .map_err(|e| format!("Error creating JWT token: {}", e))
}

pub fn decode_jwt(token: &str) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable not set".to_string())?;

    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(
        token,
        &decoding_key,
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Invalid token: {}", e))
}