use async_trait::async_trait;
use serde_json::Value;
use crate::services::function_calling::types::{Tool, ToolCall};
use crate::services::function_calling::error::FunctionCallingError;

/// Enum for specifying how tools should be chosen
#[derive(Debug, Clone)]
pub enum ToolChoice {
    /// The model can choose which tool to use (if any)
    Auto,
    /// The model must use a tool
    Required,
    /// The model must not use any tools
    None,
    /// The model must use a specific tool
    Specific(String),
}

/// Trait for adapting to different LLM providers
#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    /// Formats tools for the provider
    fn format_tools(&self, tools: &[Tool]) -> Value;
    
    /// Formats tool choice for the provider
    fn format_tool_choice(&self, choice: ToolChoice) -> Value;
    
    /// Parses tool calls from the provider's response
    async fn parse_tool_calls(&self, response: &Value) -> Result<Vec<ToolCall>, FunctionCallingError>;
    
    /// Prepares a request for the provider
    fn prepare_request(&self, request: &mut Value, tools: &[Tool], choice: ToolChoice) {
        let formatted_tools = self.format_tools(tools);
        let formatted_choice = self.format_tool_choice(choice);
        
        // Add tools and tool_choice to request based on provider format
        // Implementation depends on the provider
        // Default implementation does nothing
    }
    
    /// Returns true if the provider supports streaming
    fn supports_streaming(&self) -> bool {
        false
    }
    
    /// Returns the name of the provider
    fn name(&self) -> &'static str;
}
