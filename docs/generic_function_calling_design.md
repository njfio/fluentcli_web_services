# Generic Function Calling System Design

This document outlines the architecture and implementation details for a generic function calling system that works across multiple LLM providers.

## System Overview

The generic function calling system will provide a unified interface for defining tools/functions and handling their invocation across different LLM providers. The system will abstract away provider-specific implementation details while maintaining the full capabilities of each provider.

```
┌─────────────────┐     ┌───────────────────┐     ┌─────────────────┐
│                 │     │                   │     │                 │
│  Application    │────▶│  Generic Function │────▶│  LLM Provider   │
│                 │     │  Calling System   │     │  (OpenAI, etc.) │
│                 │◀────│                   │◀────│                 │
└─────────────────┘     └───────────────────┘     └─────────────────┘
```

## Core Components

### 1. Tool Definition Interface

A unified schema for defining tools/functions that can be translated to provider-specific formats:

```rust
pub struct ToolParameter {
    pub name: String,
    pub description: Option<String>,
    pub parameter_type: ParameterType,
    pub required: bool,
}

pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object(Vec<ToolParameter>),
    Array(Box<ParameterType>),
}

pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}
```

### 2. Provider Adapters

Adapters for each LLM provider that translate between the generic tool definition and provider-specific formats:

```rust
pub trait ProviderAdapter {
    fn format_tools(&self, tools: Vec<Tool>) -> Value;
    fn parse_tool_calls(&self, response: Value) -> Vec<ToolCall>;
    fn format_tool_choice(&self, choice: ToolChoice) -> Value;
}

pub struct OpenAIAdapter;
pub struct AnthropicAdapter;
pub struct GeminiAdapter;
pub struct GrokAdapter;
pub struct OpenRouterAdapter;
```

### 3. Tool Execution Framework

A system for registering, invoking, and handling tool executions:

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

pub trait ToolExecutor: Send + Sync {
    fn execute(&self, args: Value) -> Result<Value, ToolError>;
}

impl ToolRegistry {
    pub fn register<T: ToolExecutor + 'static>(&mut self, name: &str, executor: T) {
        self.tools.insert(name.to_string(), Box::new(executor));
    }
    
    pub fn execute(&self, name: &str, args: Value) -> Result<Value, ToolError> {
        match self.tools.get(name) {
            Some(executor) => executor.execute(args),
            None => Err(ToolError::ToolNotFound(name.to_string())),
        }
    }
}
```

### 4. Function Calling Manager

The main interface for applications to interact with the function calling system:

```rust
pub struct FunctionCallingManager {
    provider_adapter: Box<dyn ProviderAdapter>,
    tool_registry: ToolRegistry,
}

impl FunctionCallingManager {
    pub fn new(provider: LLMProvider, tool_registry: ToolRegistry) -> Self {
        let adapter: Box<dyn ProviderAdapter> = match provider {
            LLMProvider::OpenAI => Box::new(OpenAIAdapter),
            LLMProvider::Anthropic => Box::new(AnthropicAdapter),
            // ... other providers
        };
        
        Self {
            provider_adapter: adapter,
            tool_registry,
        }
    }
    
    pub fn prepare_request(&self, request: &mut Value, tools: Vec<Tool>, choice: ToolChoice) {
        let formatted_tools = self.provider_adapter.format_tools(tools);
        let formatted_choice = self.provider_adapter.format_tool_choice(choice);
        
        // Add tools and tool_choice to request based on provider format
        // ...
    }
    
    pub async fn handle_response(&self, response: Value) -> Result<Value, ToolError> {
        let tool_calls = self.provider_adapter.parse_tool_calls(response);
        
        let mut results = Vec::new();
        for call in tool_calls {
            let result = self.tool_registry.execute(&call.name, call.arguments)?;
            results.push((call.id, result));
        }
        
        Ok(json!({ "tool_results": results }))
    }
}
```

## Implementation Strategy

### Phase 1: Core Infrastructure

1. Implement the generic tool definition interface
2. Create provider adapters for OpenAI and one other provider
3. Develop the tool registry and execution framework
4. Build the function calling manager

### Phase 2: Provider Coverage

1. Implement remaining provider adapters
2. Add provider-specific optimizations
3. Create comprehensive test suite for each provider

### Phase 3: Advanced Features

1. Add streaming support for tool calls
2. Implement parallel tool execution
3. Create observability and logging system
4. Add caching layer for repeated tool calls

## Provider-Specific Considerations

### OpenAI, Grok, OpenRouter

These providers share similar schemas and can largely use the same adapter with minor modifications:

```rust
impl ProviderAdapter for OpenAIAdapter {
    fn format_tools(&self, tools: Vec<Tool>) -> Value {
        // Convert tools to OpenAI format
        json!([
            tools.iter().map(|tool| {
                json!({
                    "type": "function",
                    "function": {
                        "name": tool.name,
                        "description": tool.description,
                        "parameters": {
                            "type": "object",
                            "properties": format_parameters(&tool.parameters),
                            "required": tool.parameters.iter()
                                .filter(|p| p.required)
                                .map(|p| p.name.clone())
                                .collect::<Vec<_>>()
                        }
                    }
                })
            }).collect::<Vec<_>>()
        ])
    }
    
    // ... other methods
}
```

### Anthropic Claude

Requires special handling for system prompts:

```rust
impl ProviderAdapter for AnthropicAdapter {
    fn format_tools(&self, tools: Vec<Tool>) -> Value {
        // Convert tools to Anthropic system prompt format
        let tools_text = tools.iter().map(|tool| {
            let params = tool.parameters.iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}: {}", tool.name, params)
        }).collect::<Vec<_>>().join("\n");
        
        json!({
            "system": format!("You have access to these tools:\n<tools>\n{}\n</tools>\nReturn tool calls in JSON format.", tools_text)
        })
    }
    
    // ... other methods
}
```

### Google Gemini

Requires specific handling for function declarations:

```rust
impl ProviderAdapter for GeminiAdapter {
    fn format_tools(&self, tools: Vec<Tool>) -> Value {
        // Convert tools to Gemini format
        json!({
            "tools": [{
                "functionDeclarations": tools.iter().map(|tool| {
                    json!({
                        "name": tool.name,
                        "description": tool.description,
                        "parameters": {
                            "type": "object",
                            "properties": format_parameters(&tool.parameters),
                            "required": tool.parameters.iter()
                                .filter(|p| p.required)
                                .map(|p| p.name.clone())
                                .collect::<Vec<_>>()
                        }
                    })
                }).collect::<Vec<_>>()
            }]
        })
    }
    
    // ... other methods
}
```

## Error Handling

The system will implement comprehensive error handling for various failure modes:

1. Tool definition errors
2. Provider API errors
3. Tool execution errors
4. Response parsing errors

Each error type will include detailed context to aid debugging.

## Next Steps

1. Implement the core interfaces and traits
2. Create the OpenAI adapter as reference implementation
3. Build a simple tool registry with basic executors
4. Develop integration tests with mock LLM responses
5. Expand to additional providers
