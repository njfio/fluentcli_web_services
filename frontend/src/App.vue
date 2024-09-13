<template>
  <div id="app">
    <nav>
      <router-link to="/">Home</router-link> |
      <router-link to="/admin">Admin</router-link> |
      <router-link to="/studio">Studio</router-link> |
      <button @click="handleAuthAction">{{ authButtonText }}</button>
    </nav>
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import AuthService from './services/AuthService';

export default defineComponent({
  name: 'App',
  setup() {
    const store = useStore();
    const router = useRouter();

    const isLoggedIn = computed(() => store.state.isLoggedIn);
    const authButtonText = computed(() => (isLoggedIn.value ? 'Logout' : 'Login'));

    const handleAuthAction = () => {
      if (isLoggedIn.value) {
        // Logout
        AuthService.logout();
        store.commit('setLoggedIn', false);
        store.commit('setUser', null);
        router.push('/');
      } else {
        // Navigate to Login page
        router.push('/login');
      }
    };

    return {
      authButtonText,
      handleAuthAction,
    };
  },
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