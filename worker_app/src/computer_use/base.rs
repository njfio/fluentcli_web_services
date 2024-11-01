//! Core traits and types for computer use functionality.
//!
//! This module provides the foundational traits and types that define the tool execution
//! framework. It follows the design patterns from Anthropic's computer-use implementation
//! while providing Rust-specific enhancements.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;

/// Result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether the execution was successful.
    pub success: bool,
    /// Output from the tool execution.
    pub output: String,
    /// Optional error message if execution failed.
    pub error: Option<String>,
}

/// Core trait defining a tool's metadata and validation capabilities.
///
/// This trait must be implemented by all tools in the system. It provides
/// the basic information needed to identify and validate a tool.
pub trait ToolInfo: Send + Sync + Debug + Any {
    /// Get the unique name of the tool.
    fn name(&self) -> &'static str;

    /// Get a human-readable description of the tool.
    fn description(&self) -> &'static str;

    /// Get the list of parameters the tool accepts.
    ///
    /// Returns a vector of tuples containing:
    /// - Parameter name
    /// - Whether the parameter is required
    fn parameters(&self) -> Vec<(&'static str, bool)>;

    /// Validate the provided parameters against the tool's requirements.
    ///
    /// # Arguments
    /// * `params` - The parameters to validate
    ///
    /// # Returns
    /// * `Ok(())` if validation succeeds
    /// * `Err(String)` with error message if validation fails
    fn validate_params(&self, params: &Value) -> Result<(), String> {
        let required_params: Vec<&str> = self
            .parameters()
            .into_iter()
            .filter(|(_, required)| *required)
            .map(|(name, _)| name)
            .collect();

        for param in required_params {
            if !params.get(param).is_some() {
                return Err(format!("Missing required parameter: {}", param));
            }
        }
        Ok(())
    }

    /// Convert the tool to Any for dynamic dispatch.
    fn as_any(&self) -> &dyn Any;
}

/// Trait for tools that can be executed.
///
/// This trait extends `ToolInfo` to provide execution capabilities.
/// Tools implementing this trait can be executed by the computer system.
#[async_trait]
pub trait ToolExecution: ToolInfo {
    /// Execute the tool with the provided parameters.
    ///
    /// # Arguments
    /// * `params` - The parameters for tool execution
    ///
    /// # Returns
    /// * `Result<ToolResult, Box<dyn Error + Send + Sync>>` - The result of execution
    async fn execute(&self, params: Value) -> Result<ToolResult, Box<dyn Error + Send + Sync>>;
}

/// Collection managing available tools.
///
/// This struct provides a central registry for all available tools
/// and methods to access and manage them.
#[derive(Debug)]
pub struct ToolCollection {
    tools: Vec<Box<dyn ToolInfo>>,
}

impl ToolCollection {
    /// Create a new empty tool collection.
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    /// Add a tool to the collection.
    ///
    /// # Arguments
    /// * `tool` - The tool to add
    pub fn add_tool<T: ToolInfo + 'static>(&mut self, tool: T) {
        self.tools.push(Box::new(tool));
    }

    /// Get a tool by name.
    ///
    /// # Arguments
    /// * `name` - The name of the tool to retrieve
    ///
    /// # Returns
    /// * `Option<&Box<dyn ToolInfo>>` - The tool if found
    pub fn get_tool(&self, name: &str) -> Option<&Box<dyn ToolInfo>> {
        self.tools.iter().find(|t| t.name() == name)
    }

    /// List all available tools with their descriptions.
    ///
    /// # Returns
    /// * `Vec<(&'static str, &'static str)>` - Vector of (name, description) pairs
    pub fn list_tools(&self) -> Vec<(&'static str, &'static str)> {
        self.tools
            .iter()
            .map(|t| (t.name(), t.description()))
            .collect()
    }
}
