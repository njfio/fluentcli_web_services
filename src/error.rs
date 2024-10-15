use actix_web;
use actix_web::error::BlockingError;
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

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Generic error: {0}")]
    GenericError(Box<dyn StdError + Send + Sync>),

    #[error("Migration error: {0}")]
    MigrationError(Box<dyn StdError + Send + Sync>),

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

    #[error("Yaml parse error: {0}")]
    YamlParseError(String),

    #[error("LLM error: {0}")]
    LLMError(String),

    #[error("OpenAI error: {0}")]
    OpenAIError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Unsupported LLM provider: {0}")]
    UnsupportedProviderError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Bad request: {0}")]
    BadRequestError(String),
}

impl From<BlockingError> for AppError {
    fn from(error: BlockingError) -> Self {
        AppError::GenericError(Box::new(error))
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
            AppError::UnsupportedProviderError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": self.to_string() }))
    }
}
