# MCP Integration Guide for Function Calling

This guide explains how to integrate the Mendable Control Protocol (MCP) with our generic function calling system to enable remote tool execution.

## Overview

MCP allows for remote execution of tools across network boundaries, enabling a distributed architecture where tools can be executed on different machines or services. Our function calling system will support both local tool execution and remote execution via MCP.

```
┌─────────────────┐     ┌───────────────────┐     ┌─────────────────┐
│                 │     │                   │     │                 │
│  LLM Provider   │────▶│  Function Calling │────▶│  Local Tools    │
│                 │     │  System           │     │                 │
└─────────────────┘     └───────────────────┘     └─────────────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │                 │
                        │  MCP Client     │
                        │                 │
                        └────────┬────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │                 │
                        │  MCP Server     │
                        │                 │
                        └────────┬────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │                 │
                        │  Remote Tools   │
                        │                 │
                        └─────────────────┘
```

## MCP Client Implementation

The MCP client will implement the `ToolExecutor` trait to seamlessly integrate with our function calling system:

```rust
pub struct MCPClient {
    client: reqwest::Client,
    base_url: String,
    auth_token: String,
}

impl MCPClient {
    pub fn new(base_url: &str, auth_token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }
    
    async fn call_remote_tool(&self, tool_name: &str, args: Value) -> Result<Value, ToolError> {
        let response = self.client
            .post(&format!("{}/tools/{}", self.base_url, tool_name))
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
}

impl ToolExecutor for MCPClient {
    async fn execute(&self, tool_name: &str, args: Value) -> Result<Value, ToolError> {
        self.call_remote_tool(tool_name, args).await
    }
}
```

## MCP Server Implementation

The MCP server will expose remote tools via a REST API:

```rust
pub struct MCPServer {
    tool_registry: ToolRegistry,
    auth_service: AuthService,
}

impl MCPServer {
    pub fn new(tool_registry: ToolRegistry, auth_service: AuthService) -> Self {
        Self {
            tool_registry,
            auth_service,
        }
    }
    
    pub async fn run(self, addr: &str) -> Result<(), std::io::Error> {
        let app = Router::new()
            .route("/tools/:name", post(Self::handle_tool_execution))
            .layer(Extension(self.tool_registry))
            .layer(Extension(self.auth_service));
            
        axum::Server::bind(&addr.parse().unwrap())
            .serve(app.into_make_service())
            .await
    }
    
    async fn handle_tool_execution(
        Path(name): Path<String>,
        Extension(tool_registry): Extension<ToolRegistry>,
        Extension(auth_service): Extension<AuthService>,
        headers: HeaderMap,
        Json(args): Json<Value>,
    ) -> Result<Json<Value>, StatusCode> {
        // Validate authentication
        let auth_header = headers.get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
            
        if !auth_service.validate_token(auth_header) {
            return Err(StatusCode::UNAUTHORIZED);
        }
        
        // Execute the tool
        match tool_registry.execute(&name, args).await {
            Ok(result) => Ok(Json(result)),
            Err(e) => {
                eprintln!("Tool execution error: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
```

## Registering MCP Tools

To use MCP tools with the function calling system, register the MCP client as a tool executor:

```rust
let mut tool_registry = ToolRegistry::new();

// Register local tools
tool_registry.register("get_weather", WeatherTool::new());
tool_registry.register("search_database", DatabaseSearchTool::new());

// Register remote MCP tools
let mcp_client = MCPClient::new("https://mcp-server.example.com", "auth_token_here");
tool_registry.register("remote_image_generation", mcp_client.clone());
tool_registry.register("remote_code_execution", mcp_client);

// Create function calling manager with the registry
let manager = FunctionCallingManager::new(LLMProvider::OpenAI, tool_registry);
```

## Tool Discovery

The MCP server can provide a discovery endpoint to list available tools:

```rust
impl MCPServer {
    // ... existing code ...
    
    async fn handle_tool_discovery(
        Extension(tool_registry): Extension<ToolRegistry>,
        Extension(auth_service): Extension<AuthService>,
        headers: HeaderMap,
    ) -> Result<Json<Vec<Tool>>, StatusCode> {
        // Validate authentication
        // ... auth code ...
        
        Ok(Json(tool_registry.list_tools()))
    }
}
```

The MCP client can then discover available tools:

```rust
impl MCPClient {
    // ... existing code ...
    
    pub async fn discover_tools(&self) -> Result<Vec<Tool>, ToolError> {
        let response = self.client
            .get(&format!("{}/tools", self.base_url))
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await
            .map_err(|e| ToolError::NetworkError(e.to_string()))?;
            
        if !response.status().is_success() {
            return Err(ToolError::DiscoveryError(format!(
                "Tool discovery failed with status: {}",
                response.status()
            )));
        }
        
        response.json::<Vec<Tool>>()
            .await
            .map_err(|e| ToolError::DeserializationError(e.to_string()))
    }
}
```

## Security Considerations

1. **Authentication**: Use JWT or API keys to authenticate MCP clients
2. **Authorization**: Implement role-based access control for tools
3. **Input Validation**: Validate all inputs before execution
4. **Rate Limiting**: Prevent abuse with rate limiting
5. **Logging**: Log all tool executions for audit purposes

## Error Handling

MCP-specific error types:

```rust
pub enum ToolError {
    // ... existing errors ...
    NetworkError(String),
    RemoteToolError(String),
    DiscoveryError(String),
    AuthenticationError(String),
    AuthorizationError(String),
}
```

## Example: Complete Flow

```rust
// 1. Set up tool registry with local and remote tools
let mut tool_registry = ToolRegistry::new();
tool_registry.register("local_weather", WeatherTool::new());

let mcp_client = MCPClient::new("https://mcp-server.example.com", "auth_token_here");
let remote_tools = mcp_client.discover_tools().await?;

for tool in remote_tools {
    tool_registry.register_remote(&tool.name, mcp_client.clone());
}

// 2. Create function calling manager
let manager = FunctionCallingManager::new(LLMProvider::OpenAI, tool_registry);

// 3. Prepare LLM request with tools
let mut request = json!({
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "What's the weather in Paris?"}]
});

let tools = vec![
    Tool::new("local_weather", "Get weather information", vec![
        ToolParameter::new("location", ParameterType::String, true),
    ]),
    // Remote tools are registered with the same interface
];

manager.prepare_request(&mut request, tools, ToolChoice::Auto);

// 4. Send request to LLM provider
let response = client.post("https://api.openai.com/v1/chat/completions")
    .json(&request)
    .send()
    .await?
    .json::<Value>()
    .await?;

// 5. Handle tool calls (local or remote)
let result = manager.handle_response(response).await?;

// 6. Continue conversation with tool results
// ...
```

## Next Steps

1. Implement the MCP client and server
2. Add authentication and authorization
3. Create tool discovery mechanism
4. Develop logging and monitoring
5. Test with various remote tools
