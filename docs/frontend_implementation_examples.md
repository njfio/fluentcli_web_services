# Frontend Implementation Examples for Function Calling

This document provides concrete code examples for implementing key components of the function calling system in the frontend.

## 1. API Client Extensions

### Tool API Client

```typescript
// src/services/toolApiClient.ts
import { axiosInstance } from './apiClient';
import { Tool, ToolCall, ToolResult } from '../types/tool';

export const toolApiClient = {
  /**
   * List all available tools
   */
  listTools: async (): Promise<Tool[]> => {
    const response = await axiosInstance.get('/function-calling/tools');
    return response.data;
  },
  
  /**
   * Get details for a specific tool
   */
  getTool: async (id: string): Promise<Tool> => {
    const response = await axiosInstance.get(`/function-calling/tools/${id}`);
    return response.data;
  },
  
  /**
   * Execute a tool call
   */
  executeToolCall: async (toolCall: ToolCall): Promise<ToolResult> => {
    const response = await axiosInstance.post('/function-calling/execute', toolCall);
    return response.data;
  }
};
```

### Agent API Client

```typescript
// src/services/agentApiClient.ts
import { axiosInstance } from './apiClient';
import { Agent, CreateAgentRequest, UpdateAgentRequest } from '../types/agent';

export const agentApiClient = {
  /**
   * List all agents
   */
  listAgents: async (): Promise<Agent[]> => {
    const response = await axiosInstance.get('/agents');
    return response.data;
  },
  
  /**
   * Get details for a specific agent
   */
  getAgent: async (id: string): Promise<Agent> => {
    const response = await axiosInstance.get(`/agents/${id}`);
    return response.data;
  },
  
  /**
   * Create a new agent
   */
  createAgent: async (agentData: CreateAgentRequest): Promise<Agent> => {
    const response = await axiosInstance.post('/agents', agentData);
    return response.data;
  },
  
  /**
   * Update an existing agent
   */
  updateAgent: async (id: string, agentData: UpdateAgentRequest): Promise<Agent> => {
    const response = await axiosInstance.put(`/agents/${id}`, agentData);
    return response.data;
  },
  
  /**
   * Delete an agent
   */
  deleteAgent: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/agents/${id}`);
  }
};
```

## 2. LLM Service Extensions

```typescript
// src/services/LLMService.ts (extensions)
import { Tool, ToolCall, ToolResult } from '../types/tool';
import { axiosInstance } from './apiClient';
import { toolApiClient } from './toolApiClient';

// Add to LLMService class
async streamChatWithTools(
  userLLMConfigId: string,
  conversationId: string,
  messages: LLMMessage[],
  tools: Tool[]
): Promise<ReadableStream<Uint8Array>> {
  const userLLMConfig = await this.getUserLLMConfig(userLLMConfigId);
  
  if (!userLLMConfig.provider_id) {
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
    throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
  }
  
  if (!response.body) {
    throw new Error('No response body');
  }
  
  return response.body;
}

async executeToolCall(toolCall: ToolCall): Promise<ToolResult> {
  return toolApiClient.executeToolCall(toolCall);
}
```

## 3. Vuex Store Extensions

### Tool Store Module

```typescript
// src/store/modules/tool.ts
import { Module } from 'vuex';
import { RootState } from '../index';
import { Tool, ToolCall, ToolResult } from '../../types/tool';
import { toolApiClient } from '../../services/toolApiClient';

export interface ToolState {
  tools: Tool[];
  loading: boolean;
  error: string | null;
  activeCalls: Record<string, ToolCall & { status: string, result?: any, error?: string }>;
}

