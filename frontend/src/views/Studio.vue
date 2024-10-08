<template>
  <div class="flex h-screen bg-gray-100">
    <nav class="w-64 bg-white shadow-lg" :class="{ 'w-20': isSidebarCollapsed }">
      <div class="p-4 flex justify-between items-center">
        <h1 class="text-xl font-bold text-indigo-600" v-if="!isSidebarCollapsed">FluentCLI Studio</h1>
        <button @click="toggleSidebar" class="text-gray-500 hover:text-indigo-600">
          <i :class="isSidebarCollapsed ? 'fas fa-bars' : 'fas fa-times'"></i>
        </button>
      </div>
      <ul class="mt-6">
        <li v-for="(link, index) in sidebarLinks" :key="index" class="mb-2">
          <router-link :to="link.to" class="flex items-center p-3 text-gray-600 hover:bg-indigo-50 hover:text-indigo-600 rounded-lg transition-colors duration-200" @click="logRouteChange(link.to)">
            <i :class="link.icon + ' w-5 h-5'" :title="link.title"></i>
            <span v-if="!isSidebarCollapsed" class="ml-3">{{ link.title }}</span>
          </router-link>
        </li>
      </ul>
    </nav>
    <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-100">
      <StudioHeader @toggleSidebar="toggleSidebar" />
      <div class="p-6">
        <Suspense>
          <template #default>
            <router-view></router-view>
          </template>
          <template #fallback>
            <div>Loading...</div>
          </template>
        </Suspense>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, onErrorCaptured } from 'vue';
import { useRoute } from 'vue-router';
import { useStore } from 'vuex';
import StudioHeader from '@/components/studio/StudioHeader.vue';

console.log('Studio component script starting');

const isSidebarCollapsed = ref(false);
const route = useRoute();
const store = useStore();

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value;
};

const logRouteChange = (to: string) => {
  console.log(`Navigating to: ${to}`);
};

const sidebarLinks = [
  { to: '/studio/dashboard', icon: 'fas fa-tachometer-alt', title: 'Dashboard' },
  { to: '/studio/jobs', icon: 'fas fa-tasks', title: 'Jobs' },
  { to: '/studio/pipelines', icon: 'fas fa-project-diagram', title: 'Pipelines' },
  { to: '/studio/dockerfiles', icon: 'fab fa-docker', title: 'Docker Files' },
  { to: '/studio/configurations', icon: 'fas fa-cogs', title: 'Configurations' },
  { to: '/studio/amberstores', icon: 'fas fa-database', title: 'Amber Stores' },
  { to: '/studio/statefiles', icon: 'fas fa-file-alt', title: 'State Files' },
];

onMounted(async () => {
  console.log('Studio component mounted');
  console.log('Current route:', route.path);
  try {
    await store.dispatch('studio/fetchAllData');
    console.log('All data fetched successfully');
  } catch (error) {
    console.error('Error fetching all data:', error);
  }
});

watch(() => route.path, (newPath) => {
  console.log('Route changed to:', newPath);
});

onErrorCaptured((error, component, info) => {
  console.error('Error captured in Studio component:', error);
  console.error('Component:', component);
  console.error('Error Info:', info);
  return false; // Prevent the error from propagating further
});
</script>