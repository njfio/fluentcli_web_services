<template>
  <div id="app">
    <nav>
      <router-link to="/">Home</router-link> |
      <router-link to="/admin">Admin</router-link> |
      <router-link to="/studio">Studio</router-link> |
      <a href="#" @click="logout" v-if="isAuthenticated">Logout</a>
      <router-link to="/login" v-else>Login</router-link>
    </nav>
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';
import { useRouter } from 'vue-router';
import AuthService from './services/AuthService';

export default defineComponent({
  name: 'App',
  setup() {
    const router = useRouter();
    const isAuthenticated = computed(() => !!AuthService.getToken());

    const logout = () => {
      AuthService.logout();
      router.push('/login');
    };

    return { isAuthenticated, logout };
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