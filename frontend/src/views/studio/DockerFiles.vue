<template>
  <div class="docker-files">
    <div class="docker-files-header">
      <h1>Docker Files</h1>
      <button @click="showEditor = true" class="add-button">
        <i class="fas fa-plus"></i> Create New Docker File
      </button>
    </div>

    <!-- Docker File Editor Modal -->
    <div v-if="showEditor" class="modal">
      <div class="modal-content">
        <DockerFileEditor
          :dockerFile="selectedDockerFile"
          @save="handleSave"
          @cancel="showEditor = false"
        />
      </div>
    </div>

    <!-- List of Docker Files -->
    <div v-if="dockerFiles.length" class="docker-file-grid">
      <div v-for="dockerFile in dockerFiles" :key="dockerFile.id" class="docker-file-card">
        <h3>{{ dockerFile.name }}</h3>
        <div class="docker-file-actions">
          <button @click="editDockerFile(dockerFile)" class="edit-button">
            <i class="fas fa-edit"></i> Edit
          </button>
          <button @click="dockerFile.id && deleteDockerFile(dockerFile.id)" class="delete-button">
            <i class="fas fa-trash"></i> Delete
          </button>
        </div>
      </div>
    </div>
    <div v-else class="no-docker-files">
      <p>No Docker files available. Click the "Create New Docker File" button to create one.</p>
    </div>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="isLoading" class="loading">Loading...</p>
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
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.docker-files-header {
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

.docker-file-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.docker-file-card {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.docker-file-card h3 {
  margin: 0 0 10px 0;
  font-size: 1.2rem;
}

.docker-file-actions {
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

.no-docker-files {
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