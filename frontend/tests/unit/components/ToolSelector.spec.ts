import { mount } from '@vue/test-utils';
import { createStore } from 'vuex';
import ToolSelector from '@/components/ToolSelector.vue';

describe('ToolSelector.vue', () => {
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
  
  const createVuexStore = (tools = [], loading = false, error = null) => {
    return createStore({
      modules: {
        tool: {
          namespaced: true,
          state: {
            tools,
            loading,
            error
          },
          actions: {
            fetchTools: jest.fn()
          }
        }
      }
    });
  };
  
  it('renders correctly with tools', () => {
    const store = createVuexStore(mockTools);
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: []
      },
      global: {
        plugins: [store]
      }
    });
    
    // Check that the component renders correctly
    expect(wrapper.findAll('.tool-item').length).toBe(2);
    expect(wrapper.find('.tool-name').text()).toBe('get_weather');
    expect(wrapper.find('.tool-description').text()).toBe('Get weather information');
  });
  
  it('shows loading state', () => {
    const store = createVuexStore([], true);
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: []
      },
      global: {
        plugins: [store]
      }
    });
    
    // Check that the loading indicator is displayed
    expect(wrapper.find('.loading-indicator').exists()).toBe(true);
    expect(wrapper.find('.loading-indicator').text()).toContain('Loading tools');
  });
  
  it('shows error state', () => {
    const store = createVuexStore([], false, 'Failed to fetch tools');
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: []
      },
      global: {
        plugins: [store]
      }
    });
    
    // Check that the error message is displayed
    expect(wrapper.find('.error-message').exists()).toBe(true);
    expect(wrapper.find('.error-message').text()).toBe('Failed to fetch tools');
  });
  
  it('shows empty state', () => {
    const store = createVuexStore([]);
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: []
      },
      global: {
        plugins: [store]
      }
    });
    
    // Check that the empty state is displayed
    expect(wrapper.find('.empty-state').exists()).toBe(true);
    expect(wrapper.find('.empty-state').text()).toBe('No tools available');
  });
  
  it('selects tools correctly', async () => {
    const store = createVuexStore(mockTools);
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: []
      },
      global: {
        plugins: [store]
      }
    });
    
    // Find the first checkbox and check it
    const checkbox = wrapper.find('input[type="checkbox"]');
    await checkbox.setValue(true);
    
    // Check that the update:modelValue event is emitted with the correct value
    expect(wrapper.emitted('update:modelValue')).toBeTruthy();
    expect(wrapper.emitted('update:modelValue')![0][0]).toEqual(['tool_1']);
  });
  
  it('initializes with selected tools', () => {
    const store = createVuexStore(mockTools);
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: ['tool_1']
      },
      global: {
        plugins: [store]
      }
    });
    
    // Check that the first checkbox is checked
    const checkbox = wrapper.find('input[type="checkbox"]');
    expect(checkbox.element.checked).toBe(true);
  });
  
  it('updates when modelValue prop changes', async () => {
    const store = createVuexStore(mockTools);
    
    const wrapper = mount(ToolSelector, {
      props: {
        modelValue: []
      },
      global: {
        plugins: [store]
      }
    });
    
    // Initially no checkboxes are checked
    const checkbox = wrapper.find('input[type="checkbox"]');
    expect(checkbox.element.checked).toBe(false);
    
    // Update the modelValue prop
    await wrapper.setProps({ modelValue: ['tool_1'] });
    
    // Check that the first checkbox is now checked
    expect(checkbox.element.checked).toBe(true);
  });
});