const toolModule: Module<ToolState, RootState> = {
  namespaced: true,
  
  state: {
    tools: [],
    loading: false,
    error: null,
    activeCalls: {}
  },
  
  mutations: {
    setTools(state, tools: Tool[]) {
      state.tools = tools;
    },
    
    setLoading(state, loading: boolean) {
      state.loading = loading;
    },
    
    setError(state, error: string | null) {
      state.error = error;
    },
    
    addToolCall(state, toolCall: ToolCall) {
      state.activeCalls[toolCall.id] = {
        ...toolCall,
        status: 'pending'
      };
    },
    
    updateToolCallStatus(state, { id, status }: { id: string, status: string }) {
      if (state.activeCalls[id]) {
        state.activeCalls[id].status = status;
      }
    },
    
    setToolCallResult(state, { id, result }: { id: string, result: any }) {
      if (state.activeCalls[id]) {
        state.activeCalls[id].status = 'completed';
        state.activeCalls[id].result = result;
      }
    },
    
    setToolCallError(state, { id, error }: { id: string, error: string }) {
      if (state.activeCalls[id]) {
        state.activeCalls[id].status = 'error';
        state.activeCalls[id].error = error;
      }
    }
  },
  
  actions: {
    async fetchTools({ commit }) {
      commit('setLoading', true);
      commit('setError', null);
      
      try {
        const tools = await toolApiClient.listTools();
        commit('setTools', tools);
      } catch (error) {
        console.error('Error fetching tools:', error);
        commit('setError', 'Failed to fetch tools');
      } finally {
        commit('setLoading', false);
      }
    },
    
    async executeToolCall({ commit }, toolCall: ToolCall) {
      commit('addToolCall', toolCall);
      commit('updateToolCallStatus', { id: toolCall.id, status: 'running' });
      
      try {
        const result = await toolApiClient.executeToolCall(toolCall);
        commit('setToolCallResult', { id: toolCall.id, result: result.result });
        return result;
      } catch (error) {
        console.error('Error executing tool call:', error);
        const errorMessage = error instanceof Error ? error.message : 'Unknown error';
        commit('setToolCallError', { id: toolCall.id, error: errorMessage });
        throw error;
      }
    }
  },
  
  getters: {
    getToolById: (state) => (id: string) => {
      return state.tools.find(tool => tool.id === id);
    },
    
    getToolCallById: (state) => (id: string) => {
      return state.activeCalls[id];
    }
  }
};

export default toolModule;
```

### Agent Store Module

```typescript
// src/store/modules/agent.ts
import { Module } from 'vuex';
import { RootState } from '../index';
import { Agent } from '../../types/agent';
import { agentApiClient } from '../../services/agentApiClient';

export interface AgentState {
  agents: Agent[];
  selectedAgentId: string | null;
  loading: boolean;
  error: string | null;
}

const agentModule: Module<AgentState, RootState> = {
  namespaced: true,
  
  state: {
    agents: [],
    selectedAgentId: null,
    loading: false,
    error: null
  },
  
  mutations: {
    setAgents(state, agents: Agent[]) {
      state.agents = agents;
    },
    
    setSelectedAgentId(state, id: string | null) {
      state.selectedAgentId = id;
    },
    
    setLoading(state, loading: boolean) {
      state.loading = loading;
    },
    
    setError(state, error: string | null) {
      state.error = error;
    },
    
    addAgent(state, agent: Agent) {
      state.agents.push(agent);
    },
    
    updateAgent(state, updatedAgent: Agent) {
      const index = state.agents.findIndex(agent => agent.id === updatedAgent.id);
      if (index !== -1) {
        state.agents[index] = updatedAgent;
      }
    },
    
    removeAgent(state, id: string) {
      state.agents = state.agents.filter(agent => agent.id !== id);
      if (state.selectedAgentId === id) {
        state.selectedAgentId = null;
      }
    }
  },
  
  actions: {
    async fetchAgents({ commit }) {
      commit('setLoading', true);
      commit('setError', null);
      
      try {
        const agents = await agentApiClient.listAgents();
        commit('setAgents', agents);
      } catch (error) {
        console.error('Error fetching agents:', error);
        commit('setError', 'Failed to fetch agents');
      } finally {
        commit('setLoading', false);
      }
    },
    
    selectAgent({ commit }, id: string | null) {
      commit('setSelectedAgentId', id);
      localStorage.setItem('selectedAgentId', id || '');
    },
    
    async createAgent({ commit }, agentData) {
      commit('setLoading', true);
      commit('setError', null);
      
      try {
        const agent = await agentApiClient.createAgent(agentData);
        commit('addAgent', agent);
        return agent;
      } catch (error) {
        console.error('Error creating agent:', error);
        commit('setError', 'Failed to create agent');
        throw error;
      } finally {
        commit('setLoading', false);
      }
    },
    
    async updateAgent({ commit }, { id, agentData }) {
      commit('setLoading', true);
      commit('setError', null);
      
      try {
        const agent = await agentApiClient.updateAgent(id, agentData);
        commit('updateAgent', agent);
        return agent;
      } catch (error) {
        console.error('Error updating agent:', error);
        commit('setError', 'Failed to update agent');
        throw error;
      } finally {
        commit('setLoading', false);
      }
    },
    
    async deleteAgent({ commit }, id) {
      commit('setLoading', true);
      commit('setError', null);
      
      try {
        await agentApiClient.deleteAgent(id);
        commit('removeAgent', id);
      } catch (error) {
        console.error('Error deleting agent:', error);
        commit('setError', 'Failed to delete agent');
        throw error;
      } finally {
        commit('setLoading', false);
      }
    },
    
    initializeFromLocalStorage({ commit }) {
      const selectedAgentId = localStorage.getItem('selectedAgentId');
      if (selectedAgentId) {
        commit('setSelectedAgentId', selectedAgentId);
      }
    }
  },
  
  getters: {
    selectedAgent: (state) => {
      return state.selectedAgentId 
        ? state.agents.find(agent => agent.id === state.selectedAgentId) 
        : null;
    }
  }
};

