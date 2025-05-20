# Rust Implementation Guide for Generic Function Calling

This guide provides detailed information on implementing the generic function calling system in Rust, including code examples and best practices.

## Project Structure

```
src/
├── function_calling/
│   ├── mod.rs                 # Module exports
│   ├── types.rs               # Core data structures
│   ├── provider/
│   │   ├── mod.rs             # Provider module exports
│   │   ├── adapter.rs         # Provider adapter trait
│   │   ├── openai.rs          # OpenAI adapter
│   │   ├── anthropic.rs       # Anthropic adapter
│   │   ├── gemini.rs          # Gemini adapter
│   │   ├── grok.rs            # Grok adapter
│   │   └── openrouter.rs      # OpenRouter adapter
│   ├── tool/
│   │   ├── mod.rs             # Tool module exports
│   │   ├── registry.rs        # Tool registry
│   │   ├── executor.rs        # Tool executor trait
│   │   └── error.rs           # Tool error types
│   ├── manager.rs             # Function calling manager
│   └── error.rs               # Error types
├── mcp/
│   ├── mod.rs                 # MCP module exports
│   ├── client.rs              # MCP client
│   └── server.rs              # MCP server
├── agent/
│   ├── mod.rs                 # Agent module exports
│   ├── definition.rs          # Agent definition
│   ├── manager.rs             # Agent manager
│   └── patterns.rs            # Agent patterns
└── tools/
    ├── mod.rs                 # Tools module exports
    ├── info_retrieval/        # Information retrieval tools
    ├── data_processing/       # Data processing tools
    └── action/                # Action tools
```

## Core Data Structures

### Tool Definition

```rust
// src/function_calling/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameter_type: ParameterType,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParameterType {
    #[serde(rename = "string")]
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enum_values: Option<Vec<String>>,
    },
    #[serde(rename = "number")]
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
    },
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object {
        properties: Vec<ToolParameter>,
    },
    #[serde(rename = "array")]
    Array {
        items: Box<ParameterType>,
    },
}

impl Tool {
    pub fn new(name: &str, description: &str, parameters: Vec<ToolParameter>) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        }
    }
    
    pub fn to_json_schema(&self) -> serde_json::Value {
        let properties = self.parameters.iter().map(|param| {
            (param.name.clone(), param.to_json_schema())
        }).collect::<HashMap<_, _>>();
        
        let required = self.parameters.iter()
            .filter(|p| p.required)
            .map(|p| p.name.clone())
            .collect::<Vec<_>>();
            
        serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": required,
        })
    }
}

impl ToolParameter {
    pub fn new(name: &str, parameter_type: ParameterType, required: bool) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            parameter_type,
            required,
        }
    }
    
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    
    fn to_json_schema(&self) -> serde_json::Value {
        match &self.parameter_type {
            ParameterType::String { format, enum_values } => {
                let mut schema = serde_json::json!({
                    "type": "string",
                });
                
                if let Some(format) = format {
                    schema["format"] = serde_json::Value::String(format.clone());
                }
                
                if let Some(enum_values) = enum_values {
                    schema["enum"] = serde_json::json!(enum_values);
                }
                
                schema
            },
            ParameterType::Number { minimum, maximum } => {
                let mut schema = serde_json::json!({
                    "type": "number",
                });
                
                if let Some(minimum) = minimum {
                    schema["minimum"] = serde_json::json!(minimum);
                }
                
                if let Some(maximum) = maximum {
                    schema["maximum"] = serde_json::json!(maximum);
                }
                
                schema
            },
            ParameterType::Boolean => {
                serde_json::json!({
                    "type": "boolean",
                })
            },
            ParameterType::Object { properties } => {
                let obj_properties = properties.iter().map(|param| {
                    (param.name.clone(), param.to_json_schema())
                }).collect::<HashMap<_, _>>();
                
                let required = properties.iter()
                    .filter(|p| p.required)
                    .map(|p| p.name.clone())
                    .collect::<Vec<_>>();
                    
                serde_json::json!({
                    "type": "object",
                    "properties": obj_properties,
                    "required": required,
                })
            },
            ParameterType::Array { items } => {
                let item_schema = match items.as_ref() {
                    param_type => {
                        let dummy_param = ToolParameter {
                            name: "item".to_string(),
                            description: None,
                            parameter_type: param_type.clone(),
                            required: false,
                        };
                        dummy_param.to_json_schema()
                    }
                };
                
                serde_json::json!({
                    "type": "array",
                    "items": item_schema,
                })
            },
        }
    }
}
```

