<template>
  <div class="pipelines h-screen flex flex-col">
    <div v-if="!showEditor" class="pipelines-list flex-grow">
      <div class="pipelines-header">
        <h1>Pipelines</h1>
        <button @click="addPipeline" class="add-button">
          <i class="fas fa-plus"></i> Add New Pipeline
        </button>
      </div>

      <div v-if="pipelines.length" class="pipeline-grid">
        <div v-for="pipeline in pipelines" :key="pipeline.id" class="pipeline-card">
          <h3>{{ pipeline.name }}</h3>
          <div class="pipeline-actions">
            <button @click="editPipeline(pipeline.id)" class="edit-button">
              <i class="fas fa-edit"></i> Edit
            </button>
            <button @click="deletePipeline(pipeline.id)" class="delete-button">
              <i class="fas fa-trash"></i> Delete
            </button>
          </div>
        </div>
      </div>
      <div v-else class="no-pipelines">
        <p>No pipelines available. Click the "Add New Pipeline" button to create one.</p>
      </div>
    </div>

    <div v-if="showEditor && selectedPipeline" class="pipeline-editor-container flex-grow flex">
      <PipelineEditor
        :data="selectedPipeline"
        @save="handleSave"
        @cancel="closeEditor"
        class="flex-grow"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
import apiClient from '@/services/apiClient';


interface Pipeline {
  id: string;
  name: string;
  data: string;
}

const route = useRoute();
const router = useRouter();
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

const props = defineProps<{
  returnToJobDetails?: boolean;
}>();

const addPipeline = () => {
  selectedPipeline.value = {
    id: '',
    name: '',
    data: '',
  };
  showEditor.value = true;
};

const editPipeline = async (id: string) => {
  try {
    const response = await apiClient.get(`/pipelines/${id}`);
    selectedPipeline.value = response.data;
    showEditor.value = true;
  } catch (error) {
    console.error('Failed to fetch pipeline:', error);
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

const handleSave = async (updatedPipeline: Pipeline) => {
  try {
    const pipelineData = {
      ...updatedPipeline,
      // No YAML parsing here, just use the data as-is
      data: updatedPipeline.data,
    };

    if (updatedPipeline.id) {
      await apiClient.put(`/pipelines/${updatedPipeline.id}`, pipelineData);
    } else {
      await apiClient.post('/pipelines', pipelineData);
    }
    await fetchPipelines();
    closeEditor();
    
    if (props.returnToJobDetails) {
      router.go(-1);
    }
  } catch (error) {
    console.error('Failed to save pipeline:', error);
  }
};

const closeEditor = () => {
  showEditor.value = false;
  selectedPipeline.value = null;
  
  if (props.returnToJobDetails) {
    router.go(-1);
  }
};

onMounted(async () => {
  await fetchPipelines();
  if (route.params.id) {
    await editPipeline(route.params.id as string);
  }
});

watch(() => route.params.id, async (newId) => {
  if (newId) {
    await editPipeline(newId as string);
  } else {
    closeEditor();
  }
});
</script>

<style scoped>
.pipelines {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.pipelines-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.add-button {
  background-color: #3498db;
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.3s ease;
}

.add-button:hover {
  background-color: #2980b9;
}

.pipeline-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.pipeline-card {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.pipeline-card h3 {
  margin: 0 0 10px 0;
  font-size: 1.2rem;
}

.pipeline-actions {
  display: flex;
  justify-content: flex-end;
}

.edit-button, .delete-button {
  background-color: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  margin-left: 10px;
  transition: color 0.3s ease;
}

.edit-button {
  color: #3498db;
}

.edit-button:hover {
  color: #2980b9;
}

.delete-button {
  color: #e74c3c;
}

.delete-button:hover {
  color: #c0392b;
}

.no-pipelines {
  text-align: center;
  color: #7f8c8d;
  margin-top: 50px;
}

.error {
  color: #e74c3c;
  margin-top: 10px;
}

.loading {
  color: #3498db;
  margin-top: 10px;
}

.modal {
  position: fixed;
  z-index: 1;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  overflow: auto;
  background-color: rgba(0,0,0,0.4);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-content {
  background-color: #fefefe;
  padding: 20px;
  border: 1px solid #888;
  width: 90%;
  max-width: 1200px;
  max-height: 90vh;
  overflow-y: auto;
  border-radius: 5px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}
</style>