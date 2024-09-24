import { createStore } from 'vuex';

export default createStore({
  state: {
    isLoggedIn: false,
    user: null,
  },
  mutations: {
    setLoggedIn(state, value: boolean) {
      state.isLoggedIn = value;
    },
    setUser(state, user: any) {
      state.user = user;
    },
  },
  actions: {
    login({ commit }, { user }) { // Removed 'token' since it's unused
      commit('setLoggedIn', true);
      commit('setUser', user);
    },
    logout({ commit }) {
      commit('setLoggedIn', false);
      commit('setUser', null);
    },
  },
  getters: {
    isLoggedIn: (state) => state.isLoggedIn,
    user: (state) => state.user,
  },
});