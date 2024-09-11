<script setup lang="ts">
import axios from 'axios';
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';

interface Job {
  id: number;
  uri: string;
  config: any; // Replace with the actual type of your job config
  worker_type: string;
  // ... other properties
}

const route = useRoute();
const job = ref<Job | null>(null);

onMounted(async () => {
  try {
    const response = await axios.get(`/jobs/${route.params.id}`);
    job.value = response.data;
  } catch (error) {
    console.error(error);
    // Handle error, e.g., display an error message to the user
  }
});
</script>

<template>
  <div v-if="job">
    <h1>Job Details</h1>
    <p>URI: {{ job.uri }}</p>
    <p>Config: {{ job.config }}</p>
    <p>Worker Type: {{ job.worker_type }}</p>
    <!-- ... display other job details ... -->
  </div>
  <div v-else>
    <p>Loading job details...</p>
  </div>
</template>