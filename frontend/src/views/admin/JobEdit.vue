<script setup lang="ts">
import axios from 'axios';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { API_URL } from '@/config';

interface Job {
  id: number;
  uri: string;
  config: any; // Replace with the actual type of your job config
  worker_type: string;
  // ... other properties
}

const route = useRoute();
const router = useRouter();
const job = ref<Job | null>(null);

onMounted(async () => {
  try {
    const response = await axios.get(`${API_URL}/jobs/${route.params.id}`);
    job.value = response.data;
  } catch (error) {
    console.error('Failed to fetch job details:', error);
    // Optionally, set an error message to display to the user
  }
});

const updateJob = async () => {
  if (!job.value) return;
  try {
    await axios.put(`${API_URL}/jobs/${route.params.id}`, job.value);
    router.push('/admin/jobs'); // Redirect to job list after successful update
  } catch (error) {
    console.error('Failed to update job:', error);
    // Optionally, set an error message to display to the user
  }
};
</script>

<template>
  <div v-if="job">
    <h1>Edit Job</h1>
    <form @submit.prevent="updateJob">
      <label for="uri">URI:</label>
      <input type="text" id="uri" v-model="job.uri" />

      <!-- ... input fields for other job properties ... -->

      <button type="submit">Update Job</button>
    </form>
  </div>
  <div v-else>
    <p>Loading job details...</p>
  </div>
</template>