# Frontend Testing Guide for Function Calling

This document provides guidance on testing the function calling implementation in the frontend application.

## Testing Strategy

The testing strategy for the function calling implementation includes:

1. **Unit Tests**: Testing individual components and functions in isolation
2. **Integration Tests**: Testing the interaction between components
3. **End-to-End Tests**: Testing the complete function calling flow
4. **Manual Tests**: Testing the user experience and edge cases

## Unit Tests

### 1. API Client Tests

Test the API client extensions to ensure they correctly format requests and parse responses.

```typescript
// tests/unit/services/toolApiClient.spec.ts
import { toolApiClient } from '@/services/toolApiClient';
import axios from 'axios';
import MockAdapter from 'axios-mock-adapter';

describe('Tool API Client', () => {
  let mock: MockAdapter;
  
  beforeEach(() => {
    mock = new MockAdapter(axios);
  });
  
  afterEach(() => {
    mock.restore();
  });
  
  it('should fetch tools', async () => {
    const mockTools = [
      { id: '1', name: 'get_weather', description: 'Get weather information' },
      { id: '2', name: 'search_web', description: 'Search the web' }
    ];
    
    mock.onGet('/function-calling/tools').reply(200, mockTools);
    
    const tools = await toolApiClient.listTools();
    
    expect(tools).toEqual(mockTools);
  });
  
  it('should execute a tool call', async () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    const mockResult = {
      tool_call_id: 'call_123',
      result: { temperature: 22, condition: 'Sunny' }
    };
    
    mock.onPost('/function-calling/execute').reply(200, mockResult);
    
    const result = await toolApiClient.executeToolCall(mockToolCall);
    
    expect(result).toEqual(mockResult);
  });
});
```

### 2. Store Module Tests

Test the Vuex store modules to ensure they correctly manage state.

```typescript
// tests/unit/store/tool.spec.ts
import { createStore } from 'vuex';
import toolModule from '@/store/modules/tool';
import { toolApiClient } from '@/services/toolApiClient';

// Mock the API client
jest.mock('@/services/toolApiClient');

describe('Tool Store Module', () => {
  let store: any;
  
  beforeEach(() => {
    store = createStore({
      modules: {
        tool: toolModule
      }
    });
  });
  
  it('should set tools', () => {
    const mockTools = [
      { id: '1', name: 'get_weather', description: 'Get weather information' },
      { id: '2', name: 'search_web', description: 'Search the web' }
    ];
    
    store.commit('tool/setTools', mockTools);
    
    expect(store.state.tool.tools).toEqual(mockTools);
  });
  
  it('should add a tool call', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    store.commit('tool/addToolCall', mockToolCall);
    
    expect(store.state.tool.activeCalls['call_123']).toEqual({
      ...mockToolCall,
      status: 'pending'
    });
  });
  
  it('should update tool call status', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    store.commit('tool/addToolCall', mockToolCall);
    store.commit('tool/updateToolCallStatus', { id: 'call_123', status: 'running' });
    
    expect(store.state.tool.activeCalls['call_123'].status).toBe('running');
  });
  
  it('should set tool call result', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    const mockResult = { temperature: 22, condition: 'Sunny' };
    
    store.commit('tool/addToolCall', mockToolCall);
    store.commit('tool/setToolCallResult', { id: 'call_123', result: mockResult });
    
    expect(store.state.tool.activeCalls['call_123'].status).toBe('completed');
    expect(store.state.tool.activeCalls['call_123'].result).toEqual(mockResult);
  });
  
  it('should fetch tools', async () => {
    const mockTools = [
      { id: '1', name: 'get_weather', description: 'Get weather information' },
      { id: '2', name: 'search_web', description: 'Search the web' }
    ];
    
    (toolApiClient.listTools as jest.Mock).mockResolvedValue(mockTools);
    
    await store.dispatch('tool/fetchTools');
    
    expect(store.state.tool.tools).toEqual(mockTools);
    expect(store.state.tool.loading).toBe(false);
    expect(store.state.tool.error).toBe(null);
  });
  
  it('should execute a tool call', async () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    const mockResult = {
      tool_call_id: 'call_123',
      result: { temperature: 22, condition: 'Sunny' }
    };
    
    (toolApiClient.executeToolCall as jest.Mock).mockResolvedValue(mockResult);
    
    await store.dispatch('tool/executeToolCall', mockToolCall);
    
    expect(store.state.tool.activeCalls['call_123'].status).toBe('completed');
    expect(store.state.tool.activeCalls['call_123'].result).toEqual(mockResult.result);
  });
});
```

