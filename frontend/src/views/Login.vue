<template>
  <div v-if="!isLoading" class="login-container">
    <div class="studio-card login-card dark:bg-gray-800">
      <div class="mb-8 text-center">
        <svg class="mx-auto h-16 w-auto text-primary-600 dark:text-primary-400" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zm0-2a6 6 0 100-12 6 6 0 000 12zm-1-5a1 1 0 011-1h2a1 1 0 110 2h-2a1 1 0 01-1-1zm0-3a1 1 0 011-1h2a1 1 0 110 2h-2a1 1 0 01-1-1z"
            clip-rule="evenodd" />
        </svg>
        <h1 class="mt-4 text-4xl font-bold text-primary-600 dark:text-primary-400">FluentCLI Studio</h1>
      </div>
      <h2 class="text-2xl font-semibold mb-6 text-center text-gray-700 dark:text-gray-300">Login</h2>
      <form @submit.prevent="login" class="space-y-6">
        <div>
          <label for="username" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Username</label>
          <input id="username" v-model="username" type="text" placeholder="Enter your username" required
            class="mt-1 block w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-400 dark:focus:border-primary-400 text-gray-900 dark:text-white" />
        </div>
        <div>
          <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Password</label>
          <input id="password" v-model="password" type="password" placeholder="Enter your password" required
            class="mt-1 block w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-400 dark:focus:border-primary-400 text-gray-900 dark:text-white" />
        </div>
        <div v-if="error"
          class="bg-red-100 dark:bg-red-900 border-l-4 border-red-500 text-red-700 dark:text-red-300 p-4 rounded"
          role="alert">
          <p class="font-bold">Error</p>
          <p>{{ error }}</p>
        </div>
        <button type="submit"
          class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 dark:bg-primary-500 dark:hover:bg-primary-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-800"
          :disabled="isLoggingIn">
          <span v-if="isLoggingIn" class="flex items-center">
            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none"
              viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
              </path>
            </svg>
            Logging in...
          </span>
          <span v-else>Login</span>
        </button>
      </form>
    </div>
  </div>
  <div v-else class="flex justify-center items-center h-screen">
    <svg class="animate-spin h-10 w-10 text-primary-600" xmlns="http://www.w3.org/2000/svg" fill="none"
      viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
      </path>
    </svg>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useStore } from 'vuex';
import AuthService from '../services/AuthService';

const username = ref('');
const password = ref('');
const error = ref('');
const isLoggingIn = ref(false);
const isLoading = ref(true);
const router = useRouter();
const route = useRoute();
const store = useStore();

const isLoggedIn = computed(() => store.state.isLoggedIn);

onMounted(async () => {
  if (isLoggedIn.value) {
    router.push('/studio/dashboard');
  } else {
    isLoading.value = false;
  }
});

const login = async () => {
  error.value = '';
  isLoggingIn.value = true;

  try {
    const { token, user } = await AuthService.login(username.value, password.value);
    AuthService.setToken(token);
    store.commit('setLoggedIn', true);
    store.commit('setUser', user);
    const redirectPath = route.query.redirect as string || '/studio/dashboard';
    router.push(redirectPath);
  } catch (err: any) {
    console.error('Login failed:', err);
    error.value = err.message || 'Login failed. Please try again.';
  } finally {
    isLoggingIn.value = false;
  }
};
</script>

<style scoped>
.login-container {
  @apply min-h-screen flex items-center justify-center bg-gradient-to-r from-primary-500 to-primary-700 dark:from-gray-800 dark:to-gray-900;
  background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%239C92AC' fill-opacity='0.1'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
}

.login-card {
  @apply w-full max-w-md p-8 bg-white dark:bg-gray-800 rounded-lg shadow-xl;
  animation: fadeInUp 0.5s ease-out;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 640px) {
  .login-card {
    @apply max-w-sm mx-4;
  }
}
</style>