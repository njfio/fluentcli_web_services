import { mount } from '@vue/test-utils';
import { createStore } from 'vuex';
import AgentConfigPanel from '@/components/AgentConfigPanel.vue';

describe('AgentConfigPanel.vue', () => {
  const createVuexStore = () => {
    return createStore({
      modules: {
        tool: {
          namespaced: true,
          state: {
            tools: [
              {
                id: 'tool_1',
                name: 'get_weather',
                description: 'Get weather information'
              },
              {
                id: 'tool_2',
                name: 'search_web',
                description: 'Search the web'
              }
            ],
            loading: false,
            error: null
          },
          actions: {
            fetchTools: jest.fn()
          }
        }
      }
    });
  };
  
  it('renders correctly in create mode', () => {
    const store = createVuexStore();
    
    const wrapper = mount(AgentConfigPanel, {
      props: {},
      global: {
        plugins: [store],
        stubs: {
          'tool-selector': true
        }
      }
    });
    
    // Check that the component renders correctly
    expect(wrapper.find('.panel-header h3').text()).toBe('Create Agent');
    expect(wrapper.find('#agent-name').exists()).toBe(true);
    expect(wrapper.find('#agent-description').exists()).toBe(true);
    expect(wrapper.find('#agent-icon').exists()).toBe(true);
    expect(wrapper.find('#agent-system-prompt').exists()).toBe(true);
    expect(wrapper.find('tool-selector-stub').exists()).toBe(true);
  });
  
  it('renders correctly in edit mode', () => {
    const store = createVuexStore();
    const mockAgent = {
      id: 'agent_1',
      name: 'Weather Agent',
      description: 'An agent that can get weather information',
      icon: 'cloud',
      system_prompt: 'You are a weather assistant',
      tools: ['tool_1'],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z'
    };
    
    const wrapper = mount(AgentConfigPanel, {
      props: {
        agent: mockAgent
      },
      global: {
        plugins: [store],
        stubs: {
          'tool-selector': true
        }
      }
    });
    
    // Check that the component renders correctly
    expect(wrapper.find('.panel-header h3').text()).toBe('Edit Agent');
    expect(wrapper.find('#agent-name').element.value).toBe('Weather Agent');
    expect(wrapper.find('#agent-description').element.value).toBe('An agent that can get weather information');
    expect(wrapper.find('#agent-icon').element.value).toBe('cloud');
    expect(wrapper.find('#agent-system-prompt').element.value).toBe('You are a weather assistant');
  });
  
  it('validates form correctly', async () => {
    const store = createVuexStore();
    
    const wrapper = mount(AgentConfigPanel, {
      props: {},
      global: {
        plugins: [store],
        stubs: {
          'tool-selector': true
        }
      }
    });
    
    // Try to save with empty form
    await wrapper.find('.save-button').trigger('click');
    
    // Check that validation errors are displayed
    expect(wrapper.find('.error-message').exists()).toBe(true);
    
    // Fill in the form
    await wrapper.find('#agent-name').setValue('Test Agent');
    await wrapper.find('#agent-description').setValue('Test Description');
    
    // Set selected tools via the v-model
    await wrapper.findComponent({ name: 'tool-selector' }).vm.$emit('update:modelValue', ['tool_1']);
    
    // Try to save again
    await wrapper.find('.save-button').trigger('click');
    
    // Check that the save event is emitted with the correct data
    expect(wrapper.emitted('save')).toBeTruthy();
    expect(wrapper.emitted('save')![0][0]).toEqual({
      name: 'Test Agent',
      description: 'Test Description',
      icon: 'robot',
      system_prompt: '',
      tools: ['tool_1']
    });
  });
  
  it('emits cancel event when cancel button is clicked', async () => {
    const store = createVuexStore();
    
    const wrapper = mount(AgentConfigPanel, {
      props: {},
      global: {
        plugins: [store],
        stubs: {
          'tool-selector': true
        }
      }
    });
    
    // Click the cancel button
    await wrapper.find('.cancel-button').trigger('click');
    
    // Check that the cancel event is emitted
    expect(wrapper.emitted('cancel')).toBeTruthy();
  });
  
  it('emits cancel event when close button is clicked', async () => {
    const store = createVuexStore();
    
    const wrapper = mount(AgentConfigPanel, {
      props: {},
      global: {
        plugins: [store],
        stubs: {
          'tool-selector': true
        }
      }
    });
    
    // Click the close button
    await wrapper.find('.close-button').trigger('click');
    
    // Check that the cancel event is emitted
    expect(wrapper.emitted('cancel')).toBeTruthy();
  });
});
