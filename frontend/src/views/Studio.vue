<template>
  <div class="flex h-screen bg-gray-100">
    <StudioSidebar :isCollapsed="isSidebarCollapsed" @toggle="toggleSidebar" />
    <div class="flex-1 flex flex-col overflow-hidden">
      <StudioHeader @toggleSidebar="toggleSidebar" />
      <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gradient-to-br from-blue-50 to-indigo-100">
        <div class="container mx-auto px-6 py-8">
          <Suspense>
            <template #default>
              <router-view></router-view>
            </template>
            <template #fallback>
              <div class="flex justify-center items-center h-64">
                <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-indigo-500"></div>
              </div>
            </template>
          </Suspense>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, onErrorCaptured } from 'vue';
import { useRoute } from 'vue-router';
import { useStore } from 'vuex';
import StudioHeader from '@/components/studio/StudioHeader.vue';
import StudioSidebar from '@/components/studio/StudioSidebar.vue';

console.log('Studio component script starting');

const isSidebarCollapsed = ref(false);
const route = useRoute();
const store = useStore();

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value;
};

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

<style scoped>
/* Add any component-specific styles here */
</style>