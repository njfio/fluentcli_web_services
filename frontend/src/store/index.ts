import { createStore } from 'vuex';

export interface State {
  // Define your state properties here
  exampleProperty: string;
}

export default createStore<State>({
  state: {
    exampleProperty: '',
  },
  mutations: {
    // Define your mutations here
    setExampleProperty(state, payload: string) {
      state.exampleProperty = payload;
    },
  },
  actions: {
    // Define your actions here
    updateExampleProperty({ commit }, payload: string) {
      commit('setExampleProperty', payload);
    },
  },
  modules: {
    // Define your modules here
  },
});