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
              <router-link :to="`/studio/pipelines/${pipeline.id}`">View</router-link> |
              <router-link :to="`/studio/pipelines/${pipeline.id}/edit`">Edit</router-link> |
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
  
  <script lang="ts">
  import { defineComponent, ref, onMounted } from 'vue';
  import apiClient from '@/services/apiClient';
  import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
  
  interface Pipeline {
    id: string;
    name: string;
    data: any;
  }
  
  interface NewPipeline {
    name: string;
    data: any;
  }
  
  export default defineComponent({
    name: 'Pipelines',
    components: {
      PipelineEditor,
    },
    setup() {
      const pipelines = ref<Pipeline[]>([]);
      const showEditor = ref(false);
      const selectedPipeline = ref<Pipeline | NewPipeline | null>(null);
  
      const fetchPipelines = async () => {
        try {
          const response = await apiClient.get('/pipelines');
          pipelines.value = response.data;
        } catch (error: any) {
          console.error('Failed to fetch pipelines:', error);
        }
      };
  
      const addPipeline = () => {
        selectedPipeline.value = {
          name: '',
          data: {
            type: 'doc',
            content: [],
          },
        };
        showEditor.value = true;
      };
  
      function isExistingPipeline(pipeline: Pipeline | NewPipeline): pipeline is Pipeline {
  return (pipeline as Pipeline).id !== undefined;
}

const handleSave = async (pipeline: Pipeline | NewPipeline) => {
  try {
    if (isExistingPipeline(pipeline)) {
      // Update existing pipeline
      await apiClient.put(`/pipelines/${pipeline.id}`, pipeline);
    } else {
      // Create new pipeline without `id`
      const response = await apiClient.post('/pipelines', pipeline);
      const createdPipeline: Pipeline = response.data;
      pipelines.value.push(createdPipeline);
    }
    await fetchPipelines();
    closeEditor();
  } catch (error: any) {
    console.error('Failed to save pipeline:', error);
  }
};
  
      const closeEditor = () => {
        showEditor.value = false;
        selectedPipeline.value = null;
      };
  
      const deletePipeline = async (id: string) => {
        if (!confirm('Are you sure you want to delete this pipeline?')) return;
        try {
          await apiClient.delete(`/pipelines/${id}`);
          pipelines.value = pipelines.value.filter((p) => p.id !== id);
        } catch (error: any) {
          alert('Failed to delete the pipeline. Please try again.');
        }
      };
  
      onMounted(() => {
        fetchPipelines();
      });
  
      return {
        pipelines,
        showEditor,
        selectedPipeline,
        addPipeline,
        handleSave,
        closeEditor,
        deletePipeline,
      };
    },
  });
  </script>
  
  <style scoped>
  .pipelines {
    padding: 20px;
  }
  
  .pipelines .add-button {
    margin-bottom: 15px;
    padding: 10px 20px;
    background-color: #2980b9;
    color: #ecf0f1;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }
  
  .pipelines .add-button:hover {
    background-color: #3498db;
  }
  
  .pipeline-list {
    list-style-type: none;
    padding: 0;
  }
  
  .pipeline-item {
    display: flex;
    justify-content: space-between;
    padding: 10px 0;
    border-bottom: 1px solid #ddd;
  }
  
  .pipeline-item .actions a,
  .pipeline-item .actions .delete-button {
    margin-left: 10px;
    color: #2980b9;
    text-decoration: none;
    cursor: pointer;
  }
  
  .pipeline-item .actions .delete-button {
    background-color: #c0392b;
    color: #fff;
    border: none;
    padding: 5px 10px;
    border-radius: 3px;
  }
  
  .pipeline-item .actions .delete-button:hover {
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
  
  .modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  .modal-content {
    background-color: #fff;
    padding: 20px;
    border-radius: 5px;
  }
  </style>