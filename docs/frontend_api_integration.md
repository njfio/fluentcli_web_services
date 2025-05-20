# Frontend API Integration for Function Calling

This document details how to integrate the frontend with the backend API for function calling capabilities.

## API Endpoints

The frontend will need to interact with several new API endpoints to support function calling:

### Tool Management

| Endpoint | Method | Description | Request Body | Response |
|----------|--------|-------------|--------------|----------|
| `/function-calling/tools` | GET | List available tools | N/A | Array of tool objects |
| `/function-calling/tools/:id` | GET | Get tool details | N/A | Tool object |
| `/function-calling/execute` | POST | Execute a tool | Tool call object | Tool result object |

### Agent Management

| Endpoint | Method | Description | Request Body | Response |
|----------|--------|-------------|--------------|----------|
| `/agents` | GET | List agents | N/A | Array of agent objects |
| `/agents/:id` | GET | Get agent details | N/A | Agent object |
| `/agents` | POST | Create agent | Agent object | Created agent object |
| `/agents/:id` | PUT | Update agent | Agent object | Updated agent object |
| `/agents/:id` | DELETE | Delete agent | N/A | Success message |

### Chat with Function Calling

| Endpoint | Method | Description | Request Body | Response |
|----------|--------|-------------|--------------|----------|
| `/llm/chat_with_tools` | POST | Chat with tools | Chat request with tools | Chat response |
| `/llm/stream_chat_with_tools` | POST | Stream chat with tools | Chat request with tools | Streamed chat response |

## API Client Implementation

The API client needs to be extended to support these new endpoints:

```typescript
// Add to apiClient.ts
const apiClient = {
  // Existing endpoints...
  
  // Tool endpoints
  listTools: () => axiosInstance.get('/function-calling/tools'),
  getTool: (id) => axiosInstance.get(`/function-calling/tools/${id}`),
  executeToolCall: (toolCall) => axiosInstance.post('/function-calling/execute', toolCall),
  
  // Agent endpoints
  listAgents: () => axiosInstance.get('/agents'),
  getAgent: (id) => axiosInstance.get(`/agents/${id}`),
  createAgent: (agentData) => axiosInstance.post('/agents', agentData),
  updateAgent: (id, agentData) => axiosInstance.put(`/agents/${id}`, agentData),
  deleteAgent: (id) => axiosInstance.delete(`/agents/${id}`),
  
  // Chat with function calling
  chatWithTools: (userLLMConfigId, conversationId, messages, tools) => 
    axiosInstance.post('/llm/chat_with_tools', {
      user_llm_config_id: userLLMConfigId,
      conversation_id: conversationId,
      messages,
      tools
    }),
};
```

## LLM Service Implementation

The LLM service needs to be extended to support function calling:

```typescript
// Add to LLMService.ts
interface Tool {
  name: string;
  description: string;
  parameters: any;
}

interface ToolCall {
  id: string;
  name: string;
  arguments: any;
}

interface ToolResult {
  tool_call_id: string;
  result: any;
}

async streamChatWithTools(
  userLLMConfigId: string,
  conversationId: string,
  messages: LLMMessage[],
  tools: Tool[]
): Promise<ReadableStream<Uint8Array>> {
  const userLLMConfig = await this.getUserLLMConfig(userLLMConfigId);
  console.log('User LLM Config:', JSON.stringify(userLLMConfig, null, 2));

  if (!userLLMConfig.provider_id) {
    console.error('Provider ID is missing from the user LLM config');
    throw new Error('Provider ID is missing from the user LLM config');
  }

  const url = new URL('/llm/stream_chat_with_tools', axiosInstance.defaults.baseURL);

  // Filter out invalid messages
  const validMessages = messages.filter(msg => msg && msg.role && msg.content);

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
      messages: validMessages,
      tools: tools
    })
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error('LLMService streamChatWithTools - Error response:', errorText);
    console.error('LLMService streamChatWithTools - Response status:', response.status);
    throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
  }

  if (!response.body) {
    throw new Error('No response body');
  }

  return response.body;
}

async executeToolCall(toolCall: ToolCall): Promise<ToolResult> {
  const response = await axiosInstance.post('/function-calling/execute', toolCall);
  return response.data;
}
```

