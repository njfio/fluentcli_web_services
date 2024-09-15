<template>
    <div v-if="pipeline" class="pipeline-edit">
      <h1>Edit Pipeline</h1>
      <PipelineEditor
        :data="pipeline"
        @save="updatePipeline"
        @cancel="closeEditor"
      />
      <p v-if="isLoading" class="loading">Updating pipeline...</p>
    </div>
    <div v-else>
      <p>Loading pipeline details...</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { useRoute, useRouter } from 'vue-router';
  import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
  import apiClient from '@/services/apiClient';
  
  interface Pipeline {
    id: string;
    name: string;
    data: any;
  }
  
  const route = useRoute();
  const router = useRouter();
  const pipeline = ref<Pipeline | null>(null);
  const isLoading = ref(false);
  
  const fetchPipeline = async () => {
    try {
      const response = await apiClient.get(`/pipelines/${route.params.id}`);
      pipeline.value = response.data;
    } catch (error: any) {
      console.error('Failed to load pipeline details:', error);
    }
  };
  
  const updatePipeline = async (updatedPipeline: Pipeline) => {
    if (!pipeline.value) return;
    isLoading.value = true;
    try {
      await apiClient.put(`/pipelines/${pipeline.value.id}`, updatedPipeline);
      router.push('/studio/pipelines');
    } catch (error: any) {
      console.error('Failed to update pipeline:', error);
    } finally {
      isLoading.value = false;
    }
  };
  
  const closeEditor = () => {
    // Implement modal close if necessary
  };
  
  onMounted(() => {
    fetchPipeline();
  });
  </script>
  
  <style scoped>
  .pipeline-edit {
    padding: 20px;
  }
  .pipeline-edit form {
    display: flex;
    flex-direction: column;
  }
  .pipeline-edit label {
    margin-bottom: 5px;
  }
  .pipeline-edit input {
    margin-bottom: 15px;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 3px;
  }
  .pipeline-edit button {
    padding: 10px;
    background-color: #2980b9;
    color: #ecf0f1;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }
  .pipeline-edit button:disabled {
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