### 3. Component Tests

Test the Vue components to ensure they correctly render and handle user interactions.

```typescript
// tests/unit/components/ToolCallDisplay.spec.ts
import { mount } from '@vue/test-utils';
import ToolCallDisplay from '@/components/ToolCallDisplay.vue';

describe('ToolCallDisplay', () => {
  it('should render correctly', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' },
      status: 'completed',
      result: { temperature: 22, condition: 'Sunny' }
    };
    
    const wrapper = mount(ToolCallDisplay, {
      props: {
        toolCall: mockToolCall
      }
    });
    
    expect(wrapper.find('.tool-name').text()).toBe('get_weather');
    expect(wrapper.find('.tool-status').text()).toBe('Completed');
    expect(wrapper.find('.tool-call-arguments').exists()).toBe(true);
    expect(wrapper.find('.tool-call-result').exists()).toBe(true);
  });
  
  it('should emit retry event when retry button is clicked', async () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' },
      status: 'error',
      error: 'Failed to get weather'
    };
    
    const wrapper = mount(ToolCallDisplay, {
      props: {
        toolCall: mockToolCall
      }
    });
    
    await wrapper.find('.retry-button').trigger('click');
    
    expect(wrapper.emitted('retry')).toBeTruthy();
  });
  
  it('should toggle expanded state when expand button is clicked', async () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' },
      status: 'completed',
      result: { temperature: 22, condition: 'Sunny' }
    };
    
    const wrapper = mount(ToolCallDisplay, {
      props: {
        toolCall: mockToolCall
      }
    });
    
    // Initially expanded
    expect(wrapper.find('.tool-call-details').exists()).toBe(true);
    
    // Click to collapse
    await wrapper.find('.expand-button').trigger('click');
    expect(wrapper.find('.tool-call-details').exists()).toBe(false);
    
    // Click to expand again
    await wrapper.find('.expand-button').trigger('click');
    expect(wrapper.find('.tool-call-details').exists()).toBe(true);
  });
});
```

## Integration Tests

### 1. Function Calling Flow

Test the complete function calling flow from sending a message to executing tool calls and displaying results.