## Data Models

### Tool

```typescript
interface Tool {
  id: string;
  name: string;
  description: string;
  parameters: {
    type: string;
    properties: Record<string, {
      type: string;
      description?: string;
      enum?: string[];
      format?: string;
      minimum?: number;
      maximum?: number;
      items?: any;
    }>;
    required: string[];
  };
  icon?: string;
  category?: string;
}
```

### Tool Call

```typescript
interface ToolCall {
  id: string;
  name: string;
  arguments: any;
  status: 'pending' | 'running' | 'completed' | 'error';
  result?: any;
  error?: string;
}
```

### Agent

```typescript
interface Agent {
  id: string;
  name: string;
  description: string;
  tools: string[]; // Array of tool IDs
  icon?: string;
  system_prompt?: string;
  created_at: string;
  updated_at: string;
}
```

## Request/Response Formats

### Chat with Tools Request

```json
{
  "user_llm_config_id": "123e4567-e89b-12d3-a456-426614174000",
  "conversation_id": "123e4567-e89b-12d3-a456-426614174001",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "What's the weather in Paris?"
    }
  ],
  "tools": [
    {
      "name": "get_weather",
      "description": "Get the current weather for a location",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {
            "type": "string",
            "description": "The city and country, e.g. Paris, France"
          },
          "units": {
            "type": "string",
            "enum": ["celsius", "fahrenheit"],
            "description": "The units to use for temperature"
          }
        },
        "required": ["location"]
      }
    }
  ]
}
```

### Chat with Tools Response

```json
{
  "id": "123e4567-e89b-12d3-a456-426614174002",
  "role": "assistant",
  "content": null,
  "tool_calls": [
    {
      "id": "call_123e4567-e89b-12d3-a456-426614174003",
      "name": "get_weather",
      "arguments": {
        "location": "Paris, France",
        "units": "celsius"
      }
    }
  ]
}
```

### Tool Execution Request

```json
{
  "id": "call_123e4567-e89b-12d3-a456-426614174003",
  "name": "get_weather",
  "arguments": {
    "location": "Paris, France",
    "units": "celsius"
  }
}
```

### Tool Execution Response

```json
{
  "tool_call_id": "call_123e4567-e89b-12d3-a456-426614174003",
  "result": {
    "location": "Paris, France",
    "temperature": 22.5,
    "units": "celsius",
    "condition": "Partly Cloudy",
    "humidity": 65,
    "wind_speed": 10
  }
}
```

## Handling Streaming Responses

The streaming response from the LLM will need to be parsed to handle tool calls and content updates:

