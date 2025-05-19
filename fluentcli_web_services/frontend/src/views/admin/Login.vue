<template>
  <div class="login">
    <h2>Login</h2>
    <form @submit.prevent="login">
      <input v-model="username" type="text" placeholder="Username" required />
      <input v-model="password" type="password" placeholder="Password" required />
      <button type="submit" :disabled="isLoading">
        {{ isLoading ? 'Logging in...' : 'Login' }}
      </button>
    </form>
    <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useStore } from 'vuex';
import AuthService from '@/services/AuthService';

export default defineComponent({
  name: 'Login',
  setup() {
    const username = ref('');
    const password = ref('');
    const errorMessage = ref('');
    const isLoading = ref(false);
    const router = useRouter();
    const store = useStore();

    const login = async () => {
      errorMessage.value = '';
      isLoading.value = true;
      try {
        const { token, user } = await AuthService.login(username.value, password.value);
        AuthService.setToken(token);
        store.commit('setLoggedIn', true);
        store.commit('setUser', user);
        router.push('/');
      } catch (error: any) {
        console.error('Login failed:', error);
        errorMessage.value = error.response?.data?.message || 'Login failed. Please try again.';
      } finally {
        isLoading.value = false;
      }
    };

    return { username, password, login, errorMessage, isLoading };
  },
});
</script>

<style scoped>
.login {
  max-width: 300px;
  margin: 50px auto;
  padding: 20px;
  border: 1px solid #bdc3c7;
  border-radius: 5px;
  background-color: #fff;
}

.login h2 {
  text-align: center;
  margin-bottom: 20px;
}

.login form {
  display: flex;
  flex-direction: column;
}

.login input {
  padding: 10px;
  margin-bottom: 15px;
  border: 1px solid #bdc3c7;
  border-radius: 3px;
}

.login button {
  padding: 10px;
  background-color: #2c3e50;
  color: #ecf0f1;
  border: none;
  border-radius: 3px;
  cursor: pointer;
}

.login button:disabled {
  background-color: #95a5a6;
  cursor: not-allowed;
}

.error {
  color: red;
  text-align: center;
  margin-top: 10px;
}
</style>