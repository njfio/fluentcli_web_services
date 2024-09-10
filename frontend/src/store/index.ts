import { createStore, CommitOptions } from 'vuex';

export interface State {
  exampleProperty: string;
}

export interface Mutations {
  setExampleProperty(state: State, payload: string): void;
}

export interface Actions {
  updateExampleProperty({ commit }: { commit: Commit }, payload: string): void;
}

type Commit = (type: string, payload?: any, options?: CommitOptions) => void;

export default createStore<State>({
  state: {
    exampleProperty: '',
  },
  mutations: {
    setExampleProperty(state, payload: string) {
      state.exampleProperty = payload;
    },
  },
  actions: {
    updateExampleProperty({ commit }, payload: string) {
      commit('setExampleProperty', payload);
    },
  },
  modules: {
  },
});