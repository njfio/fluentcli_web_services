<template>
  <div :class="{ 'dark': isDarkMode }">
    <router-view v-if="isAuthInitialized" v-slot="{ Component }">
      <component :is="Component" />
    </router-view>
    <div v-else class="flex justify-center items-center h-screen">
      <svg class="animate-spin h-10 w-10 text-primary-600" xmlns="http://www.w3.org/2000/svg" fill="none"
        viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
        </path>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import AuthService from './services/AuthService';

const store = useStore();
const router = useRouter();
const isAuthInitialized = ref(false);

const isDarkMode = computed(() => store.getters['theme/isDarkMode']);

watch(isDarkMode, (newValue) => {
  if (newValue) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}, { immediate: true });

const initializeAuthState = async () => {
  const token = AuthService.getToken();
  if (token) {
    try {
      const user = await AuthService.validateToken(token);
      store.commit('setLoggedIn', true);
      store.commit('setUser', user);
      AuthService.setToken(token); // Ensure the token is set in axios headers
    } catch (error) {
      console.error('Invalid token:', error);
      AuthService.removeToken();
      store.commit('setLoggedIn', false);
      store.commit('setUser', null);
      router.push('/login');
    }
  }
  isAuthInitialized.value = true;
};

onMounted(async () => {
  store.dispatch('theme/initDarkMode');
  await initializeAuthState();
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  @apply text-gray-900 dark:text-white;
}

body {
  @apply bg-white dark:bg-gray-900;
  margin: 0;
  padding: 0;
}

.dark {
  color-scheme: dark;
}
</style>