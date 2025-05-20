import { Module } from 'vuex';
import { RootState } from '../index';
import { Agent, CreateAgentRequest, UpdateAgentRequest } from '../../types/agent';
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
      
      // Save to localStorage for persistence
      if (id) {
        localStorage.setItem('selectedAgentId', id);
      } else {
        localStorage.removeItem('selectedAgentId');
      }
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
        localStorage.removeItem('selectedAgentId');
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
    },
    
    async createAgent({ commit }, agentData: CreateAgentRequest) {
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
    
    async updateAgent({ commit }, { id, agentData }: { id: string, agentData: UpdateAgentRequest }) {
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
    
    async deleteAgent({ commit }, id: string) {
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
