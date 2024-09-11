<script setup lang="ts">
import axios from 'axios';
import { ref, onMounted } from 'vue';

interface WorkerType {
  id: number;
  name: string;
  // ... other properties
}

const workerTypes = ref<WorkerType[]>([]);

onMounted(async () => {
  try {
    const response = await axios.get('/worker-types'); // Replace with your actual API endpoint
    workerTypes.value = response.data;
  } catch (error) {
    console.error(error);
    // Handle error, e.g., display an error message to the user
  }
});
</script>

<template>
  <div>
    <h1>Worker Types</h1>
    <ul>
      <li v-for="workerType in workerTypes" :key="workerType.id">
        {{ workerType.name }}
      </li>
    </ul>
  </div>
</template>