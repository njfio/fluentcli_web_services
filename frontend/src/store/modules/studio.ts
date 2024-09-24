import { Module } from 'vuex';
import { RootState } from '../types';

interface StudioState {
  openTabs: Array<{ id: string; name: string; type: string; data: any }>;
  activeTabId: string | null;
}

const studioModule: Module<StudioState, RootState> = {
  namespaced: true,
  state: {
    openTabs: [],
    activeTabId: null,
  },
  mutations: {
    addTab(state, tab) {
      state.openTabs.push(tab);
      state.activeTabId = tab.id;
    },
    closeTab(state, tabId) {
      const index = state.openTabs.findIndex((tab) => tab.id === tabId);
      if (index !== -1) {
        state.openTabs.splice(index, 1);
        if (state.activeTabId === tabId) {
          state.activeTabId = state.openTabs.length > 0 ? state.openTabs[0].id : null;
        }
      }
    },
    setActiveTab(state, tabId) {
      state.activeTabId = tabId;
    },
    updateTabData(state, { tabId, data }) {
      const tab = state.openTabs.find((tab) => tab.id === tabId);
      if (tab) {
        tab.data = data;
      }
    },
  },
  actions: {
    openTab({ commit, state }, { id, name, type, data }) {
      const existingTab = state.openTabs.find((tab) => tab.id === id);
      if (existingTab) {
        commit('setActiveTab', id);
      } else {
        commit('addTab', { id, name, type, data });
      }
    },
  },
  getters: {
    activeTab: (state) => state.openTabs.find((tab) => tab.id === state.activeTabId),
  },
};

export default studioModule;