use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::RwLock;
use crate::services::function_calling::types::Tool;
use crate::services::function_calling::tool::executor::{ToolExecutor, DynToolExecutor};
use crate::services::function_calling::tool::error::ToolError;

/// Registry for tools
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Box<dyn DynToolExecutor + Send + Sync>>>>,
}

impl ToolRegistry {
    /// Creates a new tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a tool with the registry
    pub async fn register<T: ToolExecutor + 'static>(&self, executor: T) {
        let name = executor.name().to_string();
        let mut tools = self.tools.write().await;
        tools.insert(name, Box::new(executor));
    }

    /// Registers a remote tool with the registry
    pub async fn register_remote<T: DynToolExecutor + Send + Sync + 'static>(&self, name: &str, executor: T) {
        let mut tools = self.tools.write().await;
        tools.insert(name.to_string(), Box::new(executor));
    }

    /// Executes a tool with the given name and arguments
    pub async fn execute(&self, name: &str, args: Value) -> Result<Value, ToolError> {
        let tools = self.tools.read().await;
        match tools.get(name) {
            Some(executor) => {
                // Validate arguments before execution
                executor.validate_args(&args)?;

                // Execute the tool
                executor.execute(args).await
            },
            None => Err(ToolError::ToolNotFound(name.to_string())),
        }
    }

    /// Returns a tool with the given name
    pub async fn get_tool(&self, name: &str) -> Option<Tool> {
        let tools = self.tools.read().await;
        tools.get(name).map(|executor| {
            Tool::new(
                executor.name(),
                executor.description(),
                vec![], // Parameters would need to be defined elsewhere
            )
        })
    }

    /// Returns a list of all tools in the registry
    pub async fn list_tools(&self) -> Vec<Tool> {
        let tools = self.tools.read().await;
        tools.iter().map(|(_, executor)| {
            Tool::new(
                executor.name(),
                executor.description(),
                vec![], // Parameters would need to be defined elsewhere
            )
        }).collect()
    }

    /// Returns true if the registry contains a tool with the given name
    pub async fn contains(&self, name: &str) -> bool {
        let tools = self.tools.read().await;
        tools.contains_key(name)
    }

    /// Returns the number of tools in the registry
    pub async fn len(&self) -> usize {
        let tools = self.tools.read().await;
        tools.len()
    }

    /// Returns true if the registry is empty
    pub async fn is_empty(&self) -> bool {
        let tools = self.tools.read().await;
        tools.is_empty()
    }
}
