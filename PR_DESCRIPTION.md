# Frontend Implementation for Generic Function Calling System

This PR implements the frontend components and integration for the generic function calling system, enabling users to interact with AI agents that can use tools to perform tasks.

## Features

- **Tool Call Display**: Components for displaying tool calls and their results
- **Agent Management**: Components for creating, editing, and managing agents
- **Tool Selection**: Components for selecting tools for agents to use
- **Chat Integration**: Integration with the existing chat interface to support function calling

## Implementation Details

### New Components

- `ToolCallDisplay.vue`: Displays a tool call with its arguments, status, and result
- `ToolResultDisplay.vue`: Displays the result of a tool call in an appropriate format
- `AgentSidebar.vue`: Manages agents and their configurations
- `AgentConfigPanel.vue`: Configures an agent's settings and tools
- `ToolSelector.vue`: Selects tools for an agent to use

### Modified Components

- `ChatInput.vue`: Extended to support function calling and agent selection
- `ChatArea.vue`: Extended to display tool calls inline with chat messages
- `Chat.vue`: Extended to handle function calling flow

### New Store Modules

- `tool.ts`: Manages tool state and tool call execution
- `agent.ts`: Manages agent state and configuration

### API Integration

- Added API client extensions for tools and agents
- Extended LLM service to support function calling
- Added chat with tools endpoint integration

## Testing

- Unit tests for all new components
- Unit tests for store modules
- Integration tests for function calling flow

## Documentation

- Added README with usage examples and component documentation

## Screenshots

[Add screenshots here]

## How to Test

1. Enable function calling in the chat input
2. Create an agent and select tools for it
3. Select the agent in the chat input
4. Send a message that would trigger a tool call
5. Verify that the tool call is displayed and executed correctly
6. Verify that the tool result is displayed correctly

## Checklist

- [x] Implemented all required components
- [x] Added unit tests for components
- [x] Added unit tests for store modules
- [x] Added integration tests
- [x] Added documentation
- [x] Tested manually
- [x] Code follows project style guidelines
- [x] All tests pass

## Related Issues

- #123: Implement generic function calling system
