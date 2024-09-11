<script setup lang="ts">
import axios from 'axios';
import { ref, onMounted } from 'vue';

interface Job {
  id: number;
  uri: string;
  config: any; // Replace with the actual type of your job config
  worker_type: string;
  // ... other properties
}

const jobs = ref<Job[]>([]);

onMounted(async () => {
  try {
    const response = await axios.get('/jobs');
    jobs.value = response.data;
  } catch (error) {
    console.error(error);
    // Handle error, e.g., display an error message to the user
  }
});
</script>

<template>
  <div>
    <h1>Jobs</h1>
    <ul>
      <li v-for="job in jobs" :key="job.id">
        <router-link :to="`/admin/jobs/${job.id}`">{{ job.uri }}</router-link>
      </li>
    </ul>
  </div>
</template>