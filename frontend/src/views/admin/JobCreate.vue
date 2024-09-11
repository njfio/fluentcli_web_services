<script setup lang="ts">
import axios from 'axios';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

interface Job {
  uri: string;
  config: any; // Replace with the actual type of your job config
  worker_type: string;
  // ... other properties
}

const router = useRouter();
const newJob = ref<Job>({
  uri: '',
  config: {},
  worker_type: '',
  // ... initialize other properties
});

const createJob = async () => {
  try {
    await axios.post('/jobs', newJob.value);
    router.push('/admin/jobs'); // Redirect to job list after successful creation
  } catch (error) {
    console.error(error);
    // Handle error, e.g., display an error message to the user
  }
};
</script>

<template>
  <div>
    <h1>Create Job</h1>
    <form @submit.prevent="createJob">
      <label for="uri">URI:</label>
      <input type="text" id="uri" v-model="newJob.uri" />

      <!-- ... input fields for other job properties ... -->

      <button type="submit">Create</button>
    </form>
  </div>
</template>