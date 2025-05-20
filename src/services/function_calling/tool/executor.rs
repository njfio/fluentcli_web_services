use async_trait::async_trait;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use crate::services::function_calling::tool::error::ToolError;

/// Base trait for tool metadata and validation
pub trait ToolExecutorBase: Send + Sync {
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

/// Trait for executing tools asynchronously
#[async_trait]
pub trait ToolExecutor: ToolExecutorBase {
    /// Executes the tool with the given arguments
    async fn execute(&self, args: Value) -> Result<Value, ToolError>;
}

/// Trait for dynamic dispatch of tool execution
pub trait DynToolExecutor: ToolExecutorBase {
    /// Executes the tool with the given arguments
    fn execute<'a>(&'a self, args: Value) -> Pin<Box<dyn Future<Output = Result<Value, ToolError>> + Send + 'a>>;
}

/// Implement DynToolExecutor for any type that implements ToolExecutor
#[async_trait]
impl<T: ToolExecutor + ?Sized> DynToolExecutor for T {
    fn execute<'a>(&'a self, args: Value) -> Pin<Box<dyn Future<Output = Result<Value, ToolError>> + Send + 'a>> {
        Box::pin(self.execute(args))
    }
}
