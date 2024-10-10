import { createStore } from 'vuex';
import studioModule from './modules/studio';
import themeModule from './modules/theme';
import chatModule from './modules/chat';

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
    login({ commit }, { user }) {
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
  modules: {
    studio: studioModule,
    theme: themeModule,
    chat: chatModule,
  },
});