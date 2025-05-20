# Function Calling Components

This directory contains components for implementing function calling in the frontend application.

## Overview

Function calling allows LLMs to use tools to perform tasks. The frontend implementation includes components for:

1. Displaying tool calls and their results
2. Managing agents that can use tools
3. Selecting tools for agents to use
4. Integrating function calling into the chat interface

## Components

### ToolCallDisplay

Displays a tool call with its arguments, status, and result.

```vue
<tool-call-display
  :tool-call="toolCall"
  @retry="handleRetryToolCall"
/>
```

Props:
- `toolCall`: Object containing tool call details
  - `id`: String - Unique identifier for the tool call
  - `name`: String - Name of the tool
  - `arguments`: Object - Arguments passed to the tool
  - `status`: String - Status of the tool call (pending, running, completed, error)
  - `result`: Object (optional) - Result of the tool call
  - `error`: String (optional) - Error message if the tool call failed

Events:
- `retry`: Emitted when the user wants to retry a failed tool call

### ToolResultDisplay

Displays the result of a tool call in an appropriate format.

```vue
<tool-result-display :result="result" />
```

Props:
- `result`: Any - The result of the tool call

Supports different result types:
- Images: Displays an image with the URL from the result
- Tables: Displays a table with the data from the result
- Code: Displays code with syntax highlighting
- JSON: Displays formatted JSON

### AgentSidebar

Manages agents and their configurations.

```vue
<agent-sidebar />
```

Features:
- List agents
- Create, edit, and delete agents
- Select an agent for the current conversation

### AgentConfigPanel

Configures an agent's settings and tools.

```vue
<agent-config-panel
  :agent="agent"
  @save="handleSaveAgent"
  @cancel="handleCancelAgent"
/>
```

Props:
- `agent`: Object (optional) - Agent to edit, if not provided, a new agent will be created

Events:
- `save`: Emitted when the agent configuration is saved
- `cancel`: Emitted when the configuration is cancelled

### ToolSelector

Selects tools for an agent to use.

```vue
<tool-selector v-model="selectedTools" />
```

Props:
- `modelValue`: Array of selected tool IDs

Events:
- `update:modelValue`: Emitted when the selection changes

## Integration with Chat

The function calling components are integrated into the chat interface:

1. ChatInput has been extended to support function calling:
   - Toggle for enabling/disabling function calling
   - Agent selection dropdown
   - Integration with the agent sidebar

2. ChatArea has been extended to display tool calls:
   - Tool calls are displayed inline with chat messages
   - Tool results are displayed in an appropriate format
   - Failed tool calls can be retried

## Store Modules

### Tool Module

Manages tool state and tool call execution.

```typescript
// Get tools
const tools = store.state.tool.tools;

// Execute a tool call
await store.dispatch('tool/executeToolCall', toolCall);

// Get a tool call by ID
const toolCall = store.getters['tool/getToolCallById'](toolCallId);
```

### Agent Module

Manages agent state and configuration.

```typescript
// Get agents
const agents = store.state.agent.agents;

// Get the selected agent
const selectedAgent = store.getters['agent/selectedAgent'];

// Select an agent
store.dispatch('agent/selectAgent', agentId);

// Create an agent
store.dispatch('agent/createAgent', agentData);

// Update an agent
store.dispatch('agent/updateAgent', { id, agentData });

// Delete an agent
store.dispatch('agent/deleteAgent', agentId);
```

## API Integration

The function calling components integrate with the backend API:

1. `/function-calling/tools`: List available tools
2. `/function-calling/tools/:id`: Get tool details
3. `/function-calling/execute`: Execute a tool call
4. `/agents`: CRUD operations for agents
5. `/llm/chat_with_tools`: Chat with function calling support

## Usage Example

```vue
<template>
  <div>
    <chat-input
      v-model:userInput="userInput"
      v-model:selectedConfigId="selectedConfigId"
      :currentConversation="currentConversation"
      :isLoading="isLoading"
      @send-message="sendMessage"
      @send-message-with-tools="sendMessageWithTools"
    />
    
    <chat-area
      :messages="messages"
      @retry-tool-call="handleRetryToolCall"
    />
  </div>
</template>

<script>
export default {
  methods: {
    async sendMessageWithTools(agentId) {
      // Get the agent
      const agent = this.$store.getters['agent/selectedAgent'];
      
      // Get the tools for the agent
      const tools = agent.tools.map(toolId => {
        return this.$store.state.tool.tools.find(tool => tool.id === toolId);
      }).filter(Boolean);
      
      // Send the message with tools
      await this.$store.dispatch('chat/sendMessageWithTools', {
        conversationId: this.currentConversation.id,
        userLLMConfigId: this.selectedConfigId,
        messages: this.messages,
        tools
      });
    },
    
    async handleRetryToolCall(messageId, toolCallId) {
      // Find the message and tool call
      const message = this.messages.find(m => m.id === messageId);
      if (!message || !message.tool_calls) return;
      
      const toolCall = message.tool_calls.find(tc => tc.id === toolCallId);
      if (!toolCall) return;
      
      // Execute the tool call
      await this.$store.dispatch('tool/executeToolCall', {
        id: toolCall.id,
        name: toolCall.name,
        arguments: toolCall.arguments
      });
    }
  }
}
</script>
```
