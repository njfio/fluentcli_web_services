<template>
  <Suspense>
    <template #default>
      <router-view></router-view>
    </template>
    <template #fallback>
      <div class="flex justify-center items-center h-64">
        <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-primary-600"></div>
      </div>
    </template>
  </Suspense>
</template>

<script setup lang="ts">
import { onMounted, onErrorCaptured } from 'vue';
import { useRoute } from 'vue-router';
import { useStore } from 'vuex';

console.log('Studio component script starting');

const route = useRoute();
const store = useStore();

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