export default agentModule;
```

## 4. Component Implementation Examples

### ToolCallDisplay Component

```vue
<template>
  <div class="tool-call" :class="{ 'has-error': hasError }">
    <div class="tool-call-header">
      <div class="tool-icon">
        <icon :name="toolIcon" />
      </div>
      <div class="tool-info">
        <div class="tool-name">{{ toolCall.name }}</div>
        <div class="tool-status" :class="statusClass">{{ statusText }}</div>
      </div>
      <div class="tool-actions">
        <button v-if="hasError" @click="$emit('retry')" class="retry-button">
          <icon name="refresh" />
          Retry
        </button>
        <button @click="expanded = !expanded" class="expand-button">
          <icon :name="expanded ? 'chevron-up' : 'chevron-down'" />
        </button>
      </div>
    </div>
    
    <div v-if="expanded" class="tool-call-details">
      <div class="tool-call-arguments">
        <h4>Arguments</h4>
        <pre><code>{{ formattedArguments }}</code></pre>
      </div>
      
      <div v-if="toolCall.result" class="tool-call-result">
        <h4>Result</h4>
        <tool-result-display :result="toolCall.result" />
      </div>
      
      <div v-if="hasError" class="tool-call-error">
        <h4>Error</h4>
        <div class="error-message">{{ toolCall.error }}</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, ref, PropType } from 'vue';
import ToolResultDisplay from './ToolResultDisplay.vue';
import Icon from '../common/Icon.vue';

