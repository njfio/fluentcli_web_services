use crate::error::AppError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
    jti: String,
    version: u64,
}

pub fn generate_token(user_id: Uuid) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let version = increment_token_version(&user_id);

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
        jti: Uuid::new_v4().to_string(),
        version,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError)
}

lazy_static! {
    static ref TOKEN_VERSIONS: RwLock<HashMap<Uuid, u64>> = RwLock::new(HashMap::new());
}

pub fn get_token_version(user_id: &Uuid) -> u64 {
    TOKEN_VERSIONS
        .read()
        .unwrap()
        .get(user_id)
        .cloned()
        .unwrap_or(0)
}

pub fn increment_token_version(user_id: &Uuid) -> u64 {
    let mut versions = TOKEN_VERSIONS.write().unwrap();
    let version = versions.entry(*user_id).or_insert(0);
    *version += 1;
    *version
}

pub fn validate_token(token: &str) -> Result<(Uuid, u64), AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::AuthenticationError)?;

    let current_time = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp <= current_time {
        return Err(AppError::AuthenticationError);
    }

    let current_version = get_token_version(&token_data.claims.sub);
    if token_data.claims.version != current_version {
        return Err(AppError::AuthenticationError);
    }

    Ok((token_data.claims.sub, token_data.claims.version))
}
