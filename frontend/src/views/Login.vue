<template>
    <div class="login">
      <h2>Login</h2>
      <form @submit.prevent="login">
        <input v-model="username" type="text" placeholder="Username" required />
        <input v-model="password" type="password" placeholder="Password" required />
        <button type="submit">Login</button>
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import { useRouter } from 'vue-router';
  import { useStore } from 'vuex';
  import AuthService from '../services/AuthService';
  
  export default defineComponent({
    name: 'Login',
    setup() {
      const username = ref('');
      const password = ref('');
      const router = useRouter();
      const store = useStore();
  
      const login = async () => {
        try {
          const { token, user } = await AuthService.login(username.value, password.value);
          AuthService.setToken(token);
          store.commit('setLoggedIn', true);
          store.commit('setUser', user);
          router.push('/');
        } catch (error) {
          console.error('Login failed:', error);
          // Handle login error (e.g., show error message)
        }
      };
  
      return { username, password, login };
    },
  });
  </script>
  
  <style scoped>
  .login {
    max-width: 300px;
    margin: 50px auto;
  }
  </style>