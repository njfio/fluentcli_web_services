use async_trait::async_trait;
use serde_json::{json, Value};
use crate::services::function_calling::types::{Tool, ToolCall};
use crate::services::function_calling::provider::adapter::{ProviderAdapter, ToolChoice};
use crate::services::function_calling::error::FunctionCallingError;

/// Adapter for OpenAI's function calling API
pub struct OpenAIAdapter;

#[async_trait]
impl ProviderAdapter for OpenAIAdapter {
    fn format_tools(&self, tools: &[Tool]) -> Value {
        json!(tools.iter().map(|tool| {
            json!({
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": tool.to_json_schema(),
                }
            })
        }).collect::<Vec<_>>())
    }
    
    fn format_tool_choice(&self, choice: ToolChoice) -> Value {
        match choice {
            ToolChoice::Auto => json!("auto"),
            ToolChoice::Required => json!("required"),
            ToolChoice::None => json!("none"),
            ToolChoice::Specific(name) => json!({
                "type": "function",
                "function": {
                    "name": name
                }
            }),
        }
    }
    
    async fn parse_tool_calls(&self, response: &Value) -> Result<Vec<ToolCall>, FunctionCallingError> {
        let choices = response["choices"].as_array()
            .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                "Missing 'choices' array in response".to_string()
            ))?;
            
        if choices.is_empty() {
            return Ok(vec![]);
        }
        
        let message = &choices[0]["message"];
        let tool_calls = message["tool_calls"].as_array();
        
        match tool_calls {
            Some(calls) if !calls.is_empty() => {
                let mut result = Vec::new();
                
                for call in calls {
                    let id = call["id"].as_str()
                        .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                            "Missing 'id' in tool call".to_string()
                        ))?;
                        
                    let function = &call["function"];
                    let name = function["name"].as_str()
                        .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                            "Missing 'name' in function call".to_string()
                        ))?;
                        
                    let arguments_str = function["arguments"].as_str()
                        .ok_or_else(|| FunctionCallingError::ResponseParsingError(
                            "Missing 'arguments' in function call".to_string()
                        ))?;
                        
                    let arguments: Value = serde_json::from_str(arguments_str)
                        .map_err(|e| FunctionCallingError::ResponseParsingError(
                            format!("Failed to parse arguments: {}", e)
                        ))?;
                        
                    result.push(ToolCall {
                        id: id.to_string(),
                        name: name.to_string(),
                        arguments,
                    });
                }
                
                Ok(result)
            },
            _ => Ok(vec![]),
        }
    }
    
    fn prepare_request(&self, request: &mut Value, tools: &[Tool], choice: ToolChoice) {
        let formatted_tools = self.format_tools(tools);
        let formatted_choice = self.format_tool_choice(choice);
        
        request["tools"] = formatted_tools;
        request["tool_choice"] = formatted_choice;
    }
    
    fn supports_streaming(&self) -> bool {
        true
    }
    
    fn name(&self) -> &'static str {
        "openai"
    }
}
