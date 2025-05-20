import { createStore } from 'vuex';
import agentModule from '@/store/modules/agent';
import { agentApiClient } from '@/services/agentApiClient';

// Mock the API client
jest.mock('@/services/agentApiClient');

// Mock localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: (key: string) => store[key] || null,
    setItem: (key: string, value: string) => {
      store[key] = value.toString();
    },
    removeItem: (key: string) => {
      delete store[key];
    },
    clear: () => {
      store = {};
    }
  };
})();
Object.defineProperty(window, 'localStorage', { value: localStorageMock });

describe('Agent Store Module', () => {
  let store: any;
  
  beforeEach(() => {
    store = createStore({
      modules: {
        agent: agentModule
      }
    });
    
    // Reset mocks
    jest.clearAllMocks();
    localStorageMock.clear();
  });
  
  it('should set agents', () => {
    const mockAgents = [
      { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] },
      { id: '2', name: 'Search Agent', description: 'Search the web', tools: ['2'] }
    ];
    
    store.commit('agent/setAgents', mockAgents);
    
    expect(store.state.agent.agents).toEqual(mockAgents);
  });
  
  it('should set selected agent ID', () => {
    store.commit('agent/setSelectedAgentId', '1');
    
    expect(store.state.agent.selectedAgentId).toBe('1');
    expect(localStorageMock.getItem('selectedAgentId')).toBe('1');
  });
  
  it('should clear selected agent ID when set to null', () => {
    store.commit('agent/setSelectedAgentId', '1');
    store.commit('agent/setSelectedAgentId', null);
    
    expect(store.state.agent.selectedAgentId).toBe(null);
    expect(localStorageMock.getItem('selectedAgentId')).toBe(null);
  });
  
  it('should add an agent', () => {
    const mockAgent = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    
    store.commit('agent/addAgent', mockAgent);
    
    expect(store.state.agent.agents).toEqual([mockAgent]);
  });
  
  it('should update an agent', () => {
    const mockAgent = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    const updatedAgent = { id: '1', name: 'Updated Agent', description: 'Updated description', tools: ['1', '2'] };
    
    store.commit('agent/addAgent', mockAgent);
    store.commit('agent/updateAgent', updatedAgent);
    
    expect(store.state.agent.agents[0]).toEqual(updatedAgent);
  });
  
  it('should remove an agent', () => {
    const mockAgent1 = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    const mockAgent2 = { id: '2', name: 'Search Agent', description: 'Search the web', tools: ['2'] };
    
    store.commit('agent/addAgent', mockAgent1);
    store.commit('agent/addAgent', mockAgent2);
    store.commit('agent/removeAgent', '1');
    
    expect(store.state.agent.agents).toEqual([mockAgent2]);
  });
  
  it('should clear selected agent ID when removing the selected agent', () => {
    const mockAgent = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    
    store.commit('agent/addAgent', mockAgent);
    store.commit('agent/setSelectedAgentId', '1');
    store.commit('agent/removeAgent', '1');
    
    expect(store.state.agent.selectedAgentId).toBe(null);
    expect(localStorageMock.getItem('selectedAgentId')).toBe(null);
  });
  
  it('should fetch agents', async () => {
    const mockAgents = [
      { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] },
      { id: '2', name: 'Search Agent', description: 'Search the web', tools: ['2'] }
    ];
    
    (agentApiClient.listAgents as jest.Mock).mockResolvedValue(mockAgents);
    
    await store.dispatch('agent/fetchAgents');
    
    expect(agentApiClient.listAgents).toHaveBeenCalled();
    expect(store.state.agent.agents).toEqual(mockAgents);
    expect(store.state.agent.loading).toBe(false);
    expect(store.state.agent.error).toBe(null);
  });
  
  it('should handle fetch agents error', async () => {
    const mockError = new Error('Failed to fetch agents');
    
    (agentApiClient.listAgents as jest.Mock).mockRejectedValue(mockError);
    
    await store.dispatch('agent/fetchAgents');
    
    expect(agentApiClient.listAgents).toHaveBeenCalled();
    expect(store.state.agent.agents).toEqual([]);
    expect(store.state.agent.loading).toBe(false);
    expect(store.state.agent.error).toBe('Failed to fetch agents');
  });
  
  it('should select an agent', () => {
    store.dispatch('agent/selectAgent', '1');
    
    expect(store.state.agent.selectedAgentId).toBe('1');
    expect(localStorageMock.getItem('selectedAgentId')).toBe('1');
  });
  
  it('should create an agent', async () => {
    const mockAgentData = { name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    const mockCreatedAgent = { id: '1', ...mockAgentData, created_at: '2023-01-01T00:00:00Z', updated_at: '2023-01-01T00:00:00Z' };
    
    (agentApiClient.createAgent as jest.Mock).mockResolvedValue(mockCreatedAgent);
    
    await store.dispatch('agent/createAgent', mockAgentData);
    
    expect(agentApiClient.createAgent).toHaveBeenCalledWith(mockAgentData);
    expect(store.state.agent.agents[0]).toEqual(mockCreatedAgent);
  });
  
  it('should update an agent', async () => {
    const mockAgent = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    const mockUpdateData = { name: 'Updated Agent', description: 'Updated description', tools: ['1', '2'] };
    const mockUpdatedAgent = { ...mockAgent, ...mockUpdateData, updated_at: '2023-01-02T00:00:00Z' };
    
    store.commit('agent/addAgent', mockAgent);
    
    (agentApiClient.updateAgent as jest.Mock).mockResolvedValue(mockUpdatedAgent);
    
    await store.dispatch('agent/updateAgent', { id: '1', agentData: mockUpdateData });
    
    expect(agentApiClient.updateAgent).toHaveBeenCalledWith('1', mockUpdateData);
    expect(store.state.agent.agents[0]).toEqual(mockUpdatedAgent);
  });
  
  it('should delete an agent', async () => {
    const mockAgent = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    
    store.commit('agent/addAgent', mockAgent);
    
    (agentApiClient.deleteAgent as jest.Mock).mockResolvedValue(undefined);
    
    await store.dispatch('agent/deleteAgent', '1');
    
    expect(agentApiClient.deleteAgent).toHaveBeenCalledWith('1');
    expect(store.state.agent.agents).toEqual([]);
  });
  
  it('should initialize from localStorage', () => {
    localStorageMock.setItem('selectedAgentId', '1');
    
    store.dispatch('agent/initializeFromLocalStorage');
    
    expect(store.state.agent.selectedAgentId).toBe('1');
  });
  
  it('should get selected agent', () => {
    const mockAgent = { id: '1', name: 'Weather Agent', description: 'Get weather information', tools: ['1'] };
    
    store.commit('agent/addAgent', mockAgent);
    store.commit('agent/setSelectedAgentId', '1');
    
    const selectedAgent = store.getters['agent/selectedAgent'];
    
    expect(selectedAgent).toEqual(mockAgent);
  });
  
  it('should return null for selected agent when none is selected', () => {
    const selectedAgent = store.getters['agent/selectedAgent'];
    
    expect(selectedAgent).toBe(null);
  });
});
