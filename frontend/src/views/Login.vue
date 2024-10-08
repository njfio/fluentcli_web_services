<template>
  <div class="login-container">
    <div class="studio-card login-card">
      <h2 class="text-2xl font-bold mb-6 text-center text-primary-color">Login to FluentCLI Studio</h2>
      <form @submit.prevent="login" class="space-y-4">
        <div>
          <label for="username" class="studio-label">Username</label>
          <input
            id="username"
            v-model="username"
            type="text"
            placeholder="Enter your username"
            required
            class="studio-input"
          />
        </div>
        <div>
          <label for="password" class="studio-label">Password</label>
          <input
            id="password"
            v-model="password"
            type="password"
            placeholder="Enter your password"
            required
            class="studio-input"
          />
        </div>
        <div v-if="error" class="studio-error" role="alert">
          {{ error }}
        </div>
        <button
          type="submit"
          class="studio-button studio-button-primary w-full"
          :disabled="isLoading"
        >
          <span v-if="isLoading" class="flex items-center justify-center">
            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Logging in...
          </span>
          <span v-else>Login</span>
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useStore } from 'vuex';
import AuthService from '../services/AuthService';

const username = ref('');
const password = ref('');
const error = ref('');
const isLoading = ref(false);
const router = useRouter();
const store = useStore();

const login = async () => {
  error.value = '';
  isLoading.value = true;

  try {
    const { token, user } = await AuthService.login(username.value, password.value);
    AuthService.setToken(token);
    store.commit('setLoggedIn', true);
    store.commit('setUser', user);
    router.push('/studio/dashboard');
  } catch (err: any) {
    console.error('Login failed:', err);
    error.value = err.message || 'Login failed. Please try again.';
  } finally {
    isLoading.value = false;
  }
};
</script>

<style scoped>
.login-container {
  @apply min-h-screen flex items-center justify-center bg-gradient-to-r from-indigo-500 to-purple-600;
}

.login-card {
  @apply w-full max-w-md p-8 bg-white rounded-lg shadow-xl;
}

.login-card,
.studio-input,
.studio-button {
  @apply transition-all duration-300 ease-in-out;
}

.studio-input:focus {
  @apply transform scale-105;
}

.studio-button:not(:disabled):hover {
  @apply transform scale-105;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}

.login-card {
  animation: fadeIn 0.5s ease-out;
}
</style>