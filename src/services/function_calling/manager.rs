use std::sync::Arc;
use serde_json::Value;
use log::{debug, error, info};
use crate::services::function_calling::types::{Tool, ToolResult};
use crate::services::function_calling::provider::adapter::{ProviderAdapter, ToolChoice};
use crate::services::function_calling::tool::registry::ToolRegistry;
use crate::services::function_calling::error::FunctionCallingError;

/// Manager for function calling
pub struct FunctionCallingManager {
    provider_adapter: Arc<dyn ProviderAdapter>,
    tool_registry: Arc<ToolRegistry>,
}

impl FunctionCallingManager {
    /// Creates a new function calling manager
    pub fn new(provider_adapter: Arc<dyn ProviderAdapter>, tool_registry: Arc<ToolRegistry>) -> Self {
        Self {
            provider_adapter,
            tool_registry,
        }
    }
    
    /// Prepares a request for the provider
    pub fn prepare_request(&self, request: &mut Value, tools: &[Tool], choice: ToolChoice) {
        debug!("Preparing request with {} tools", tools.len());
        self.provider_adapter.prepare_request(request, tools, choice);
    }
    
    /// Handles a response from the provider
    pub async fn handle_response(&self, response: &Value) -> Result<Vec<ToolResult>, FunctionCallingError> {
        debug!("Handling response from provider");
        let tool_calls = self.provider_adapter.parse_tool_calls(response).await?;
        
        if tool_calls.is_empty() {
            debug!("No tool calls found in response");
            return Ok(vec![]);
        }
        
        info!("Found {} tool calls in response", tool_calls.len());
        
        let mut results = Vec::new();
        for call in tool_calls {
            debug!("Executing tool: {}", call.name);
            let result = match self.tool_registry.execute(&call.name, call.arguments.clone()).await {
                Ok(result) => {
                    debug!("Tool execution successful");
                    result
                },
                Err(e) => {
                    error!("Tool execution failed: {}", e);
                    return Err(FunctionCallingError::ToolExecutionError(e));
                }
            };
            
            results.push(ToolResult {
                tool_call_id: call.id,
                result,
            });
        }
        
        Ok(results)
    }
    
    /// Returns the name of the provider
    pub fn provider_name(&self) -> &str {
        self.provider_adapter.name()
    }
    
    /// Returns true if the provider supports streaming
    pub fn supports_streaming(&self) -> bool {
        self.provider_adapter.supports_streaming()
    }
}
