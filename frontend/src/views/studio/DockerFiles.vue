<template>
  <div class="docker-files">
    <h2>Docker Files</h2>
    <button @click="showEditor = true" class="create-button">Create New Docker File</button>
    <table v-if="dockerFiles.length">
      <thead>
        <tr>
          <th>Name</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="dockerFile in dockerFiles" :key="dockerFile.id">
          <td>{{ dockerFile.name }}</td>
          <td>
            <button @click="editDockerFile(dockerFile)" class="edit-button">Edit</button>
            <button @click="dockerFile.id && deleteDockerFile(dockerFile.id)" class="delete-button">Delete</button>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-else>No Docker files available.</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="isLoading" class="loading">Loading...</p>

    <DockerFileEditor
      v-if="showEditor"
      :dockerFile="selectedDockerFile"
      @save="handleSave"
      @cancel="showEditor = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import DockerFileEditor from '@/components/studio/editors/DockerFileEditor.vue';
import apiClient from '@/services/apiClient';

interface DockerFile {
  id?: string;
  name: string;
  content: string;
}

const dockerFiles = ref<DockerFile[]>([]);
const showEditor = ref(false);
const selectedDockerFile = ref<DockerFile | null>(null);
const error = ref<string | null>(null);
const isLoading = ref(false);

const fetchDockerFiles = async () => {
  isLoading.value = true;
  error.value = null;
  try {
    const response = await apiClient.get('/docker_files');
    dockerFiles.value = response.data;
  } catch (err: any) {
    error.value = 'Failed to fetch Docker files. Please try again.';
    console.error('Error fetching Docker files:', err);
  } finally {
    isLoading.value = false;
  }
};




const editDockerFile = (dockerFile: DockerFile) => {
  selectedDockerFile.value = { ...dockerFile };
  showEditor.value = true;
};

const handleSave = async (dockerFile: DockerFile) => {
  isLoading.value = true;
  error.value = null;
  try {
    if (dockerFile.id) {
      await apiClient.put(`/docker_files/${dockerFile.id}`, dockerFile);
    } else {
      await apiClient.post('/docker_files', dockerFile);
    }
    await fetchDockerFiles();
    showEditor.value = false;
  } catch (err: any) {
    error.value = 'Failed to save Docker file. Please try again.';
    console.error('Error saving Docker file:', err);
  } finally {
    isLoading.value = false;
  }
};

const deleteDockerFile = async (id: string) => {
  if (!confirm('Are you sure you want to delete this Docker file?')) return;
  isLoading.value = true;
  error.value = null;
  try {
    await apiClient.delete(`/docker_files/${id}`);
    await fetchDockerFiles();
  } catch (err: any) {
    error.value = 'Failed to delete Docker file. Please try again.';
    console.error('Error deleting Docker file:', err);
  } finally {
    isLoading.value = false;
  }
};

onMounted(fetchDockerFiles);
</script>

<style scoped>
.docker-files {
  padding: 20px;
}
.create-button {
  margin-bottom: 15px;
}
table {
  width: 100%;
  border-collapse: collapse;
}
th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}
th {
  background-color: #f2f2f2;
}
.edit-button, .delete-button {
  margin-right: 5px;
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