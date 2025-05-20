import { Module } from 'vuex';
import { RootState } from '../types';
import { Tool, ToolCall, ToolCallWithStatus } from '../../types/tool';
import { toolApiClient } from '../../services/toolApiClient';

export interface ToolState {
  tools: Tool[];
  loading: boolean;
  error: string | null;
  activeCalls: Record<string, ToolCallWithStatus>;
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
        state.activeCalls[id].status = status as 'pending' | 'running' | 'completed' | 'error';
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
    },

    clearToolCalls(state) {
      state.activeCalls = {};
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
    },

    getActiveToolCalls: (state) => {
      return Object.values(state.activeCalls);
    }
  }
};

export default toolModule;
