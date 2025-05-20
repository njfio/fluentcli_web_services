import { mount } from '@vue/test-utils';
import { createStore } from 'vuex';
import AgentSidebar from '@/components/AgentSidebar.vue';

describe('AgentSidebar.vue', () => {
  const mockAgents = [
    {
      id: 'agent_1',
      name: 'Weather Agent',
      description: 'An agent that can get weather information',
      icon: 'robot',
      tools: ['tool_1'],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z'
    },
    {
      id: 'agent_2',
      name: 'Search Agent',
      description: 'An agent that can search the web',
      icon: 'search',
      tools: ['tool_2'],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z'
    }
  ];
  
  const createVuexStore = (agents = [], selectedAgentId = null, loading = false, error = null) => {
    return createStore({
      modules: {
        agent: {
          namespaced: true,
          state: {
            agents,
            selectedAgentId,
            loading,
            error
          },
          getters: {
            selectedAgent: (state) => {
              return state.selectedAgentId 
                ? state.agents.find(agent => agent.id === state.selectedAgentId) 
                : null;
            }
          },
          actions: {
            fetchAgents: jest.fn(),
            selectAgent: jest.fn(),
            createAgent: jest.fn(),
            updateAgent: jest.fn(),
            deleteAgent: jest.fn(),
            initializeFromLocalStorage: jest.fn()
          },
          mutations: {
            setSelectedAgentId: (state, id) => {
              state.selectedAgentId = id;
            }
          }
        }
      }
    });
  };
  
  it('renders correctly with agents', () => {
    const store = createVuexStore(mockAgents);
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Check that the component renders correctly
    expect(wrapper.findAll('.agent-item').length).toBe(2);
    expect(wrapper.find('.agent-name').text()).toBe('Weather Agent');
    expect(wrapper.find('.agent-description').text()).toBe('An agent that can get weather information');
  });
  
  it('shows loading state', () => {
    const store = createVuexStore([], null, true);
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Check that the loading indicator is displayed
    expect(wrapper.find('.loading-indicator').exists()).toBe(true);
    expect(wrapper.find('.loading-indicator').text()).toContain('Loading agents');
  });
  
  it('shows error state', () => {
    const store = createVuexStore([], null, false, 'Failed to fetch agents');
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Check that the error message is displayed
    expect(wrapper.find('.error-message').exists()).toBe(true);
    expect(wrapper.find('.error-message').text()).toBe('Failed to fetch agents');
  });
  
  it('shows empty state', () => {
    const store = createVuexStore([]);
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Check that the empty state is displayed
    expect(wrapper.find('.empty-state').exists()).toBe(true);
    expect(wrapper.find('.empty-state').text()).toContain('No agents found');
  });
  
  it('selects an agent when clicked', async () => {
    const store = createVuexStore(mockAgents);
    const selectAgentSpy = jest.spyOn(store._actions['agent/selectAgent'][0], 'rawHandler');
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Click the first agent
    await wrapper.find('.agent-item').trigger('click');
    
    // Check that the selectAgent action is dispatched with the correct ID
    expect(selectAgentSpy).toHaveBeenCalledWith('agent_1');
  });
  
  it('shows create agent panel when create button is clicked', async () => {
    const store = createVuexStore(mockAgents);
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Initially, the create agent panel is not shown
    expect(wrapper.find('agent-config-panel-stub').exists()).toBe(false);
    
    // Click the create button
    await wrapper.find('.create-button').trigger('click');
    
    // Check that the create agent panel is shown
    expect(wrapper.find('agent-config-panel-stub').exists()).toBe(true);
  });
  
  it('shows edit agent panel when edit button is clicked', async () => {
    const store = createVuexStore(mockAgents);
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Initially, the edit agent panel is not shown
    expect(wrapper.find('agent-config-panel-stub').exists()).toBe(false);
    
    // Click the edit button
    await wrapper.find('.edit-button').trigger('click');
    
    // Check that the edit agent panel is shown
    expect(wrapper.find('agent-config-panel-stub').exists()).toBe(true);
    expect(wrapper.find('agent-config-panel-stub').attributes('agent')).toBeTruthy();
  });
  
  it('shows delete confirmation when delete button is clicked', async () => {
    const store = createVuexStore(mockAgents);
    
    const wrapper = mount(AgentSidebar, {
      global: {
        plugins: [store],
        stubs: {
          'agent-config-panel': true
        }
      }
    });
    
    // Initially, the delete confirmation is not shown
    expect(wrapper.find('.confirmation-dialog').exists()).toBe(false);
    
    // Click the delete button
    await wrapper.find('.delete-button').trigger('click');
    
    // Check that the delete confirmation is shown
    expect(wrapper.find('.confirmation-dialog').exists()).toBe(true);
    expect(wrapper.find('.confirmation-dialog h3').text()).toContain('Delete Weather Agent');
  });
});
