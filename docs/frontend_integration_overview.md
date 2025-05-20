# Frontend Integration for Generic Function Calling System

This document provides an overview of how to integrate the generic function calling system into the frontend application.

## Overview

The frontend integration will enable users to interact with AI agents that can use tools to perform tasks. The integration involves:

1. Extending the existing chat interface to support function calling
2. Creating components for displaying tool calls and results
3. Implementing client-side handling of function calling requests and responses
4. Developing a user interface for managing tools and agents

## Current Architecture

The current frontend architecture consists of:

- **Vue.js Components**: Chat interfaces, input components, and message displays
- **Vuex Store**: State management for conversations, messages, and LLM configurations
- **API Client**: Communication with the backend services
- **LLM Service**: Handling of LLM requests and streaming responses

The main components involved in chat functionality are:

- `ChatArea.vue`: Displays chat messages
- `ChatInput.vue`: Handles user input and message sending
- `ChatLogic.ts`: Contains the business logic for chat functionality
- `LLMService.ts`: Manages communication with LLM providers

## Integration Points

### 1. API Client Extension

The `apiClient.ts` file needs to be extended to support function calling endpoints:

```typescript
// Add to apiClient.ts
const apiClient = {
  // Existing endpoints...
  
  // Function calling endpoints
  listTools: () => axiosInstance.get('/function-calling/tools'),
  getTool: (id) => axiosInstance.get(`/function-calling/tools/${id}`),
  executeToolCall: (toolCall) => axiosInstance.post('/function-calling/execute', toolCall),
  
  // Agent endpoints
  listAgents: () => axiosInstance.get('/agents'),
  getAgent: (id) => axiosInstance.get(`/agents/${id}`),
  createAgent: (agentData) => axiosInstance.post('/agents', agentData),
  updateAgent: (id, agentData) => axiosInstance.put(`/agents/${id}`, agentData),
  deleteAgent: (id) => axiosInstance.delete(`/agents/${id}`),
};
```

### 2. LLM Service Extension

The `LLMService.ts` file needs to be extended to support function calling:

```typescript
// Add to LLMService.ts
async streamChatWithFunctionCalling(
  userLLMConfigId: string, 
  conversationId: string, 
  messages: LLMMessage[],
  tools: Tool[]
): Promise<ReadableStream<Uint8Array>> {
  const userLLMConfig = await this.getUserLLMConfig(userLLMConfigId);
  
  const url = new URL('/llm/stream_chat_with_tools', axiosInstance.defaults.baseURL);
  
  const response = await fetch(url.toString(), {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${localStorage.getItem('token')}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      user_llm_config_id: userLLMConfigId,
      provider_id: userLLMConfig.provider_id,
      conversation_id: conversationId,
      messages: messages,
      tools: tools
    })
  });
  
  // Handle response and create transform stream
  // ...
}
```

### 3. Chat Logic Extension

The `ChatLogic.ts` file needs to be extended to handle function calling:

```typescript
// Add to ChatLogic.ts
const sendMessageWithTools = async () => {
  try {
    isLoading.value = true;
    error.value = '';
    
    // Create user message
    const userMessageResponse = await apiClient.createMessage(
      currentConversation.value.id,
      'user',
      userInput.value,
      'User',
      undefined,
      undefined,
      undefined
    );
    
    // Add user message to store
    const userMessage = userMessageResponse.data;
    userMessage.renderedContent = await renderMarkdown(userMessage.content);
    store.commit('chat/addMessage', userMessage);
    
    // Get available tools
    const toolsResponse = await apiClient.listTools();
    const tools = toolsResponse.data;
    
    // Prepare messages for LLM
    const llmMessages = currentMessages.value
      .filter(m => m && typeof m === 'object' && 'role' in m && 'content' in m)
      .map(m => ({
        role: m.role as 'system' | 'user' | 'assistant',
        content: m.content,
      }));
    
    // Stream chat with function calling
    const stream = await LLMService.streamChatWithFunctionCalling(
      selectedConfigId.value,
      currentConversation.value.id,
      llmMessages,
      tools
    );
    
    // Process stream and handle tool calls
    // ...
  } catch (error) {
    console.error('Error sending message with tools:', error);
    error.value = 'Failed to send message. Please try again.';
  } finally {
    isLoading.value = false;
    userInput.value = '';
  }
};
```

### 4. Message Component Extension

The message display components need to be extended to show tool calls and results:

