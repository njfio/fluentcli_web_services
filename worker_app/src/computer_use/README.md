# Computer Use Implementation

This module provides a structured approach to tool execution in a computer environment, following the design patterns from Anthropic's computer-use implementation.

## Architecture

The implementation follows a layered architecture:

### Core Traits and Types

- `ToolInfo`: Base trait defining tool metadata and validation
- `ToolExecution`: Trait for executable tools
- `ToolResult`: Result type for tool execution
- `ToolCollection`: Container managing available tools

### Tool Implementations

1. `ExecuteCommand`: Execute CLI commands (similar to bash.py)
   - Supports both Windows and Unix commands
   - Provides command output and error handling

2. `FileOperations`: File system operations
   - Read/write operations with text and binary support
   - File search with regex and pattern matching
   - Directory listing and metadata retrieval
   - Recursive operations support

3. `SiteInspector`: Website inspection
   - Screenshot capture
   - Console log collection
   - Page navigation and state inspection

4. `FollowupQuestion`: User interaction
   - Question formatting
   - Response handling

### Orchestration

The `Computer` struct orchestrates tool execution:

- Tool management and discovery
- Parameter validation
- Error handling and result formatting
- Integration with LLM service

## Usage

### Tool Registration

Tools are registered through the `ToolCollection`:

```rust
let mut collection = ToolCollection::new();
collection.add_tool(ExecuteCommand);
collection.add_tool(FileOperations);
// ... add other tools
```

### Tool Execution

Tools can be executed through the `Computer` struct:

```rust
let computer = Computer::new();
let result = computer.execute_tool(
    "execute_command",
    json!({
        "command": "echo 'Hello, World!'"
    })
).await?;
```

### API Integration

The implementation exposes HTTP endpoints:

- `POST /{tool_name}`: Execute a specific tool
- `GET /tools`: List available tools

## Error Handling

The implementation provides comprehensive error handling:

- Parameter validation
- Tool execution errors
- Resource access errors
- Integration errors

## Testing

Each component includes unit tests:

- Tool implementation tests
- Computer orchestration tests
- API endpoint tests

## Integration with LLM Service

The implementation integrates with the LLM service through:

1. Tool execution handlers
2. Result formatting
3. Error propagation
4. Streaming support

## Best Practices

1. Tool Implementation:
   - Implement both `ToolInfo` and `ToolExecution`
   - Provide comprehensive parameter validation
   - Handle errors gracefully
   - Include unit tests

2. Error Handling:
   - Use specific error types
   - Provide context in error messages
   - Handle cleanup in error cases

3. Testing:
   - Test happy path and error cases
   - Use temporary resources
   - Mock external dependencies

## Example

```rust
// Create a computer instance
let computer = Computer::new();

// Execute a command
let result = computer
    .execute_tool(
        "execute_command",
        json!({
            "command": "echo 'test'"
        }),
    )
    .await?;

// Handle the result
if result.success {
    println!("Output: {}", result.output);
} else {
    eprintln!("Error: {:?}", result.error);
}
