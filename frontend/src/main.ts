// frontend/src/main.ts
import './assets/tailwind.css'
import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import AuthService from './services/AuthService';
import 'highlight.js/styles/github.css';

async function initApp() {
  console.log('Initializing app...');
  try {
    console.log('Initializing auth state...');
    await AuthService.initializeAuthState();
    console.log('Auth state initialized');

    console.log('Creating Vue app...');
    const app = createApp(App);
    app.use(router);
    app.use(store);

    console.log('Mounting app...');
    app.mount('#app');
    console.log('App mounted');
  } catch (error) {
    console.error('Error initializing app:', error);
  }
}

console.log('Starting app initialization');
initApp().then(() => {
  console.log('App initialization complete');
}).catch((error) => {
  console.error('App initialization failed:', error);
});
