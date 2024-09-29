// src/error.rs

use actix_web;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use diesel::r2d2;
use std::error::Error as StdError;
use thiserror::Error;

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

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Pipeline error: {0}")]
    PipelineError(String),

    #[error("Job error: {0}")]
    JobError(String),

    #[error("Docker file error: {0}")]
    DockerFileError(String),

    #[error("Trigger error: {0}")]
    TriggerError(String),

    #[error("Timer error: {0}")]
    TimerError(String),

    #[error("Worker error: {0}")]
    WorkerError(String),

    #[error("Temp file error: {0}")]
    TempFileError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<Box<dyn StdError + Send + Sync>> for AppError {
    fn from(error: Box<dyn StdError + Send + Sync>) -> Self {
        AppError::GenericError(error)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::AuthenticationError => StatusCode::UNAUTHORIZED,
            AppError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": self.to_string() }))
    }
}
