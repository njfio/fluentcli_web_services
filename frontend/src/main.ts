// frontend/src/main.ts

import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import AuthService from '@/services/AuthService';
import 'highlight.js/styles/github.css';

const token = AuthService.getToken();
if (token) {
  AuthService.setToken(token);
  store.commit('setLoggedIn', true);
  // Optionally fetch user data here
}

createApp(App).use(router).use(store).mount('#app');