### Tool Execution

```rust
// src/function_calling/tool/executor.rs

use async_trait::async_trait;
use serde_json::Value;
use crate::function_calling::tool::error::ToolError;

#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, args: Value) -> Result<Value, ToolError>;
    
    fn name(&self) -> &str;
    
    fn description(&self) -> &str;
}
```

### Tool Registry

```rust
// src/function_calling/tool/registry.rs

use std::collections::HashMap;
use serde_json::Value;
use crate::function_calling::types::Tool;
use crate::function_calling::tool::executor::ToolExecutor;
use crate::function_calling::tool::error::ToolError;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
    
    pub fn register<T: ToolExecutor + 'static>(&mut self, name: &str, executor: T) {
        self.tools.insert(name.to_string(), Box::new(executor));
    }
    
    pub fn register_remote(&mut self, name: &str, executor: impl ToolExecutor + 'static) {
        self.tools.insert(name.to_string(), Box::new(executor));
    }
    
    pub async fn execute(&self, name: &str, args: Value) -> Result<Value, ToolError> {
        match self.tools.get(name) {
            Some(executor) => executor.execute(args).await,
            None => Err(ToolError::ToolNotFound(name.to_string())),
        }
    }
    
    pub fn get_tool(&self, name: &str) -> Option<&Box<dyn ToolExecutor>> {
        self.tools.get(name)
    }
    
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools.iter().map(|(name, executor)| {
            Tool::new(
                name,
                executor.description(),
                vec![], // Parameters would need to be defined elsewhere
            )
        }).collect()
    }
}
```

## Provider Adapters

### Provider Adapter Trait

```rust
// src/function_calling/provider/adapter.rs

use async_trait::async_trait;
use serde_json::Value;
use crate::function_calling::types::{Tool, ToolCall};
use crate::function_calling::error::FunctionCallingError;

pub enum ToolChoice {
    Auto,
    Required,
    None,
    Specific(String),
}

#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    fn format_tools(&self, tools: &[Tool]) -> Value;
    
    fn format_tool_choice(&self, choice: ToolChoice) -> Value;
    
    async fn parse_tool_calls(&self, response: &Value) -> Result<Vec<ToolCall>, FunctionCallingError>;
    
    fn prepare_request(&self, request: &mut Value, tools: &[Tool], choice: ToolChoice) {
        let formatted_tools = self.format_tools(tools);
        let formatted_choice = self.format_tool_choice(choice);
        
        // Add tools and tool_choice to request based on provider format
        // Implementation depends on the provider
    }
    
    fn supports_streaming(&self) -> bool {
        false
    }
    
    fn name(&self) -> &'static str;
}
```

### OpenAI Adapter

```rust
// src/function_calling/provider/openai.rs

use async_trait::async_trait;
use serde_json::{json, Value};
use crate::function_calling::types::{Tool, ToolCall};
use crate::function_calling::provider::adapter::{ProviderAdapter, ToolChoice};
use crate::function_calling::error::FunctionCallingError;

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
```

## Function Calling Manager

