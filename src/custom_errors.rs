use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(diesel::result::Error),
    ValidationError(String),
    AuthenticationError,
    FluentCLIError(String),
    NotFound,
    InternalServerError,
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError => write!(f, "Authentication failed"),
            AppError::FluentCLIError(msg) => write!(f, "FluentCLI error: {}", msg),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}