```typescript
// Add to ChatArea.vue
const renderToolCall = (toolCall) => {
  return `
    <div class="tool-call">
      <div class="tool-call-header">
        <span class="tool-name">${toolCall.name}</span>
        <span class="tool-status">${toolCall.status}</span>
      </div>
      <div class="tool-call-arguments">
        <pre>${JSON.stringify(toolCall.arguments, null, 2)}</pre>
      </div>
      ${toolCall.result ? `
        <div class="tool-call-result">
          <pre>${JSON.stringify(toolCall.result, null, 2)}</pre>
        </div>
      ` : ''}
    </div>
  `;
};
```

## New Components

### 1. ToolCallDisplay Component

```vue
<template>
  <div class="tool-call-display">
    <div class="tool-call-header">
      <div class="tool-name">{{ toolCall.name }}</div>
      <div class="tool-status" :class="statusClass">{{ statusText }}</div>
    </div>
    <div class="tool-call-arguments">
      <pre>{{ formattedArguments }}</pre>
    </div>
    <div v-if="toolCall.result" class="tool-call-result">
      <pre>{{ formattedResult }}</pre>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, PropType } from 'vue';

interface ToolCall {
  id: string;
  name: string;
  arguments: any;
  status: 'pending' | 'running' | 'completed' | 'error';
  result?: any;
  error?: string;
}

export default defineComponent({
  name: 'ToolCallDisplay',
  props: {
    toolCall: {
      type: Object as PropType<ToolCall>,
      required: true,
    },
  },
  setup(props) {
    const statusClass = computed(() => {
      return {
        'status-pending': props.toolCall.status === 'pending',
        'status-running': props.toolCall.status === 'running',
        'status-completed': props.toolCall.status === 'completed',
        'status-error': props.toolCall.status === 'error',
      };
    });
    
    const statusText = computed(() => {
      switch (props.toolCall.status) {
        case 'pending': return 'Pending';
        case 'running': return 'Running...';
        case 'completed': return 'Completed';
        case 'error': return 'Error';
        default: return 'Unknown';
      }
    });
    
    const formattedArguments = computed(() => {
      return JSON.stringify(props.toolCall.arguments, null, 2);
    });
    
    const formattedResult = computed(() => {
      return JSON.stringify(props.toolCall.result, null, 2);
    });
    
    return {
      statusClass,
      statusText,
      formattedArguments,
      formattedResult,
    };
  },
});
</script>
```

### 2. AgentConfigPanel Component

```vue
<template>
  <div class="agent-config-panel">
    <h2>Configure Agent</h2>
    <div class="form-group">
      <label for="agent-name">Agent Name</label>
      <input id="agent-name" v-model="agentName" type="text" />
    </div>
    <div class="form-group">
      <label for="agent-description">Description</label>
      <textarea id="agent-description" v-model="agentDescription"></textarea>
    </div>
    <div class="form-group">
      <label>Available Tools</label>
      <div class="tool-list">
        <div v-for="tool in availableTools" :key="tool.id" class="tool-item">
          <input
            type="checkbox"
            :id="`tool-${tool.id}`"
            :value="tool.id"
            v-model="selectedToolIds"
          />
          <label :for="`tool-${tool.id}`">{{ tool.name }}</label>
        </div>
      </div>
    </div>
    <div class="form-actions">
      <button @click="saveAgent" :disabled="!isValid">Save Agent</button>
      <button @click="cancel">Cancel</button>
    </div>
  </div>
</template>

<script lang="ts">
// Implementation details...
</script>
```

## Data Flow

1. **User sends a message**:
   - Message is sent to the backend
   - Backend processes the message with the LLM
   - LLM may return tool calls

2. **Tool calls are processed**:
   - Backend executes tool calls
   - Results are sent back to the LLM
   - LLM generates a final response

3. **Response is displayed**:
   - Tool calls and results are displayed in the chat
   - Final LLM response is displayed

## UI/UX Considerations

1. **Tool Call Visualization**:
   - Tool calls should be visually distinct from regular messages
   - Tool status should be clearly indicated (pending, running, completed, error)
   - Tool results should be formatted appropriately based on the data type

2. **Agent Configuration**:
   - Users should be able to create and configure agents
   - Tool selection should be intuitive
   - Agent capabilities should be clearly communicated

3. **Error Handling**:
   - Tool execution errors should be clearly displayed
   - Users should be able to retry failed tool calls
   - System should gracefully handle network issues

## Next Steps

1. Implement the API client extensions
2. Create the new components for tool calls and agent configuration
3. Extend the chat logic to handle function calling
4. Update the message display to show tool calls and results
5. Implement error handling and retry mechanisms
