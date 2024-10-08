<template>
  <StudioLayout>
    <router-view></router-view>
  </StudioLayout>
</template>

<script setup lang="ts">
import { onMounted, onErrorCaptured } from 'vue';
import { useRoute } from 'vue-router';
import { useStore } from 'vuex';
import StudioLayout from '@/components/studio/StudioLayout.vue';

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