```typescript
// tests/integration/functionCalling.spec.ts
import { mount, flushPromises } from '@vue/test-utils';
import { createStore } from 'vuex';
import ChatView from '@/views/ChatView.vue';
import toolModule from '@/store/modules/tool';
import agentModule from '@/store/modules/agent';
import chatModule from '@/store/modules/chat';
import { toolApiClient } from '@/services/toolApiClient';
import LLMService from '@/services/LLMService';

// Mock API clients and services
jest.mock('@/services/toolApiClient');
jest.mock('@/services/LLMService');

describe('Function Calling Flow', () => {
  let store: any;
  let wrapper: any;
  
  beforeEach(() => {
    store = createStore({
      modules: {
        tool: toolModule,
        agent: agentModule,
        chat: chatModule
      }
    });
    
    // Set up initial state
    store.commit('chat/setConversations', [{
      id: 'conv_123',
      title: 'Test Conversation'
    }]);
    
    store.commit('chat/setCurrentConversation', {
      id: 'conv_123',
      title: 'Test Conversation'
    });
    
    store.commit('tool/setTools', [
      {
        id: 'tool_1',
        name: 'get_weather',
        description: 'Get weather information',
        parameters: {
          type: 'object',
          properties: {
            location: {
              type: 'string',
              description: 'Location to get weather for'
            }
          },
          required: ['location']
        }
      }
    ]);
    
    store.commit('agent/setAgents', [
      {
        id: 'agent_1',
        name: 'Weather Agent',
        description: 'An agent that can get weather information',
        tools: ['tool_1'],
        system_prompt: 'You are a weather assistant'
      }
    ]);
    
    store.commit('agent/setSelectedAgentId', 'agent_1');
    
    wrapper = mount(ChatView, {
      global: {
        plugins: [store]
      }
    });
  });
  
  it('should send a message with function calling and execute tool calls', async () => {
    // Mock the stream response
    const mockStream = new ReadableStream({
      start(controller) {
        // Simulate chunks of data
        controller.enqueue(new TextEncoder().encode('data: {"choices":[{"delta":{"tool_calls":[{"id":"call_123","function":{"name":"get_weather"}}]}}]}\n\n'));
        controller.enqueue(new TextEncoder().encode('data: {"choices":[{"delta":{"tool_calls":[{"id":"call_123","function":{"arguments":"{\\"location\\":\\"Paris\\"}"}}]}}]}\n\n'));
        controller.enqueue(new TextEncoder().encode('data: [DONE]\n\n'));
        controller.close();
      }
    });
    
    (LLMService.streamChatWithTools as jest.Mock).mockResolvedValue(mockStream);
    
    const mockToolResult = {
      tool_call_id: 'call_123',
      result: { temperature: 22, condition: 'Sunny' }
    };
    
    (toolApiClient.executeToolCall as jest.Mock).mockResolvedValue(mockToolResult);
    
    // Type a message
    const input = wrapper.find('textarea');
    await input.setValue('What is the weather in Paris?');
    
    // Send the message
    const sendButton = wrapper.find('.chat-input-send-button');
    await sendButton.trigger('click');
    
    // Wait for all promises to resolve
    await flushPromises();
    
    // Check that the tool call was added
    expect(store.state.tool.activeCalls['call_123']).toBeTruthy();
    expect(store.state.tool.activeCalls['call_123'].name).toBe('get_weather');
    expect(store.state.tool.activeCalls['call_123'].arguments).toEqual({ location: 'Paris' });
    
    // Check that the tool call was executed
    expect(toolApiClient.executeToolCall).toHaveBeenCalledWith({
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    });
    
    // Check that the tool result was set
    expect(store.state.tool.activeCalls['call_123'].result).toEqual({ temperature: 22, condition: 'Sunny' });
    expect(store.state.tool.activeCalls['call_123'].status).toBe('completed');
    
    // Check that the tool call is displayed
    await flushPromises();
    expect(wrapper.find('.tool-call').exists()).toBe(true);
    expect(wrapper.find('.tool-name').text()).toBe('get_weather');
    expect(wrapper.find('.tool-status').text()).toBe('Completed');
  });
});
```

## End-to-End Tests

End-to-end tests should be implemented using a tool like Cypress to test the complete user flow.

