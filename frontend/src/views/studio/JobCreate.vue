<template>
    <div class="job-create">
      <h2>Create New Job</h2>
      <form @submit.prevent="createJob">
        <div>
          <label for="uri">URI:</label>
          <input type="text" id="uri" v-model="job.uri" required />
        </div>
        <div>
          <label for="worker_type">Worker Type:</label>
          <input type="text" id="worker_type" v-model="job.worker_type" required />
        </div>
        <!-- Add more fields as necessary -->
        <button type="submit">Create</button>
      </form>
      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
      <p v-if="isLoading">Creating job...</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';
  import apiClient from '@/services/apiClient';
  
  interface Job {
    uri: string;
    worker_type: string;
    // ... other properties
  }
  
  const job = ref<Job>({
    uri: '',
    worker_type: '',
    // ... initialize other properties
  });
  
  const router = useRouter();
  const errorMessage = ref('');
  const isLoading = ref(false);
  
  const createJob = async () => {
    isLoading.value = true;
    errorMessage.value = '';
    try {
      await apiClient.post('/jobs', job.value); // Adjust endpoint if needed
      router.push('/studio/jobs');
    } catch (error: any) {
      console.error('Failed to create job:', error);
      errorMessage.value = error.response?.data?.message || 'Failed to create job.';
    } finally {
      isLoading.value = false;
    }
  };
  </script>
  
  <style scoped>
  .job-create {
    padding: 20px;
  }
  
  .job-create form {
    display: flex;
    flex-direction: column;
  }
  
  .job-create label {
    margin-bottom: 5px;
  }
  
  .job-create input {
    margin-bottom: 15px;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 3px;
  }
  
  .job-create button {
    padding: 10px;
    background-color: #2c3e50;
    color: #ecf0f1;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }
  
  .job-create button:disabled {
    background-color: #95a5a6;
    cursor: not-allowed;
  }
  
  .error {
    color: red;
    margin-top: 10px;
  }
  
  .loading {
    color: #3498db;
    margin-top: 10px;
  }
  </style>