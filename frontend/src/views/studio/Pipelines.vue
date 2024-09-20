<template>
  <div class="pipelines">
    <h1>Pipelines</h1>
    <button @click="addPipeline" class="add-button">Add New Pipeline</button>

    <!-- Pipeline Editor Modal -->
<div v-if="showEditor && selectedPipeline" class="modal">
  <div class="modal-content">
    <PipelineEditor
      :data="selectedPipeline"
      @save="handleSave"
      @cancel="closeEditor"
    />
  </div>
</div>

    <!-- List of Pipelines -->
    <div v-if="pipelines.length">
      <ul class="pipeline-list">
        <li v-for="pipeline in pipelines" :key="pipeline.id" class="pipeline-item">
          <span>{{ pipeline.name }}</span>
          <div class="actions">
            <button @click="editPipeline(pipeline)">Edit</button>
            <button @click="deletePipeline(pipeline.id)" class="delete-button">Delete</button>
          </div>
        </li>
      </ul>
    </div>
    <div v-else>
      <p>No pipelines available.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
import apiClient from '@/services/apiClient';
import * as yaml from 'js-yaml';

interface Pipeline {
  id: string;
  name: string;
  data: string;
}

const pipelines = ref<Pipeline[]>([]);
const showEditor = ref(false);
const selectedPipeline = ref<Pipeline | null>(null);

const fetchPipelines = async () => {
  try {
    const response = await apiClient.get('/pipelines');
    pipelines.value = response.data;
  } catch (error) {
    console.error('Failed to fetch pipelines:', error);
  }
};

const addPipeline = () => {
  selectedPipeline.value = {
    id: '',  // Use an empty string as a temporary ID
    name: '',
    data: '',
  };
  showEditor.value = true;
};

const editPipeline = (pipeline: Pipeline) => {
  selectedPipeline.value = { ...pipeline };
  showEditor.value = true;
};

const handleSave = async (updatedPipeline: Pipeline) => {
  try {
    const pipelineData = {
      ...updatedPipeline,
      data: yaml.load(updatedPipeline.data),
    };

    if (updatedPipeline.id) {
      await apiClient.put(`/pipelines/${updatedPipeline.id}`, pipelineData);
    } else {
      await apiClient.post('/pipelines', pipelineData);
    }
    await fetchPipelines();
    closeEditor();
  } catch (error) {
    console.error('Failed to save pipeline:', error);
  }
};

const deletePipeline = async (id: string | undefined) => {
  if (!id) {
    console.error('Cannot delete pipeline: ID is undefined');
    return;
  }
  if (confirm('Are you sure you want to delete this pipeline?')) {
    try {
      await apiClient.delete(`/pipelines/${id}`);
      await fetchPipelines();
    } catch (error) {
      console.error('Failed to delete pipeline:', error);
    }
  }
};

const closeEditor = () => {
  showEditor.value = false;
  selectedPipeline.value = null;
};

onMounted(() => {
  fetchPipelines();
});
</script>

<style scoped>
/* Add your styles here */
</style>