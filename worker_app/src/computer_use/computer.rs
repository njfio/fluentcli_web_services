//! Computer orchestration for tool execution.
//!
//! This module provides the main orchestration layer for tool execution,
//! handling tool lookup, validation, execution, and result formatting.
//! It follows the design patterns from Anthropic's computer-use implementation.

use crate::computer_use::base::{ToolCollection, ToolExecution, ToolInfo, ToolResult};
use log::{debug, error, info};
use serde_json::Value;
use std::error::Error;
use std::fmt;

/// Errors that can occur during computer operations.
#[derive(Debug)]
pub enum ComputerError {
    /// Tool was not found in the collection
    ToolNotFound(String),
    /// Error occurred during tool execution
    ExecutionError(String),
    /// Parameter validation failed
    ValidationError(String),
}

impl fmt::Display for ComputerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputerError::ToolNotFound(msg) => write!(f, "Tool not found: {}", msg),
            ComputerError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            ComputerError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for ComputerError {}

/// Main orchestrator for tool execution.
///
/// The Computer struct manages a collection of tools and provides methods
/// to execute them, handle errors, and format results.
#[derive(Debug)]
pub struct Computer {
    tools: ToolCollection,
}

impl Computer {
    /// Create a new Computer instance with default tools.
    pub fn new() -> Self {
        let tools = crate::computer_use::tools::create_tool_collection();
        Self { tools }
    }

    /// Execute a tool by name with given parameters.
    ///
    /// # Arguments
    /// * `tool_name` - Name of the tool to execute
    /// * `params` - Parameters for tool execution
    ///
    /// # Returns
    /// * `Result<ToolResult, ComputerError>` - Result of tool execution
    ///
    /// # Example
    /// ```no_run
    /// use serde_json::json;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let computer = Computer::new();
    /// let result = computer
    ///     .execute_tool(
    ///         "execute_command",
    ///         json!({
    ///             "command": "echo 'test'"
    ///         }),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        params: Value,
    ) -> Result<ToolResult, ComputerError> {
        info!("Executing tool '{}' with params: {}", tool_name, params);

        // Find the tool
        let tool = self.get_tool(tool_name).ok_or_else(|| {
            let msg = format!("Tool '{}' not found", tool_name);
            error!("{}", msg);
            ComputerError::ToolNotFound(msg)
        })?;

        // Validate parameters
        if let Err(validation_error) = tool.validate_params(&params) {
            let msg = format!("Parameter validation failed: {}", validation_error);
            error!("{}", msg);
            return Err(ComputerError::ValidationError(msg));
        }

        // Execute the tool if it implements ToolExecution
        if let Some(exec_tool) = tool.as_any().downcast_ref::<Box<dyn ToolExecution>>() {
            debug!("Executing tool '{}'", tool_name);
            match exec_tool.execute(params).await {
                Ok(result) => {
                    if result.success {
                        info!("Tool '{}' executed successfully", tool_name);
                    } else {
                        error!("Tool '{}' execution failed: {:?}", tool_name, result.error);
                    }
                    Ok(result)
                }
                Err(e) => {
                    let msg = format!("Tool execution failed: {}", e);
                    error!("{}", msg);
                    Err(ComputerError::ExecutionError(msg))
                }
            }
        } else {
            let msg = format!("Tool '{}' does not implement ToolExecution", tool_name);
            error!("{}", msg);
            Err(ComputerError::ExecutionError(msg))
        }
    }

    /// Get a tool by name.
    ///
    /// # Arguments
    /// * `name` - Name of the tool to retrieve
    pub fn get_tool(&self, name: &str) -> Option<&Box<dyn ToolInfo>> {
        self.tools.get_tool(name)
    }

    /// List all available tools with their descriptions.
    pub fn list_available_tools(&self) -> Vec<(&'static str, &'static str)> {
        self.tools.list_tools()
    }

    /// Check if a tool exists.
    ///
    /// # Arguments
    /// * `name` - Name of the tool to check
    pub fn has_tool(&self, name: &str) -> bool {
        self.get_tool(name).is_some()
    }

    /// Get tool description.
    ///
    /// # Arguments
    /// * `name` - Name of the tool
    pub fn get_tool_description(&self, name: &str) -> Option<&'static str> {
        self.get_tool(name).map(|tool| tool.description())
    }

    /// Get tool parameters.
    ///
    /// # Arguments
    /// * `name` - Name of the tool
    pub fn get_tool_parameters(&self, name: &str) -> Option<Vec<(&'static str, bool)>> {
        self.get_tool(name).map(|tool| tool.parameters())
    }

    /// Format tool execution result for LLM consumption.
    ///
    /// # Arguments
    /// * `tool_name` - Name of the tool that produced the result
    /// * `result` - Result to format
    pub fn format_result(&self, tool_name: &str, result: &ToolResult) -> Value {
        serde_json::json!({
            "tool": tool_name,
            "success": result.success,
            "output": result.output,
            "error": result.error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_execute_tool() {
        let computer = Computer::new();

        // Test successful execution
        let result = computer
            .execute_tool(
                "execute_command",
                json!({
                    "command": "echo 'test'"
                }),
            )
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("test"));

        // Test tool not found
        let err = computer
            .execute_tool("nonexistent_tool", json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, ComputerError::ToolNotFound(_)));

        // Test parameter validation
        let err = computer
            .execute_tool("execute_command", json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, ComputerError::ValidationError(_)));
    }

    #[test]
    fn test_tool_management() {
        let computer = Computer::new();

        // Test tool existence
        assert!(computer.has_tool("execute_command"));
        assert!(!computer.has_tool("nonexistent_tool"));

        // Test tool description
        assert!(computer.get_tool_description("execute_command").is_some());
        assert!(computer.get_tool_description("nonexistent_tool").is_none());

        // Test tool parameters
        let params = computer.get_tool_parameters("execute_command").unwrap();
        assert!(!params.is_empty());
    }

    #[test]
    fn test_result_formatting() {
        let computer = Computer::new();
        let result = ToolResult {
            success: true,
            output: "test output".to_string(),
            error: None,
        };

        let formatted = computer.format_result("test_tool", &result);
        assert_eq!(formatted["tool"], "test_tool");
        assert_eq!(formatted["success"], true);
        assert_eq!(formatted["output"], "test output");
        assert_eq!(formatted["error"], Value::Null);
    }
}
