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
    
    // Reset mocks
    jest.clearAllMocks();
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
  
  it('should set tool call error', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    const mockError = 'Failed to get weather data';
    
    store.commit('tool/addToolCall', mockToolCall);
    store.commit('tool/setToolCallError', { id: 'call_123', error: mockError });
    
    expect(store.state.tool.activeCalls['call_123'].status).toBe('error');
    expect(store.state.tool.activeCalls['call_123'].error).toEqual(mockError);
  });
  
  it('should clear tool calls', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    store.commit('tool/addToolCall', mockToolCall);
    store.commit('tool/clearToolCalls');
    
    expect(store.state.tool.activeCalls).toEqual({});
  });
  
  it('should fetch tools', async () => {
    const mockTools = [
      { id: '1', name: 'get_weather', description: 'Get weather information' },
      { id: '2', name: 'search_web', description: 'Search the web' }
    ];
    
    (toolApiClient.listTools as jest.Mock).mockResolvedValue(mockTools);
    
    await store.dispatch('tool/fetchTools');
    
    expect(toolApiClient.listTools).toHaveBeenCalled();
    expect(store.state.tool.tools).toEqual(mockTools);
    expect(store.state.tool.loading).toBe(false);
    expect(store.state.tool.error).toBe(null);
  });
  
  it('should handle fetch tools error', async () => {
    const mockError = new Error('Failed to fetch tools');
    
    (toolApiClient.listTools as jest.Mock).mockRejectedValue(mockError);
    
    await store.dispatch('tool/fetchTools');
    
    expect(toolApiClient.listTools).toHaveBeenCalled();
    expect(store.state.tool.tools).toEqual([]);
    expect(store.state.tool.loading).toBe(false);
    expect(store.state.tool.error).toBe('Failed to fetch tools');
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
    
    expect(toolApiClient.executeToolCall).toHaveBeenCalledWith(mockToolCall);
    expect(store.state.tool.activeCalls['call_123'].status).toBe('completed');
    expect(store.state.tool.activeCalls['call_123'].result).toEqual(mockResult.result);
  });
  
  it('should handle execute tool call error', async () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    const mockError = new Error('Failed to execute tool call');
    
    (toolApiClient.executeToolCall as jest.Mock).mockRejectedValue(mockError);
    
    try {
      await store.dispatch('tool/executeToolCall', mockToolCall);
    } catch (error) {
      // Expected to throw
    }
    
    expect(toolApiClient.executeToolCall).toHaveBeenCalledWith(mockToolCall);
    expect(store.state.tool.activeCalls['call_123'].status).toBe('error');
    expect(store.state.tool.activeCalls['call_123'].error).toBe('Failed to execute tool call');
  });
  
  it('should get tool by ID', () => {
    const mockTools = [
      { id: '1', name: 'get_weather', description: 'Get weather information' },
      { id: '2', name: 'search_web', description: 'Search the web' }
    ];
    
    store.commit('tool/setTools', mockTools);
    
    const tool = store.getters['tool/getToolById']('1');
    
    expect(tool).toEqual(mockTools[0]);
  });
  
  it('should get tool call by ID', () => {
    const mockToolCall = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    store.commit('tool/addToolCall', mockToolCall);
    
    const toolCall = store.getters['tool/getToolCallById']('call_123');
    
    expect(toolCall).toEqual({
      ...mockToolCall,
      status: 'pending'
    });
  });
  
  it('should get active tool calls', () => {
    const mockToolCall1 = {
      id: 'call_123',
      name: 'get_weather',
      arguments: { location: 'Paris' }
    };
    
    const mockToolCall2 = {
      id: 'call_456',
      name: 'search_web',
      arguments: { query: 'weather in Paris' }
    };
    
    store.commit('tool/addToolCall', mockToolCall1);
    store.commit('tool/addToolCall', mockToolCall2);
    
    const activeToolCalls = store.getters['tool/getActiveToolCalls'];
    
    expect(activeToolCalls).toHaveLength(2);
    expect(activeToolCalls[0].id).toBe('call_123');
    expect(activeToolCalls[1].id).toBe('call_456');
  });
});
