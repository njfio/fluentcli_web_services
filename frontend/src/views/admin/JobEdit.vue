<script setup lang="ts">
import axios from 'axios';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

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
    const response = await axios.get(`/jobs/${route.params.id}`);
    job.value = response.data;
  } catch (error) {
    console.error(error);
    // Handle error, e.g., display an error message to the user
  }
});

const updateJob = async () => {
  try {
    await axios.put(`/jobs/${route.params.id}`, job.value);
    router.push('/admin/jobs'); // Redirect to job list after successful update
  } catch (error) {
    console.error(error);
    // Handle error, e.g., display an error message to the user
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

      <button type="submit">Update</button>
    </form>
  </div>
  <div v-else>
    <p>Loading job details...</p>
  </div>
</template>