export default defineComponent({
  name: 'ToolCallDisplay',
  components: {
    ToolResultDisplay,
    Icon
  },
  props: {
    toolCall: {
      type: Object as PropType<{
        id: string;
        name: string;
        arguments: any;
        status: string;
        result?: any;
        error?: string;
      }>,
      required: true
    }
  },
  emits: ['retry'],
  setup(props) {
    const expanded = ref(true);
    
    const hasError = computed(() => props.toolCall.status === 'error');
    
    const statusClass = computed(() => {
      return {
        'status-pending': props.toolCall.status === 'pending',
        'status-running': props.toolCall.status === 'running',
        'status-completed': props.toolCall.status === 'completed',
        'status-error': props.toolCall.status === 'error'
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
    
    const toolIcon = computed(() => {
      // Map tool names to icons
      const iconMap: Record<string, string> = {
        'get_weather': 'cloud',
        'search_web': 'search',
        'calculate': 'calculator',
        'get_stock_price': 'trending-up',
        'generate_image': 'image'
      };
      
      return iconMap[props.toolCall.name] || 'tool';
    });
    
    const formattedArguments = computed(() => {
      return JSON.stringify(props.toolCall.arguments, null, 2);
    });
    
    return {
      expanded,
      hasError,
      statusClass,
      statusText,
      toolIcon,
      formattedArguments
    };
  }
});
</script>
```

### ChatLogic Extension for Function Calling

```typescript
// Add to ChatLogic.ts
import { useStore } from 'vuex';
import { computed, ref } from 'vue';
import { Tool, ToolCall } from '../types/tool';

export function useFunctionCalling() {
  const store = useStore();
  
  const enableFunctionCalling = ref(false);
  const processingToolCalls = ref(false);
  
  const selectedAgentId = computed({
    get: () => store.state.agent.selectedAgentId,
    set: (value) => store.dispatch('agent/selectAgent', value)
  });
  
  const selectedAgent = computed(() => store.getters['agent/selectedAgent']);
  
  const availableTools = computed(() => store.state.tool.tools);
  
  const agentTools = computed(() => {
    if (!selectedAgent.value) return [];
    
    return selectedAgent.value.tools
      .map(toolId => availableTools.value.find(tool => tool.id === toolId))
      .filter(Boolean) as Tool[];
  });
  
  const sendMessageWithTools = async (userInput: string, conversationId: string, userLLMConfigId: string, messages: any[]) => {
    try {
      // Create user message
      const userMessageResponse = await apiClient.createMessage(
        conversationId,
        'user',
        userInput,
        'User',
        undefined,
        undefined,
        undefined
      );
      
      const userMessage = userMessageResponse.data;
      userMessage.renderedContent = await renderMarkdown(userMessage.content);
      store.commit('chat/addMessage', userMessage);
      
      // Prepare messages for LLM
      const llmMessages = messages
        .filter(m => m && typeof m === 'object' && 'role' in m && 'content' in m)
        .map(m => ({
          role: m.role as 'system' | 'user' | 'assistant',
          content: m.content,
        }));
      
      // Add system prompt from agent if available
      if (selectedAgent.value && selectedAgent.value.system_prompt) {
        llmMessages.unshift({
          role: 'system',
          content: selectedAgent.value.system_prompt
        });
      }
      
      // Stream chat with function calling
      const stream = await LLMService.streamChatWithTools(
        userLLMConfigId,
        conversationId,
        llmMessages,
        agentTools.value
      );
      
      await processToolCallStream(stream, conversationId);
    } catch (error) {
      console.error('Error sending message with tools:', error);
      throw error;
    }
  };
  
  const processToolCallStream = async (stream: ReadableStream<Uint8Array>, conversationId: string) => {
    const reader = stream.getReader();
    const decoder = new TextDecoder();
    
    let assistantMessage: any = null;
    let fullContent = '';
    let toolCalls: ToolCall[] = [];
    
    try {
      processingToolCalls.value = true;
      
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        
        const chunk = decoder.decode(value);
        // Process chunk and handle tool calls
        // ...
      }
      
      // Execute tool calls if any
      if (toolCalls.length > 0) {
        for (const toolCall of toolCalls) {
          await store.dispatch('tool/executeToolCall', toolCall);
        }
      }
    } catch (error) {
      console.error('Error processing tool call stream:', error);
      throw error;
    } finally {
      processingToolCalls.value = false;
      reader.releaseLock();
    }
  };
  
  const retryToolCall = async (toolCallId: string) => {
    const toolCall = store.getters['tool/getToolCallById'](toolCallId);
    if (toolCall) {
      await store.dispatch('tool/executeToolCall', {
        id: toolCall.id,
        name: toolCall.name,
        arguments: toolCall.arguments
      });
    }
  };
  
  return {
    enableFunctionCalling,
    processingToolCalls,
    selectedAgentId,
    selectedAgent,
    availableTools,
    agentTools,
    sendMessageWithTools,
    retryToolCall
  };
}
```