```rust
// src/function_calling/manager.rs

use serde_json::Value;
use crate::function_calling::types::{Tool, ToolCall, ToolResult};
use crate::function_calling::provider::adapter::{ProviderAdapter, ToolChoice};
use crate::function_calling::tool::registry::ToolRegistry;
use crate::function_calling::error::FunctionCallingError;

pub struct FunctionCallingManager {
    provider_adapter: Box<dyn ProviderAdapter>,
    tool_registry: ToolRegistry,
}

impl FunctionCallingManager {
    pub fn new(provider_adapter: Box<dyn ProviderAdapter>, tool_registry: ToolRegistry) -> Self {
        Self {
            provider_adapter,
            tool_registry,
        }
    }
    
    pub fn prepare_request(&self, request: &mut Value, tools: &[Tool], choice: ToolChoice) {
        self.provider_adapter.prepare_request(request, tools, choice);
    }
    
    pub async fn handle_response(&self, response: &Value) -> Result<Vec<ToolResult>, FunctionCallingError> {
        let tool_calls = self.provider_adapter.parse_tool_calls(response).await?;
        
        let mut results = Vec::new();
        for call in tool_calls {
            let result = self.tool_registry.execute(&call.name, call.arguments.clone()).await
                .map_err(|e| FunctionCallingError::ToolExecutionError(e))?;
                
            results.push(ToolResult {
                tool_call_id: call.id,
                result,
            });
        }
        
        Ok(results)
    }
    
    pub fn provider_name(&self) -> &str {
        self.provider_adapter.name()
    }
    
    pub fn supports_streaming(&self) -> bool {
        self.provider_adapter.supports_streaming()
    }
}
```

## MCP Client Implementation

```rust
// src/mcp/client.rs

use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use crate::function_calling::tool::executor::ToolExecutor;
use crate::function_calling::tool::error::ToolError;

pub struct MCPClient {
    client: Client,
    base_url: String,
    auth_token: String,
    tool_name: String,
    tool_description: String,
}

impl MCPClient {
    pub fn new(base_url: &str, auth_token: &str, tool_name: &str, tool_description: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            auth_token: auth_token.to_string(),
            tool_name: tool_name.to_string(),
            tool_description: tool_description.to_string(),
        }
    }
    
    async fn call_remote_tool(&self, args: Value) -> Result<Value, ToolError> {
        let response = self.client
            .post(&format!("{}/tools/{}", self.base_url, self.tool_name))
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(&args)
            .send()
            .await
            .map_err(|e| ToolError::NetworkError(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(ToolError::RemoteToolError(format!(
                "Remote tool execution failed with status: {}",
                response.status()
            )));
        }
        
        response.json::<Value>()
            .await
            .map_err(|e| ToolError::DeserializationError(e.to_string()))
    }
    
    pub async fn discover_tools(&self) -> Result<Vec<Tool>, ToolError> {
        // Implementation for tool discovery
        // ...
    }
}

#[async_trait]
impl ToolExecutor for MCPClient {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        self.call_remote_tool(args).await
    }
    
    fn name(&self) -> &str {
        &self.tool_name
    }
    
    fn description(&self) -> &str {
        &self.tool_description
    }
}
```

## Agent Implementation

```rust
// src/agent/manager.rs

use serde_json::{json, Value};
use crate::function_calling::manager::FunctionCallingManager;
use crate::function_calling::types::{Tool, Message};
use crate::function_calling::provider::adapter::ToolChoice;
use crate::agent::definition::AgentDefinition;
use crate::agent::error::AgentError;

pub struct AgentManager {
    definition: AgentDefinition,
    function_calling_manager: FunctionCallingManager,
    conversation_history: Vec<Message>,
    llm_client: Box<dyn LLMClient>,
}

impl AgentManager {
    pub fn new(
        definition: AgentDefinition,
        function_calling_manager: FunctionCallingManager,
        llm_client: Box<dyn LLMClient>,
    ) -> Self {
        Self {
            definition,
            function_calling_manager,
            conversation_history: Vec::new(),
            llm_client,
        }
    }
    
    pub async fn process_message(&mut self, user_message: &str) -> Result<String, AgentError> {
        // Add user message to conversation history
        self.conversation_history.push(Message::user(user_message));
        
        // Prepare LLM request with tools
        let mut request = self.prepare_request();
        self.function_calling_manager.prepare_request(
            &mut request,
            &self.definition.tools,
            ToolChoice::Auto,
        );
        
        // Send request to LLM provider
        let response = self.llm_client.send_request(request).await?;
        
        // Process the response
        let (assistant_message, tool_calls) = self.process_response(&response).await?;
        
        // If there were tool calls, execute them and continue the conversation
        if !tool_calls.is_empty() {
            let tool_results = self.function_calling_manager.handle_response(&response).await?;
            
            // Add tool results to conversation history
            for result in &tool_results {
                self.conversation_history.push(Message::tool_result(
                    &result.tool_call_id,
                    &result.result,
                ));
            }
            
            // Continue conversation with tool results
            let mut follow_up_request = self.prepare_follow_up_request();
            self.function_calling_manager.prepare_request(
                &mut follow_up_request,
                &self.definition.tools,
                ToolChoice::Auto,
            );
            
            let follow_up_response = self.llm_client.send_request(follow_up_request).await?;
            
            let (final_message, _) = self.process_response(&follow_up_response).await?;
            self.conversation_history.push(Message::assistant(&final_message));
            
            Ok(final_message)
        } else {
            // No tool calls, just return the assistant message
            self.conversation_history.push(Message::assistant(&assistant_message));
            Ok(assistant_message)
        }
    }
    
    // Helper methods
    fn prepare_request(&self) -> Value {
        // Create request with system prompt and conversation history
        let system_prompt = self.definition.generate_system_prompt();
        
        let messages = self.conversation_history.iter().map(|msg| {
            match msg {
                Message::User { content } => json!({
                    "role": "user",
                    "content": content,
                }),
                Message::Assistant { content } => json!({
                    "role": "assistant",
                    "content": content,
                }),
                Message::System { content } => json!({
                    "role": "system",
                    "content": content,
                }),
                Message::ToolResult { tool_call_id, content } => json!({
                    "role": "tool",
                    "tool_call_id": tool_call_id,
                    "content": content,
                }),
            }
        }).collect::<Vec<_>>();
        
        json!({
            "model": self.definition.model,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt,
                },
                ...messages,
            ],
        })
    }
    
    // Other helper methods
    // ...
}
```

