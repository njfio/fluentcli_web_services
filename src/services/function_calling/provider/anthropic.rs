use async_trait::async_trait;
use serde_json::{json, Value};
use crate::services::function_calling::types::{Tool, ToolCall};
use crate::services::function_calling::provider::adapter::{ProviderAdapter, ToolChoice};
use crate::services::function_calling::error::FunctionCallingError;

/// Adapter for Anthropic Claude's tool use API
pub struct AnthropicAdapter;

#[async_trait]
impl ProviderAdapter for AnthropicAdapter {
    fn format_tools(&self, tools: &[Tool]) -> Value {
        json!(tools.iter().map(|tool| {
            json!({
                "name": tool.name,
                "description": tool.description,
                "input_schema": tool.to_json_schema(),
            })
        }).collect::<Vec<_>>())
    }
    
    fn format_tool_choice(&self, choice: ToolChoice) -> Value {
        match choice {
            ToolChoice::Auto => json!(null),
            ToolChoice::Required => json!(true),
            ToolChoice::None => json!(false),
            ToolChoice::Specific(name) => json!({
                "name": name
            }),
        }
    }
    
    async fn parse_tool_calls(&self, response: &Value) -> Result<Vec<ToolCall>, FunctionCallingError> {
        let content = response["content"].as_array()
            .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                "Missing 'content' array in response".to_string()
            ))?;
            
        let mut result = Vec::new();
        
        for item in content {
            if item["type"].as_str() == Some("tool_use") {
                let id = item["id"].as_str()
                    .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                        "Missing 'id' in tool_use".to_string()
                    ))?;
                    
                let name = item["name"].as_str()
                    .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                        "Missing 'name' in tool_use".to_string()
                    ))?;
                    
                let input = &item["input"];
                
                result.push(ToolCall {
                    id: id.to_string(),
                    name: name.to_string(),
                    arguments: input.clone(),
                });
            }
        }
        
        Ok(result)
    }
    
    fn prepare_request(&self, request: &mut Value, tools: &[Tool], choice: ToolChoice) {
        let formatted_tools = self.format_tools(tools);
        
        // Claude uses a different approach - tools are defined in the request
        request["tools"] = formatted_tools;
        
        // For Claude, tool choice is controlled via the system prompt
        if !request.as_object().unwrap().contains_key("system") {
            request["system"] = json!("");
        }
        
        let system_prompt = match choice {
            ToolChoice::Required => "You must use a tool to answer this query.",
            ToolChoice::None => "Do not use any tools to answer this query.",
            _ => "You have access to tools. Use them when appropriate.",
        };
        
        // Append to existing system prompt or create new one
        if let Some(existing) = request["system"].as_str() {
            request["system"] = json!(format!("{}\n\n{}", existing, system_prompt));
        } else {
            request["system"] = json!(system_prompt);
        }
    }
    
    fn supports_streaming(&self) -> bool {
        true
    }
    
    fn name(&self) -> &'static str {
        "anthropic"
    }
}
