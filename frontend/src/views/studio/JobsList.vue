<template>
  <div class="job-list">
    <h2>Job List</h2>
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>URI</th>
          <th>Worker Type</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="job in jobs" :key="job.id">
          <td>{{ job.id }}</td>
          <td>{{ job.uri }}</td>
          <td>{{ job.worker_type }}</td>
          <td>
            <router-link :to="`/admin/jobs/${job.id}`">View</router-link> |
            <router-link :to="`/admin/jobs/${job.id}/edit`">Edit</router-link> |
            <button @click="deleteJob(job.id)" class="delete-button">Delete</button>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-if="jobs.length === 0">No jobs available.</p>
    <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
    <p v-if="isLoading">Loading jobs...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import apiClient from '@/services/apiClient';

interface Job {
  id: number;
  uri: string;
  config: any; // Replace with the actual type of your job config
  worker_type: string;
  // ... other properties
}

const jobs = ref<Job[]>([]);
const isLoading = ref(true);
const errorMessage = ref('');

const fetchJobs = async () => {
  try {
    const response = await apiClient.get('/jobs'); // Adjust endpoint if needed
    jobs.value = response.data;
  } catch (error: any) {
    console.error('Failed to fetch jobs:', error);
    errorMessage.value = error.response?.data?.message || 'Failed to load jobs.';
  } finally {
    isLoading.value = false;
  }
};

const deleteJob = async (id: number) => {
  if (!confirm('Are you sure you want to delete this job?')) return;
  try {
    await apiClient.delete(`/jobs/${id}`); // Adjust endpoint if needed
    jobs.value = jobs.value.filter(job => job.id !== id);
  } catch (error) {
    console.error('Failed to delete job:', error);
    alert('Failed to delete the job. Please try again.');
  }
};

onMounted(() => {
  fetchJobs();
});
</script>

<style scoped>
.job-list {
  padding: 20px;
}

.job-list table {
  width: 100%;
  border-collapse: collapse;
}

.job-list th,
.job-list td {
  border: 1px solid #ddd;
  padding: 8px;
}

.job-list th {
  background-color: #f2f2f2;
  text-align: left;
}

.delete-button {
  background-color: #c0392b;
  color: #fff;
  border: none;
  padding: 5px 10px;
  border-radius: 3px;
  cursor: pointer;
}

.delete-button:hover {
  background-color: #e74c3c;
}

.error {
  color: red;
}

p {
  margin-top: 20px;
}

.loading {
  color: #3498db;
}
</style>