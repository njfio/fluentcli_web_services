// frontend/src/main.ts

import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import AuthService from '@/services/AuthService';

const token = AuthService.getToken();
if (token) {
  AuthService.setToken(token);
  store.commit('setLoggedIn', true);
  // Optionally fetch user data here
}

createApp(App).use(router).use(store).mount('#app');