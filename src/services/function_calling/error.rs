use std::fmt;
use crate::services::function_calling::tool::error::ToolError;

/// Represents an error that can occur in the function calling system
#[derive(Debug)]
pub enum FunctionCallingError {
    /// Error occurred while preparing the request
    RequestPreparationError(String),
    /// Error occurred while parsing the response
    ResponseParsingError(String),
    /// Error occurred while executing a tool
    ToolExecutionError(ToolError),
    /// Error occurred while communicating with the LLM provider
    ProviderError(String),
    /// Error occurred due to an invalid configuration
    ConfigurationError(String),
}

impl fmt::Display for FunctionCallingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestPreparationError(msg) => write!(f, "Request preparation error: {}", msg),
            Self::ResponseParsingError(msg) => write!(f, "Response parsing error: {}", msg),
            Self::ToolExecutionError(err) => write!(f, "Tool execution error: {}", err),
            Self::ProviderError(msg) => write!(f, "Provider error: {}", msg),
            Self::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for FunctionCallingError {}

impl From<ToolError> for FunctionCallingError {
    fn from(error: ToolError) -> Self {
        Self::ToolExecutionError(error)
    }
}

impl From<serde_json::Error> for FunctionCallingError {
    fn from(error: serde_json::Error) -> Self {
        Self::ResponseParsingError(error.to_string())
    }
}

impl From<reqwest::Error> for FunctionCallingError {
    fn from(error: reqwest::Error) -> Self {
        Self::ProviderError(error.to_string())
    }
}
