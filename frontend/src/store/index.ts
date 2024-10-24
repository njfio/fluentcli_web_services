import { createStore } from 'vuex';
import studioModule from './modules/studio';
import themeModule from './modules/theme';
import chatModule from './modules/chat';
import chatUIModule from './modules/chatUI';
import { RootState, User } from './types';

export default createStore<RootState>({
  state: {
    auth: {
      user: null,
      isAuthenticated: false,
    },
    chat: {
      isExpanded: false,
      isSidebarOpen: true,
    },
  },
  mutations: {
    setUser(state, user: User | null) {
      console.log('setUser mutation called with user:', user);
      state.auth.user = user;
      state.auth.isAuthenticated = !!user;
      console.log('New auth state:', state.auth);
    },
    setLoggedIn(state, value: boolean) {
      console.log('setLoggedIn mutation called with value:', value);
      state.auth.isAuthenticated = value;
      console.log('New auth state:', state.auth);
    },
  },
  actions: {
    login({ commit }, { user }: { user: User }) {
      console.log('login action called with user:', user);
      commit('setUser', user);
      commit('setLoggedIn', true);
    },
    logout({ commit }) {
      console.log('logout action called');
      commit('setUser', null);
      commit('setLoggedIn', false);
    },
  },
  getters: {
    isAuthenticated: (state) => {
      console.log('isAuthenticated getter called, returning:', state.auth.isAuthenticated);
      return state.auth.isAuthenticated;
    },
    user: (state) => {
      console.log('user getter called, returning:', state.auth.user);
      return state.auth.user;
    },
    userId: (state) => {
      console.log('userId getter called, returning:', state.auth.user?.user_id);
      return state.auth.user?.user_id;
    },
  },
  modules: {
    studio: studioModule,
    theme: themeModule,
    chat: chatModule,
    chatUI: chatUIModule,
  },
});