```typescript
async function processStream(stream: ReadableStream<Uint8Array>): Promise<void> {
  const reader = stream.getReader();
  const decoder = new TextDecoder();
  
  let assistantMessage: Message | null = null;
  let fullContent = '';
  let toolCalls: ToolCall[] = [];
  
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      
      const chunk = decoder.decode(value);
      const lines = chunk.split('\n').filter(line => line.trim() !== '');
      
      for (const line of lines) {
        if (line.startsWith('data: ')) {
          const data = line.substring(6);
          
          if (data === '[DONE]') {
            // Stream is complete
            break;
          }
          
          try {
            const parsedData = JSON.parse(data);
            
            if (parsedData.choices && parsedData.choices.length > 0) {
              const choice = parsedData.choices[0];
              
              if (choice.delta?.content) {
                // Content update
                fullContent += choice.delta.content;
                
                if (!assistantMessage) {
                  // Create a new assistant message
                  const response = await apiClient.createMessage(
                    conversationId,
                    'assistant',
                    fullContent,
                    'Assistant',
                    undefined,
                    undefined,
                    undefined
                  );
                  
                  assistantMessage = response.data;
                  assistantMessage.renderedContent = await renderMarkdown(assistantMessage.content);
                  store.commit('chat/addMessage', assistantMessage);
                } else {
                  // Update existing message
                  assistantMessage.content = fullContent;
                  assistantMessage.renderedContent = await renderMarkdown(fullContent);
                  store.commit('chat/updateMessage', assistantMessage);
                }
              }
              
              if (choice.delta?.tool_calls) {
                // Tool call update
                for (const toolCallDelta of choice.delta.tool_calls) {
                  const existingToolCall = toolCalls.find(tc => tc.id === toolCallDelta.id);
                  
                  if (existingToolCall) {
                    // Update existing tool call
                    if (toolCallDelta.function?.name) {
                      existingToolCall.name = toolCallDelta.function.name;
                    }
                    
                    if (toolCallDelta.function?.arguments) {
                      existingToolCall.arguments = {
                        ...existingToolCall.arguments,
                        ...JSON.parse(toolCallDelta.function.arguments)
                      };
                    }
                  } else {
                    // Create new tool call
                    const newToolCall: ToolCall = {
                      id: toolCallDelta.id,
                      name: toolCallDelta.function?.name || '',
                      arguments: toolCallDelta.function?.arguments ? JSON.parse(toolCallDelta.function.arguments) : {},
                      status: 'pending'
                    };
                    
                    toolCalls.push(newToolCall);
                  }
                }
                
                // Create or update assistant message with tool calls
                if (!assistantMessage) {
                  const response = await apiClient.createMessage(
                    conversationId,
                    'assistant',
                    '',
                    'Assistant',
                    undefined,
                    undefined,
                    undefined
                  );
                  
                  assistantMessage = response.data;
                  assistantMessage.tool_calls = toolCalls;
                  store.commit('chat/addMessage', assistantMessage);
                } else {
                  assistantMessage.tool_calls = toolCalls;
                  store.commit('chat/updateMessage', assistantMessage);
                }
              }
            }
          } catch (error) {
            console.error('Error parsing stream data:', error);
          }
        }
      }
    }
    
    // Execute tool calls
    if (toolCalls.length > 0) {
      await executeToolCalls(toolCalls, assistantMessage);
    }
  } catch (error) {
    console.error('Error processing stream:', error);
  } finally {
    reader.releaseLock();
  }
}
```

## Error Handling

The frontend should handle various error scenarios:

1. **API Errors**:
   - Network errors
   - Authentication errors
   - Server errors

2. **Tool Execution Errors**:
   - Invalid arguments
   - External service errors
   - Timeout errors

3. **Streaming Errors**:
   - Connection drops
   - Malformed data

Example error handling:

```typescript
try {
  // API call or stream processing
} catch (error) {
  if (error instanceof NetworkError) {
    // Handle network error
    showErrorMessage('Network error. Please check your connection and try again.');
  } else if (error instanceof AuthenticationError) {
    // Handle authentication error
    showErrorMessage('Authentication error. Please log in again.');
    logout();
  } else if (error instanceof ToolExecutionError) {
    // Handle tool execution error
    showErrorMessage(`Tool execution failed: ${error.message}`);
    markToolCallAsFailed(error.toolCallId, error.message);
  } else {
    // Handle other errors
    showErrorMessage('An unexpected error occurred. Please try again.');
    console.error('Unexpected error:', error);
  }
}
```

## Security Considerations

1. **Authentication**: Ensure all API requests include proper authentication tokens.
2. **Input Validation**: Validate all user inputs before sending to the API.
3. **Tool Permissions**: Implement proper authorization for tool access.
4. **Data Sanitization**: Sanitize tool results before displaying to prevent XSS attacks.

## Performance Optimization

1. **Caching**: Cache tool definitions and agent configurations to reduce API calls.
2. **Lazy Loading**: Load tools and agents only when needed.
3. **Debouncing**: Debounce user inputs to prevent excessive API calls.
4. **Pagination**: Implement pagination for listing tools and agents if the lists become large.
