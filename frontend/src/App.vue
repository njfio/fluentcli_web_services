<template>
  <div id="app">
    <router-view></router-view>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useStore } from 'vuex';
import AuthService from './services/AuthService';

const router = useRouter();
const store = useStore();

onMounted(async () => {
  const token = AuthService.getToken();
  if (token) {
    try {
      const user = await AuthService.validateToken(token);
      store.commit('setLoggedIn', true);
      store.commit('setUser', user);
      if (router.currentRoute.value.path === '/login') {
        router.push('/studio/dashboard');
      }
    } catch (error) {
      console.error('Token validation failed:', error);
      AuthService.removeToken();
      store.commit('setLoggedIn', false);
      store.commit('setUser', null);
      router.push('/login');
    }
  } else {
    router.push('/login');
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

nav {
  padding: 30px;
}

nav a {
  font-weight: bold;
  color: #2c3e50;
}

nav a.router-link-exact-active {
  color: #42b983;
}
</style>