use async_trait::async_trait;
use serde_json::Value;
use crate::services::function_calling::tool::error::ToolError;

/// Trait for executing tools
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Executes the tool with the given arguments
    async fn execute(&self, args: Value) -> Result<Value, ToolError>;
    
    /// Returns the name of the tool
    fn name(&self) -> &str;
    
    /// Returns the description of the tool
    fn description(&self) -> &str;
    
    /// Validates the arguments against the tool's schema
    fn validate_args(&self, args: &Value) -> Result<(), ToolError> {
        // Default implementation does no validation
        Ok(())
    }
}
