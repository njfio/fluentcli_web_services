<template>
  <div class="docker-files">
    <h1 class="text-2xl font-bold mb-6">Docker Files</h1>

    <div v-if="loading" class="text-center">Loading...</div>
    <div v-else-if="error" class="text-red-500">{{ error }}</div>
    <div v-else-if="dockerFiles.length" class="docker-file-table-container">
      <table class="docker-file-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Description</th>
            <th>Created At</th>
            <th>Last Modified</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="dockerFile in dockerFiles" :key="dockerFile.id">
            <td>{{ dockerFile.name }}</td>
            <td>{{ dockerFile.description }}</td>
            <td>{{ formatDate(dockerFile.createdAt) }}</td>
            <td>{{ formatDate(dockerFile.updatedAt) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else class="no-docker-files">
      <p>No Docker files available.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useStore } from 'vuex';
import { formatDate } from '@/utils/dateFormatter';

interface DockerFile {
  id: string;
  name: string;
  description: string;
  createdAt: string;
  updatedAt: string;
}

const store = useStore();

const dockerFiles = computed<DockerFile[]>(() => store.getters['studio/getDockerFiles']);
const loading = ref(false);
const error = ref('');

console.log('DockerFiles component initialized');

onMounted(async () => {
  console.log('DockerFiles component mounted');
  await fetchDockerFiles();
});

watch(dockerFiles, (newValue) => {
  console.log('dockerFiles updated:', newValue);
});

const fetchDockerFiles = async () => {
  console.log('Fetching Docker files...');
  loading.value = true;
  error.value = '';
  try {
    await store.dispatch('studio/fetchDockerFiles');
    console.log('Docker files fetched successfully');
  } catch (err: any) {
    console.error('Failed to fetch Docker files:', err);
    error.value = `Error fetching Docker files: ${err.message || 'Unknown error'}`;
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.docker-files {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.docker-file-table-container {
  overflow-x: auto;
}

.docker-file-table {
  width: 100%;
  border-collapse: collapse;
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.docker-file-table th,
.docker-file-table td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid #e0e0e0;
}

.docker-file-table th {
  background-color: #f5f5f5;
  font-weight: bold;
}

.docker-file-table tr:hover {
  background-color: #f9f9f9;
}

.no-docker-files {
  text-align: center;
  color: #7f8c8d;
  margin-top: 50px;
}
</style>
