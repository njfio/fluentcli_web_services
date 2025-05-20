# Generic Function Calling System

This module provides a unified interface for function calling across multiple LLM providers, including OpenAI, Anthropic Claude, Google Gemini, Grok, and OpenRouter.

## Overview

The generic function calling system abstracts away provider-specific implementation details while maintaining the full capabilities of each provider. It allows you to define tools once and use them with any supported LLM provider.

## Core Components

### Tool Definition

Tools are defined using the `Tool` struct, which includes a name, description, and parameters:

```rust
let weather_tool = Tool::new(
    "get_weather",
    "Get the current weather for a location",
    vec![
        ToolParameter::new(
            "location",
            ParameterType::String {
                format: None,
                enum_values: None,
            },
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
```

### Tool Executor

Tool executors implement the `ToolExecutor` trait, which defines how a tool is executed:

```rust
#[async_trait]
impl ToolExecutor for WeatherTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolError> {
        // Extract and validate parameters
        let location = args["location"].as_str()
            .ok_or_else(|| ToolError::InvalidArgument("location must be a string".to_string()))?;
            
        // Execute the tool logic
        // ...
        
        // Return the result
        Ok(result)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}
```

### Provider Adapters

Provider adapters implement the `ProviderAdapter` trait, which defines how to format tools and parse responses for a specific LLM provider:

```rust
#[async_trait]
impl ProviderAdapter for OpenAIAdapter {
    fn format_tools(&self, tools: &[Tool]) -> Value {
        // Format tools for OpenAI
        // ...
    }
    
    async fn parse_tool_calls(&self, response: &Value) -> Result<Vec<ToolCall>, FunctionCallingError> {
        // Parse tool calls from OpenAI response
        // ...
    }
    
    // ...
}
```

### Function Calling Manager

The `FunctionCallingManager` is the main interface for applications to interact with the function calling system:

```rust
let function_calling_manager = FunctionCallingManager::new(provider_adapter, tool_registry);

// Prepare a request with tools
function_calling_manager.prepare_request(&mut request, &[weather_tool], ToolChoice::Auto);

// Handle a response with tool calls
let tool_results = function_calling_manager.handle_response(&response).await?;
```

## Usage Example

Here's a complete example of how to use the function calling system:

```rust
// 1. Create the tool registry
let tool_registry = Arc::new(ToolRegistry::new());

// 2. Register tools
let weather_tool = WeatherTool::new();
tool_registry.register(weather_tool).await;

// 3. Create the provider adapter
let provider_adapter = Arc::new(OpenAIAdapter);

// 4. Create the function calling manager
let function_calling_manager = FunctionCallingManager::new(provider_adapter, tool_registry);

// 5. Define the tools for the request
let weather_tool_def = Tool::new(
    "get_weather",
    "Get the current weather for a location",
    vec![
        // Parameters...
    ],
);

// 6. Prepare the request
let mut request = json!({
    "model": "gpt-4",
    "messages": [
        // Messages...
    ],
});

function_calling_manager.prepare_request(&mut request, &[weather_tool_def], ToolChoice::Auto);

// 7. Send the request to the LLM provider
let response = client.post("https://api.openai.com/v1/chat/completions")
    .json(&request)
    .send()
    .await?
    .json::<Value>()
    .await?;

// 8. Handle the response
let tool_results = function_calling_manager.handle_response(&response).await?;

// 9. Continue the conversation with the tool results
// ...
```

## Supported Providers

- **OpenAI**: Full support for function calling with the `tools` and `tool_choice` parameters.
- **Anthropic Claude**: Support for tool use via the system prompt and content blocks.
- **Google Gemini**: (Coming soon) Support for function calling with the `tools` and `tool_config` parameters.
- **Grok**: (Coming soon) Support for function calling with the `tools` and `tool_choice` parameters.
- **OpenRouter**: (Coming soon) Support for function calling with the `tools` and `tool_choice` parameters.

## Adding New Tools

To add a new tool:

1. Create a struct that implements the `ToolExecutor` trait.
2. Register the tool with the `ToolRegistry`.
3. Define the tool parameters using the `Tool` struct.

## Adding New Providers

To add a new provider:

1. Create a struct that implements the `ProviderAdapter` trait.
2. Implement the required methods for formatting tools and parsing responses.
3. Update the `get_provider_adapter` function to return the new adapter for the appropriate provider type.

## Error Handling

The system includes comprehensive error handling for various failure modes:

- `FunctionCallingError`: Errors related to the function calling system.
- `ToolError`: Errors related to tool execution.

## Testing

The system includes unit tests for all components. See `src/tests/function_calling_tests.rs` for examples.
