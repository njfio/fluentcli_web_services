use std::fmt;

/// Represents an error that can occur during tool execution
#[derive(Debug)]
pub enum ToolError {
    /// The tool was not found in the registry
    ToolNotFound(String),
    /// A required parameter is missing
    MissingParameter(String),
    /// A parameter has an invalid value
    InvalidArgument(String),
    /// An error occurred while executing the tool
    ExecutionError(String),
    /// An error occurred while communicating with an external service
    ExternalServiceError(String),
    /// An error occurred while deserializing data
    DeserializationError(String),
    /// An error occurred due to authentication failure
    AuthenticationError(String),
    /// An error occurred due to rate limiting
    RateLimitExceeded(String),
    /// An error occurred due to permission issues
    PermissionDenied(String),
    /// An error occurred while communicating over the network
    NetworkError(String),
    /// An error occurred while executing a remote tool
    RemoteToolError(String),
    /// An error occurred while discovering tools
    DiscoveryError(String),
    /// An internal error occurred
    InternalError(String),
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolNotFound(name) => write!(f, "Tool not found: {}", name),
            Self::MissingParameter(param) => write!(f, "Missing required parameter: {}", param),
            Self::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            Self::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            Self::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            Self::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            Self::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
            Self::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::RemoteToolError(msg) => write!(f, "Remote tool error: {}", msg),
            Self::DiscoveryError(msg) => write!(f, "Discovery error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ToolError {}

impl From<serde_json::Error> for ToolError {
    fn from(error: serde_json::Error) -> Self {
        Self::DeserializationError(error.to_string())
    }
}

impl From<reqwest::Error> for ToolError {
    fn from(error: reqwest::Error) -> Self {
        Self::NetworkError(error.to_string())
    }
}
