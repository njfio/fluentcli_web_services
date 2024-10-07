<template>
  <div class="pipelines h-screen flex flex-col">
    <div class="pipelines-list flex-grow">
      <div class="pipelines-header">
        <h1>Pipelines</h1>
        <button @click="addPipeline" class="add-button">
          <i class="fas fa-plus"></i> Add New Pipeline
        </button>
      </div>

      <div v-if="loading" class="text-center">Loading...</div>
      <div v-else-if="error" class="text-red-500">{{ error }}</div>
      <div v-else-if="pipelines.length" class="pipeline-table-container">
        <table class="pipeline-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
              <th>Created At</th>
              <th>Last Modified</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="pipeline in pipelines" :key="pipeline.id">
              <td>{{ pipeline.name }}</td>
              <td>{{ pipeline.description }}</td>
              <td>{{ formatDate(pipeline.createdAt) }}</td>
              <td>{{ formatDate(pipeline.lastModified) }}</td>
              <td>
                <button @click="editPipeline(pipeline.id)" class="edit-button">
                  <i class="fas fa-edit"></i> Edit
                </button>
                <button @click="deletePipeline(pipeline.id)" class="delete-button">
                  <i class="fas fa-trash"></i> Delete
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-else class="no-pipelines">
        <p>No pipelines available. Click the "Add New Pipeline" button to create one.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useStore } from 'vuex';
import { formatDate } from '@/utils/dateFormatter';

interface Pipeline {
  id: string;
  name: string;
  description: string;
  createdAt: string;
  lastModified: string;
}

const router = useRouter();
const store = useStore();

const pipelines = computed<Pipeline[]>(() => store.getters['studio/getPipelines']);
const loading = ref(false);
const error = ref('');

onMounted(async () => {
  console.log('Pipelines component mounted');
  await fetchPipelines();
});

const fetchPipelines = async () => {
  loading.value = true;
  error.value = '';
  try {
    console.log('Fetching pipelines...');
    await store.dispatch('studio/fetchPipelines');
    console.log('Pipelines fetched:', pipelines.value);
  } catch (err: any) {
    console.error('Failed to fetch pipelines:', err);
    error.value = `Error fetching pipelines: ${err.message || 'Unknown error'}`;
  } finally {
    loading.value = false;
  }
};

const addPipeline = () => {
  router.push({ name: 'PipelineEditor' });
};

const editPipeline = (id: string) => {
  router.push({ name: 'PipelineEditor', params: { id } });
};

const deletePipeline = async (id: string) => {
  if (confirm('Are you sure you want to delete this pipeline?')) {
    try {
      await store.dispatch('studio/deletePipeline', id);
      await fetchPipelines();
    } catch (err: any) {
      console.error('Failed to delete pipeline:', err);
      error.value = `Error deleting pipeline: ${err.message || 'Unknown error'}`;
    }
  }
};
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

.pipeline-table-container {
  overflow-x: auto;
}

.pipeline-table {
  width: 100%;
  border-collapse: collapse;
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.pipeline-table th,
.pipeline-table td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid #e0e0e0;
}

.pipeline-table th {
  background-color: #f5f5f5;
  font-weight: bold;
}

.pipeline-table tr:hover {
  background-color: #f9f9f9;
}

.edit-button,
.delete-button {
  background-color: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  margin-right: 10px;
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
</style>