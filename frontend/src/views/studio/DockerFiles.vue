<template>
  <div class="docker-files">
    <h1 class="text-2xl font-bold mb-6">Docker Files</h1>

    <div class="mb-4">
      <button @click="createNewDockerFile" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Create New Docker File
      </button>
    </div>

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
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="dockerFile in dockerFiles" :key="dockerFile.id">
            <td>{{ dockerFile.name }}</td>
            <td>{{ dockerFile.description }}</td>
            <td>{{ formatDate(dockerFile.createdAt) }}</td>
            <td>{{ formatDate(dockerFile.updatedAt) }}</td>
            <td>
              <button @click="editDockerFile(dockerFile.id)" class="text-blue-500 hover:text-blue-700 mr-2">
                Edit
              </button>
              <button @click="deleteDockerFile(dockerFile.id)" class="text-red-500 hover:text-red-700">
                Delete
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else class="no-docker-files">
      <p>No Docker files available. Click the "Create New Docker File" button to create one.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { formatDate } from '@/utils/dateFormatter';

const store = useStore();
const router = useRouter();

const dockerFiles = computed(() => {
  console.log('Computing dockerFiles:', store.getters['studio/getDockerFiles']);
  return store.getters['studio/getDockerFiles'];
});
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

const createNewDockerFile = () => {
  console.log('Creating new Docker file');
  router.push({ name: 'NewDockerFile' });
};

const editDockerFile = (id: string) => {
  console.log('Editing Docker file:', id);
  router.push({ name: 'DockerFileEditor', params: { id } });
};

const deleteDockerFile = async (id: string) => {
  if (confirm('Are you sure you want to delete this Docker file?')) {
    console.log('Deleting Docker file:', id);
    try {
      await store.dispatch('studio/deleteDockerFile', id);
      await fetchDockerFiles();
    } catch (err: any) {
      console.error('Failed to delete Docker file:', err);
      error.value = `Error deleting Docker file: ${err.message || 'Unknown error'}`;
    }
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
