use crate::error::AppError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
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
    info!("Generating token for user: {}", user_id);
    debug!("Token claims: {:?}", claims);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        error!("Failed to generate token: {}", e);
        AppError::InternalServerError
    })
}

lazy_static! {
    static ref TOKEN_VERSIONS: RwLock<HashMap<Uuid, u64>> = RwLock::new(HashMap::new());
}

pub fn get_token_version(user_id: &Uuid) -> u64 {
    let version = TOKEN_VERSIONS
        .read()
        .unwrap()
        .get(user_id)
        .cloned()
        .unwrap_or(0);
    debug!("Current token version for user {}: {}", user_id, version);
    version
}

pub fn increment_token_version(user_id: &Uuid) -> u64 {
    let mut versions = TOKEN_VERSIONS.write().unwrap();
    let version = versions.entry(*user_id).or_insert(0);
    *version += 1;
    debug!(
        "Incremented token version for user {} to {}",
        user_id, *version
    );
    *version
}

pub fn validate_token(token: &str) -> Result<(Uuid, u64), AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    info!("Validating token");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        error!("Token decoding failed: {}", e);
        AppError::AuthenticationError
    })?;

    let current_time = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp <= current_time {
        warn!(
            "Token has expired. Expiration: {}, Current time: {}",
            token_data.claims.exp, current_time
        );
        return Err(AppError::AuthenticationError);
    }

    let current_version = get_token_version(&token_data.claims.sub);
    if token_data.claims.version != current_version {
        warn!(
            "Token version mismatch. Token version: {}, Current version: {}",
            token_data.claims.version, current_version
        );
        return Err(AppError::AuthenticationError);
    }

    info!(
        "Token validated successfully for user: {}",
        token_data.claims.sub
    );
    Ok((token_data.claims.sub, token_data.claims.version))
}
