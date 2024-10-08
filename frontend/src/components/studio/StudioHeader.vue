<template>
  <header class="bg-gradient-to-r from-indigo-600 to-indigo-800 shadow-md">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center py-4">
        <div class="flex items-center">
          <button @click="$emit('toggleSidebar')" class="text-white mr-4 focus:outline-none">
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <h2 class="text-2xl font-semibold text-white">{{ pageTitle }}</h2>
        </div>
        <div class="flex items-center">
          <div class="relative" v-if="user">
            <button @click="toggleUserMenu" class="flex items-center text-white focus:outline-none">
              <img :src="user.avatar || 'https://www.gravatar.com/avatar/?d=mp'" alt="User avatar" class="h-8 w-8 rounded-full mr-2">
              <span class="mr-1">{{ user.name }}</span>
              <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            <div v-if="showUserMenu" class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg py-1 z-10">
              <a href="#" class="block px-4 py-2 text-sm text-gray-700 hover:bg-indigo-500 hover:text-white">Profile</a>
              <a href="#" class="block px-4 py-2 text-sm text-gray-700 hover:bg-indigo-500 hover:text-white">Settings</a>
              <a @click="logout" class="block px-4 py-2 text-sm text-gray-700 hover:bg-indigo-500 hover:text-white cursor-pointer">Logout</a>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-2">
      <nav class="text-white text-sm" aria-label="Breadcrumb">
        <ol class="list-none p-0 inline-flex">
          <li class="flex items-center">
            <a href="#" class="hover:text-indigo-200">Home</a>
            <svg class="fill-current w-3 h-3 mx-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512">
              <path d="M285.476 272.971L91.132 467.314c-9.373 9.373-24.569 9.373-33.941 0l-22.667-22.667c-9.357-9.357-9.375-24.522-.04-33.901L188.505 256 34.484 101.255c-9.335-9.379-9.317-24.544.04-33.901l22.667-22.667c9.373-9.373 24.569-9.373 33.941 0L285.475 239.03c9.373 9.372 9.373 24.568.001 33.941z"/>
            </svg>
          </li>
          <li class="flex items-center">
            <a href="#" class="hover:text-indigo-200">{{ pageTitle }}</a>
          </li>
        </ol>
      </nav>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useStore } from 'vuex';

const route = useRoute();
const router = useRouter();
const store = useStore();

const pageTitle = computed(() => {
  const routeName = route.name as string;
  return routeName.charAt(0).toUpperCase() + routeName.slice(1);
});

const user = computed(() => store.state.user);
const showUserMenu = ref(false);

const toggleUserMenu = () => {
  showUserMenu.value = !showUserMenu.value;
};

const logout = () => {
  store.dispatch('logout');
  router.push('/login');
};

// Close user menu when clicking outside
const closeUserMenu = (e: MouseEvent) => {
  if (showUserMenu.value && !(e.target as HTMLElement).closest('.user-menu')) {
    showUserMenu.value = false;
  }
};

// Add event listener for closing user menu
if (typeof window !== 'undefined') {
  window.addEventListener('click', closeUserMenu);
}
</script>

<style scoped>
/* Add any component-specific styles here */
</style>