```typescript
// cypress/integration/functionCalling.spec.ts
describe('Function Calling', () => {
  beforeEach(() => {
    // Set up test data
    cy.intercept('GET', '/agents', { fixture: 'agents.json' });
    cy.intercept('GET', '/function-calling/tools', { fixture: 'tools.json' });
    cy.intercept('GET', '/chat/conversations', { fixture: 'conversations.json' });
    cy.intercept('GET', '/chat/conversations/*/messages', { fixture: 'messages.json' });
    
    // Log in
    cy.login();
    
    // Navigate to chat
    cy.visit('/chat');
  });
  
  it('should send a message with function calling and execute tool calls', () => {
    // Select a conversation
    cy.get('.conversation-item').first().click();
    
    // Select an agent
    cy.get('.agent-selector').click();
    cy.get('.agent-option').contains('Weather Agent').click();
    
    // Enable function calling
    cy.get('#enable-function-calling').check();
    
    // Type a message
    cy.get('textarea').type('What is the weather in Paris?');
    
    // Intercept the stream chat request
    cy.intercept('POST', '/llm/stream_chat_with_tools', (req) => {
      req.reply({
        statusCode: 200,
        headers: {
          'Content-Type': 'text/event-stream'
        },
        body: 'data: {"choices":[{"delta":{"tool_calls":[{"id":"call_123","function":{"name":"get_weather"}}]}}]}\n\n' +
              'data: {"choices":[{"delta":{"tool_calls":[{"id":"call_123","function":{"arguments":"{\\"location\\":\\"Paris\\"}"}}]}}]}\n\n' +
              'data: [DONE]\n\n'
      });
    });
    
    // Intercept the tool execution request
    cy.intercept('POST', '/function-calling/execute', {
      statusCode: 200,
      body: {
        tool_call_id: 'call_123',
        result: { temperature: 22, condition: 'Sunny' }
      }
    });
    
    // Send the message
    cy.get('.chat-input-send-button').click();
    
    // Check that the tool call is displayed
    cy.get('.tool-call').should('exist');
    cy.get('.tool-name').should('contain', 'get_weather');
    cy.get('.tool-status').should('contain', 'Completed');
    
    // Check that the tool result is displayed
    cy.get('.tool-call-result').should('exist');
    cy.get('.tool-call-result').should('contain', '22');
    cy.get('.tool-call-result').should('contain', 'Sunny');
  });
});
```

## Manual Testing Checklist

### Basic Functionality

- [ ] User can enable/disable function calling
- [ ] User can select an agent
- [ ] User can send a message with function calling enabled
- [ ] Tool calls are displayed correctly
- [ ] Tool results are displayed correctly
- [ ] User can retry failed tool calls

### Agent Management

- [ ] User can create a new agent
- [ ] User can edit an existing agent
- [ ] User can delete an agent
- [ ] User can select tools for an agent
- [ ] User can set a system prompt for an agent

### Error Handling

- [ ] Application handles network errors gracefully
- [ ] Application handles tool execution errors gracefully
- [ ] Application displays helpful error messages
- [ ] User can retry failed operations

### Performance

- [ ] Application remains responsive during tool execution
- [ ] Streaming responses are rendered progressively
- [ ] Large tool results are handled efficiently

### Accessibility

- [ ] All components are keyboard accessible
- [ ] All components have proper ARIA attributes
- [ ] Color contrast meets accessibility standards
- [ ] Screen readers can access all content

## Test Data

Create test data for tools and agents to use in tests:

```json
// fixtures/tools.json
[
  {
    "id": "tool_1",
    "name": "get_weather",
    "description": "Get weather information",
    "parameters": {
      "type": "object",
      "properties": {
        "location": {
          "type": "string",
          "description": "Location to get weather for"
        },
        "units": {
          "type": "string",
          "enum": ["celsius", "fahrenheit"],
          "description": "Units for temperature"
        }
      },
      "required": ["location"]
    }
  },
  {
    "id": "tool_2",
    "name": "search_web",
    "description": "Search the web",
    "parameters": {
      "type": "object",
      "properties": {
        "query": {
          "type": "string",
          "description": "Search query"
        },
        "num_results": {
          "type": "number",
          "description": "Number of results to return"
        }
      },
      "required": ["query"]
    }
  }
]
```

```json
// fixtures/agents.json
[
  {
    "id": "agent_1",
    "name": "Weather Agent",
    "description": "An agent that can get weather information",
    "tools": ["tool_1"],
    "system_prompt": "You are a weather assistant",
    "icon": "cloud"
  },
  {
    "id": "agent_2",
    "name": "Research Agent",
    "description": "An agent that can search the web",
    "tools": ["tool_2"],
    "system_prompt": "You are a research assistant",
    "icon": "search"
  }
]
```