## Example Usage

```rust
// Example usage of the generic function calling system

use serde_json::json;
use crate::function_calling::types::{Tool, ToolParameter, ParameterType};
use crate::function_calling::provider::openai::OpenAIAdapter;
use crate::function_calling::provider::adapter::ToolChoice;
use crate::function_calling::tool::registry::ToolRegistry;
use crate::function_calling::manager::FunctionCallingManager;
use crate::tools::weather::WeatherTool;
use crate::tools::search::WebSearchTool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create tools
    let weather_tool = Tool::new(
        "get_weather",
        "Get current weather for a location",
        vec![
            ToolParameter::new(
                "location",
                ParameterType::String { format: None, enum_values: None },
                true,
            ).with_description("City and country, e.g., 'Paris, France'"),
            ToolParameter::new(
                "units",
                ParameterType::String {
                    format: None,
                    enum_values: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
                },
                false,
            ).with_description("Temperature units: 'celsius' or 'fahrenheit'"),
        ],
    );
    
    let search_tool = Tool::new(
        "search_web",
        "Search the web for information",
        vec![
            ToolParameter::new(
                "query",
                ParameterType::String { format: None, enum_values: None },
                true,
            ).with_description("Search query"),
            ToolParameter::new(
                "num_results",
                ParameterType::Number { minimum: Some(1.0), maximum: Some(10.0) },
                false,
            ).with_description("Number of results to return (1-10)"),
        ],
    );
    
    // 2. Create tool registry and register tools
    let mut tool_registry = ToolRegistry::new();
    tool_registry.register("get_weather", WeatherTool::new("weather_api_key"));
    tool_registry.register("search_web", WebSearchTool::new("search_api_key"));
    
    // 3. Create function calling manager
    let provider_adapter = Box::new(OpenAIAdapter);
    let function_calling_manager = FunctionCallingManager::new(provider_adapter, tool_registry);
    
    // 4. Prepare request
    let mut request = json!({
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "What's the weather like in Paris today? Also, find me some information about the Eiffel Tower."
            }
        ],
    });
    
    function_calling_manager.prepare_request(
        &mut request,
        &[weather_tool, search_tool],
        ToolChoice::Auto,
    );
    
    // 5. Send request to OpenAI (simplified)
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", "Bearer YOUR_API_KEY")
        .json(&request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    // 6. Handle tool calls
    let tool_results = function_calling_manager.handle_response(&response).await?;
    
    // 7. Continue conversation with tool results
    // ...
    
    Ok(())
}
```

## Next Steps

1. Implement error handling and logging
2. Add streaming support
3. Create additional provider adapters
4. Develop MCP integration
5. Build agent framework
6. Create tool library
