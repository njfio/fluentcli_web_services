<template>
    <div class="pipeline-create">
      <h1>Create Pipeline</h1>
      <PipelineEditor
        :data="pipeline"
        @save="createPipeline"
        @cancel="closeEditor"
      />
      <p v-if="isLoading" class="loading">Creating pipeline...</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';
  import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
  import apiClient from '@/services/apiClient';
  
  interface Pipeline {
    name: string;
    data: any;
  }
  
  const router = useRouter();
  const pipeline = ref<Pipeline>({
    name: '',
    data: {
      type: 'doc',
      content: [],
    },
  });
  const isLoading = ref(false);
  
  const createPipeline = async (newPipeline: Pipeline) => {
    isLoading.value = true;
    try {
      await apiClient.post('/pipelines', newPipeline);
      router.push('/studio/pipelines');
    } catch (error: any) {
      console.error('Failed to create pipeline:', error);
    } finally {
      isLoading.value = false;
    }
  };
  
  const closeEditor = () => {
    // Implement modal close if necessary
  };
  </script>
  
  <style scoped>
  .pipeline-create {
    padding: 20px;
  }
  .pipeline-create form {
    display: flex;
    flex-direction: column;
  }
  .pipeline-create label {
    margin-bottom: 5px;
  }
  .pipeline-create input {
    margin-bottom: 15px;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 3px;
  }
  .pipeline-create button {
    padding: 10px;
    background-color: #2c3e50;
    color: #ecf0f1;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }
  .pipeline-create button:disabled {
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