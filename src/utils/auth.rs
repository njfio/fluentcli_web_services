use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::InternalServerError)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|_| AppError::InternalServerError)
}
