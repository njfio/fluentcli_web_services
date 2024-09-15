<script setup lang="ts">
import axios from 'axios';
import { ref, onMounted } from 'vue';
import { API_URL } from '@/config';

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
    const response = await axios.get(`${API_URL}/jobs`);
    jobs.value = response.data;
  } catch (error) {
    console.error('Failed to fetch jobs:', error);
    // Optionally, set an error message to display to the user
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
    <p v-if="jobs.length === 0">No jobs available.</p>
  </div>
</template>