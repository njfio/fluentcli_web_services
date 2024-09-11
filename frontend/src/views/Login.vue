<template>
    <div class="login">
      <h2>Login</h2>
      <form @submit.prevent="login">
        <input v-model="username" type="text" placeholder="Username" required>
        <input v-model="password" type="password" placeholder="Password" required>
        <button type="submit">Login</button>
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import { useRouter } from 'vue-router';
  import AuthService from '../services/AuthService';
  
  export default defineComponent({
    name: 'Login',
    setup() {
      const username = ref('');
      const password = ref('');
      const router = useRouter();
  
      const login = async () => {
        try {
          const token = await AuthService.login(username.value, password.value);
          localStorage.setItem('token', token);
          router.push('/');
        } catch (error) {
          console.error('Login failed:', error);
          // Handle login error (e.g., show error message)
        }
      };
  
      return { username, password, login };
    }
  });
  </script>