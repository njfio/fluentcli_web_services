<!-- frontend/src/views/studio/PipelineDetails.vue -->
<template>
    <div v-if="pipeline" class="pipeline-details">
      <h1>Pipeline Details</h1>
      <p><strong>ID:</strong> {{ pipeline.id }}</p>
      <p><strong>Name:</strong> {{ pipeline.name }}</p>
      <p><strong>Description:</strong> {{ pipeline.description }}</p>
      <!-- Display other pipeline properties as needed -->
      <router-link :to="`/studio/pipelines/${pipeline.id}/edit`">Edit Pipeline</router-link>
    </div>
    <div v-else>
      <p>Loading pipeline details...</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { useRoute } from 'vue-router';
  import apiClient from '@/services/apiClient';
  
  interface Pipeline {
    id: number;
    name: string;
    description: string;
    // ... other properties
  }
  
  const route = useRoute();
  const pipeline = ref<Pipeline | null>(null);
  const errorMessage = ref('');
  
  const fetchPipeline = async () => {
    try {
      const response = await apiClient.get(`/pipelines/${route.params.id}`);
      pipeline.value = response.data;
    } catch (error: any) {
      errorMessage.value = error.response?.data?.message || 'Failed to load pipeline details.';
    }
  };
  
  onMounted(() => {
    fetchPipeline();
  });
  </script>
  
  <style scoped>
  .pipeline-details {
    padding: 20px;
  }
  .pipeline-details p {
    margin-bottom: 10px;
  }
  .pipeline-details a {
    color: #42b983;
    text-decoration: none;
  }
  .pipeline-details a:hover {
    text-decoration: underline;
  }
  </style>