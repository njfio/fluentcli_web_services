// src/error.rs

use thiserror::Error;
use std::error::Error as StdError;
use diesel::r2d2;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("R2D2 error: {0}")]
    R2D2Error(#[from] r2d2::PoolError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Generic error: {0}")]
    GenericError(Box<dyn StdError + Send + Sync>),

    #[error("Migration error: {0}")]
    MigrationError(#[from] diesel_migrations::MigrationError),

    #[error("Authentication error")]
    AuthenticationError,

    #[error("Fluent CLI error: {0}")]
    FluentCLIError(String),
}

impl From<Box<dyn StdError + Send + Sync>> for AppError {
    fn from(error: Box<dyn StdError + Send + Sync>) -> Self {
        AppError::GenericError(error)
    }
}
