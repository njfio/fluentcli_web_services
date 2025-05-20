import { mount } from '@vue/test-utils';
import ToolCallDisplay from '@/components/ToolCallDisplay.vue';

describe('ToolCallDisplay.vue', () => {
  it('renders correctly with completed status', () => {
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
      },
      global: {
        stubs: {
          'tool-result-display': true
        }
      }
    });
    
    // Check that the component renders correctly
    expect(wrapper.find('.tool-name').text()).toBe('get_weather');
    expect(wrapper.find('.tool-status').text()).toBe('Completed');
    expect(wrapper.find('.tool-status').classes()).toContain('status-completed');
    
    // Check that the arguments are displayed
    expect(wrapper.find('.tool-call-arguments').exists()).toBe(true);
    
    // Check that the result is displayed
    expect(wrapper.find('.tool-call-result').exists()).toBe(true);
  });
  
  it('renders correctly with error status', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' },
      status: 'error',
      error: 'Failed to get weather data'
    };
    
    const wrapper = mount(ToolCallDisplay, {
      props: {
        toolCall: mockToolCall
      },
      global: {
        stubs: {
          'tool-result-display': true
        }
      }
    });
    
    // Check that the component renders correctly
    expect(wrapper.find('.tool-name').text()).toBe('get_weather');
    expect(wrapper.find('.tool-status').text()).toBe('Error');
    expect(wrapper.find('.tool-status').classes()).toContain('status-error');
    
    // Check that the arguments are displayed
    expect(wrapper.find('.tool-call-arguments').exists()).toBe(true);
    
    // Check that the error is displayed
    expect(wrapper.find('.tool-call-error').exists()).toBe(true);
    expect(wrapper.find('.error-message').text()).toBe('Failed to get weather data');
    
    // Check that the retry button is displayed
    expect(wrapper.find('.retry-button').exists()).toBe(true);
  });
  
  it('emits retry event when retry button is clicked', async () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' },
      status: 'error',
      error: 'Failed to get weather data'
    };
    
    const wrapper = mount(ToolCallDisplay, {
      props: {
        toolCall: mockToolCall
      },
      global: {
        stubs: {
          'tool-result-display': true
        }
      }
    });
    
    // Click the retry button
    await wrapper.find('.retry-button').trigger('click');
    
    // Check that the retry event is emitted
    expect(wrapper.emitted('retry')).toBeTruthy();
    expect(wrapper.emitted('retry')?.length).toBe(1);
  });
  
  it('toggles expanded state when expand button is clicked', async () => {
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
      },
      global: {
        stubs: {
          'tool-result-display': true
        }
      }
    });
    
    // Initially expanded
    expect(wrapper.find('.tool-call-details').exists()).toBe(true);
    
    // Click the expand button to collapse
    await wrapper.find('.expand-button').trigger('click');
    
    // Check that it's collapsed
    expect(wrapper.find('.tool-call-details').exists()).toBe(false);
    
    // Click the expand button again to expand
    await wrapper.find('.expand-button').trigger('click');
    
    // Check that it's expanded again
    expect(wrapper.find('.tool-call-details').exists()).toBe(true);
  });
});
