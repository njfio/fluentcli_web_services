<!-- frontend/src/components/studio/PipelineList.vue -->
<template>
    <div class="pipeline-list">
      <h2>Pipeline List</h2>
      <router-link to="/studio/pipelines/create" class="create-button">Create New Pipeline</router-link>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Description</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="pipeline in pipelines" :key="pipeline.id">
            <td>{{ pipeline.id }}</td>
            <td>{{ pipeline.name }}</td>
            <td>{{ pipeline.description }}</td>
            <td>
              <router-link :to="`/studio/pipelines/${pipeline.id}`">View</router-link> |
              <router-link :to="`/studio/pipelines/${pipeline.id}/edit`">Edit</router-link> |
              <button @click="deletePipeline(pipeline.id)" class="delete-button">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-if="pipelines.length === 0">No pipelines available.</p>
      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
      <p v-if="isLoading" class="loading">Loading pipelines...</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import apiClient from '@/services/apiClient';
  
  interface Pipeline {
    id: number;
    name: string;
    description: string;
    // ... other properties
  }
  
  const pipelines = ref<Pipeline[]>([]);
  const isLoading = ref(true);
  const errorMessage = ref('');
  
  const fetchPipelines = async () => {
    try {
      const response = await apiClient.fetchPipelines();
      pipelines.value = response.data;
    } catch (error: any) {
      errorMessage.value = error.response?.data?.message || 'Failed to load pipelines.';
    } finally {
      isLoading.value = false;
    }
  };
  
  const deletePipeline = async (id: number) => {
    if (!confirm('Are you sure you want to delete this pipeline?')) return;
    try {
      await apiClient.deletePipeline(id.toString());
      pipelines.value = pipelines.value.filter(pipeline => pipeline.id !== id);
    } catch (error: any) {
      alert('Failed to delete the pipeline. Please try again.');
    }
  };
  
  onMounted(() => {
    fetchPipelines();
  });
  </script>
  
  <style scoped>
  .pipeline-list {
    padding: 20px;
  }
  .pipeline-list .create-button {
    display: inline-block;
    margin-bottom: 15px;
  }
  .pipeline-list table {
    width: 100%;
    border-collapse: collapse;
  }
  .pipeline-list th,
  .pipeline-list td {
    border: 1px solid #ddd;
    padding: 8px;
  }
  .pipeline-list th {
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
    margin-top: 10px;
  }
  .loading {
    color: #3498db;
    margin-top: 10px;
  }
  </style>