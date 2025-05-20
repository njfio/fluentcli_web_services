import { mount, flushPromises } from '@vue/test-utils';
import { createStore } from 'vuex';
import ChatInput from '@/components/chat/ChatInput.vue';
import ChatArea from '@/components/chat/ChatArea.vue';
import AgentSidebar from '@/components/AgentSidebar.vue';
import toolModule from '@/store/modules/tool';
import agentModule from '@/store/modules/agent';
import { toolApiClient } from '@/services/toolApiClient';
import { agentApiClient } from '@/services/agentApiClient';
import LLMService from '@/services/LLMService';

// Mock the API clients and services
jest.mock('@/services/toolApiClient');
jest.mock('@/services/agentApiClient');
jest.mock('@/services/LLMService');

describe('Function Calling Integration', () => {
  const mockTools = [
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
    },
    {
      id: 'tool_2',
      name: 'search_web',
      description: 'Search the web',
      parameters: {
        type: 'object',
        properties: {
          query: {
            type: 'string',
            description: 'Search query'
          }
        },
        required: ['query']
      }
    }
  ];
  
  const mockAgents = [
    {
      id: 'agent_1',
      name: 'Weather Agent',
      description: 'An agent that can get weather information',
      icon: 'robot',
      system_prompt: 'You are a weather assistant',
      tools: ['tool_1'],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z'
    }
  ];
  
  const mockConversation = {
    id: 'conv_1',
    title: 'Test Conversation',
    mode: 'chat',
    createdAt: '2023-01-01T00:00:00Z',
    updatedAt: '2023-01-01T00:00:00Z'
  };
  
  const mockUserLLMConfigs = [
    {
      id: 'config_1',
      name: 'Test Config',
      user_id: 'user_1',
      provider_id: 'provider_1',
      api_key_id: 'key_1'
    }
  ];
  
  const createVuexStore = () => {
    return createStore({
      modules: {
        tool: toolModule,
        agent: agentModule,
        chat: {
          namespaced: true,
          state: {
            conversations: [mockConversation],
            currentConversation: mockConversation,
            messages: [],
            userLLMConfigs: mockUserLLMConfigs
          },
          actions: {
            createMessage: jest.fn().mockResolvedValue({
              data: {
                id: 'msg_1',
                conversationId: 'conv_1',
                role: 'user',
                content: 'What is the weather in Paris?',
                createdAt: '2023-01-01T00:00:00Z'
              }
            }),
            sendMessageWithTools: jest.fn().mockResolvedValue({
              data: {
                id: 'msg_2',
                conversationId: 'conv_1',
                role: 'assistant',
                content: '',
                createdAt: '2023-01-01T00:00:00Z',
                tool_calls: [
                  {
                    id: 'call_1',
                    name: 'get_weather',
                    arguments: { location: 'Paris' },
                    status: 'pending'
                  }
                ]
              }
            })
          },
          mutations: {
            addMessage: jest.fn(),
            updateMessage: jest.fn()
          }
        }
      }
    });
  };
  
  beforeEach(() => {
    // Reset mocks
    jest.clearAllMocks();
    
    // Mock API responses
    (toolApiClient.listTools as jest.Mock).mockResolvedValue(mockTools);
    (agentApiClient.listAgents as jest.Mock).mockResolvedValue(mockAgents);
    (toolApiClient.executeToolCall as jest.Mock).mockResolvedValue({
      tool_call_id: 'call_1',
      result: { temperature: 22, condition: 'Sunny' }
    });
  });
  
  it('should enable function calling and select an agent', async () => {
    const store = createVuexStore();
    
    // Fetch tools and agents
    await store.dispatch('tool/fetchTools');
    await store.dispatch('agent/fetchAgents');
    
    const wrapper = mount(ChatInput, {
      props: {
        userLLMConfigs: mockUserLLMConfigs,
        selectedConfigId: 'config_1',
        userInput: '',
        currentConversation: mockConversation,
        isLoading: false
      },
      global: {
        plugins: [store],
        stubs: {
          'agent-sidebar': true
        }
      }
    });
    
    // Enable function calling
    const enableFunctionCallingCheckbox = wrapper.find('#enable-function-calling');
    await enableFunctionCallingCheckbox.setValue(true);
    
    // Check that the agent selector is displayed
    expect(wrapper.find('.agent-selector-container').exists()).toBe(true);
    
    // Select an agent
    const agentSelector = wrapper.find('select[v-model="selectedAgentId"]');
    await agentSelector.setValue('agent_1');
    
    // Check that the agent is selected
    expect(store.state.agent.selectedAgentId).toBe('agent_1');
  });
  
  it('should send a message with function calling', async () => {
    const store = createVuexStore();
    
    // Fetch tools and agents
    await store.dispatch('tool/fetchTools');
    await store.dispatch('agent/fetchAgents');
    
    // Select an agent
    await store.dispatch('agent/selectAgent', 'agent_1');
    
    const wrapper = mount(ChatInput, {
      props: {
        userLLMConfigs: mockUserLLMConfigs,
        selectedConfigId: 'config_1',
        userInput: 'What is the weather in Paris?',
        currentConversation: mockConversation,
        isLoading: false
      },
      global: {
        plugins: [store],
        stubs: {
          'agent-sidebar': true
        }
      }
    });
    
    // Enable function calling
    const enableFunctionCallingCheckbox = wrapper.find('#enable-function-calling');
    await enableFunctionCallingCheckbox.setValue(true);
    
    // Send the message
    await wrapper.find('.chat-input-send-button').trigger('click');
    
    // Wait for promises to resolve
    await flushPromises();
    
    // Check that the sendMessageWithTools action was dispatched
    expect(store.state.chat.actions.sendMessageWithTools).toHaveBeenCalled();
    
    // Check that a tool call was added to the store
    expect(store.state.tool.activeCalls).toHaveProperty('call_1');
  });
  
  it('should display tool calls in the chat area', async () => {
    const store = createVuexStore();
    
    // Fetch tools and agents
    await store.dispatch('tool/fetchTools');
    await store.dispatch('agent/fetchAgents');
    
    // Add a message with tool calls
    const message = {
      id: 'msg_1',
      role: 'assistant',
      content: '',
      createdAt: '2023-01-01T00:00:00Z',
      tool_calls: [
        {
          id: 'call_1',
          name: 'get_weather',
          arguments: { location: 'Paris' },
          status: 'completed',
          result: { temperature: 22, condition: 'Sunny' }
        }
      ]
    };
    
    const wrapper = mount(ChatArea, {
      props: {
        messages: [message],
        gridLayout: 'default',
        gridColumns: 1
      },
      global: {
        plugins: [store],
        stubs: {
          'tool-call-display': true,
          'response-toolbar': true,
          'response-top-toolbar': true
        }
      }
    });
    
    // Check that the tool call is displayed
    expect(wrapper.find('tool-call-display-stub').exists()).toBe(true);
  });
});
