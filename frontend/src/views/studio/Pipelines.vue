<template>
    <div class="pipelines">
      <h1>Pipelines</h1>
      <button @click="showEditor = true" class="add-button">Add New Pipeline</button>
      
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
      <div v-else>
        <ul class="pipeline-list">
          <li v-for="pipeline in pipelines" :key="pipeline.id" class="pipeline-item">
            <span>{{ pipeline.name }}</span>
            <div class="actions">
              <button @click="editPipeline(pipeline)" class="edit-button">Edit</button>
              <button @click="deletePipeline(pipeline.id)" class="delete-button">Delete</button>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, onMounted } from 'vue';
  import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
  import axios from 'axios';
  
  interface Stage {
    name: string;
    command: string;
  }
  
  interface Pipeline {
    id: string;
    name: string;
    stages: Stage[];
  }
  
  export default defineComponent({
    name: 'Pipelines',
    components: {
      PipelineEditor,
    },
    setup() {
      const pipelines = ref<Pipeline[]>([]);
      const showEditor = ref(false);
      const selectedPipeline = ref<Pipeline | null>(null);
  
      const fetchPipelines = async () => {
        try {
          const response = await axios.get('/api/pipelines');
          pipelines.value = response.data;
        } catch (error) {
          console.error('Failed to fetch pipelines:', error);
        }
      };
  
      const handleSave = async (pipeline: Pipeline) => {
        try {
          if (pipeline.id) {
            await axios.put(`/api/pipelines/${pipeline.id}`, pipeline);
          } else {
            await axios.post('/api/pipelines', pipeline);
          }
          await fetchPipelines();
          closeEditor();
        } catch (error) {
          console.error('Failed to save pipeline:', error);
        }
      };
  
      const editPipeline = (pipeline: Pipeline) => {
        selectedPipeline.value = { ...pipeline };
        showEditor.value = true;
      };
  
      const deletePipeline = async (id: string) => {
        try {
          await axios.delete(`/api/pipelines/${id}`);
          await fetchPipelines();
        } catch (error) {
          console.error('Failed to delete pipeline:', error);
        }
      };
  
      const closeEditor = () => {
        selectedPipeline.value = null;
        showEditor.value = false;
      };
  
      onMounted(() => {
        fetchPipelines();
      });
  
      return {
        pipelines,
        showEditor,
        selectedPipeline,
        handleSave,
        editPipeline,
        deletePipeline,
        closeEditor,
      };
    },
  });
  </script>
  
  <style scoped>
  .pipelines {
    padding: 20px;
  }
  
  .add-button {
    background-color: #2c3e50;
    color: #ecf0f1;
    border: none;
    padding: 10px 15px;
    border-radius: 5px;
    cursor: pointer;
    margin-bottom: 20px;
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
    width: 500px;
  }
  
  .pipeline-list {
    list-style: none;
    padding: 0;
  }
  
  .pipeline-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 0;
    border-bottom: 1px solid #bdc3c7;
  }
  
  .actions button {
    margin-left: 10px;
    padding: 5px 10px;
    cursor: pointer;
  }
  
  .edit-button {
    background-color: #2980b9;
    color: #fff;
    border: none;
    border-radius: 3px;
  }
  
  .delete-button {
    background-color: #c0392b;
    color: #fff;
    border: none;
    border-radius: 3px;
  }
  </style>