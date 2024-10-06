<template>
  <div id="app" class="min-h-screen bg-gray-100 text-gray-900 flex flex-col">
    <header class="bg-white shadow-sm">
      <nav class="container mx-auto px-4 py-3 flex justify-between items-center">
        <router-link to="/" class="text-xl font-bold text-indigo-600">FluentCLI Web Services</router-link>
        <div v-if="isLoggedIn">
          <router-link to="/studio/dashboard" class="text-gray-600 hover:text-indigo-600 mr-4">Studio</router-link>
          <button @click="logout" class="text-gray-600 hover:text-indigo-600">Logout</button>
        </div>
      </nav>
    </header>
    <main class="flex-grow container mx-auto px-4 py-8">
      <router-view v-if="isInitialized"></router-view>
      <div v-else class="text-center py-8">Loading...</div>
    </main>
    <footer class="bg-white border-t">
      <div class="container mx-auto px-4 py-4 text-center text-gray-600">
        &copy; 2023 FluentCLI Web Services. All rights reserved.
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import AuthService from './services/AuthService';

const store = useStore();
const router = useRouter();
const isInitialized = ref(false);

const isLoggedIn = computed(() => store.state.isLoggedIn);

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
  isInitialized.value = true;
});

const logout = () => {
  AuthService.removeToken();
  store.commit('setLoggedIn', false);
  store.commit('setUser', null);
  router.push('/login');
};
</script>

<style>
#app {
  font-family: 'Inter', sans-serif;
}
</style>