<!-- frontend/src/views/studio/DockerFileEdit.vue -->
<template>
  <div class="docker-file-edit">
    <h1>{{ isNew ? 'Create' : 'Edit' }} Docker File</h1>
    <div v-if="dockerFile" class="editor-container">
      <DockerFileEditor
        :dockerFile="dockerFile"
        @save="updateDockerFile"
        @cancel="cancelEdit"
      />
    </div>
    <div v-else class="loading-container">
      <p>Loading Docker file details...</p>
    </div>
    <p v-if="isLoading" class="loading">{{ isNew ? 'Creating' : 'Updating' }} Docker file...</p>
    <p v-if="error" class="error">{{ error }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import DockerFileEditor from '@/components/studio/editors/DockerFileEditor.vue';
import apiClient from '@/services/apiClient';

interface DockerFile {
  id?: string;
  name: string;
  content: string;
}

const route = useRoute();
const router = useRouter();
const dockerFile = ref<DockerFile | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

const isNew = computed(() => route.params.id === 'new');

const fetchDockerFile = async () => {
  if (isNew.value) {
    dockerFile.value = { name: '', content: '' };
    return;
  }

  try {
    const response = await apiClient.get(`/docker_files/${route.params.id}`);
    dockerFile.value = response.data;
  } catch (err: any) {
    console.error('Failed to load Docker file details:', err);
    error.value = 'Failed to load Docker file. Please try again.';
  }
};

const updateDockerFile = async (updatedDockerFile: DockerFile) => {
  isLoading.value = true;
  error.value = null;
  try {
    if (isNew.value) {
      await apiClient.post('/docker_files', updatedDockerFile);
    } else {
      await apiClient.put(`/docker_files/${dockerFile.value?.id}`, updatedDockerFile);
    }
    router.push('/studio/dockerfiles');
  } catch (err: any) {
    console.error('Failed to update Docker file:', err);
    error.value = `Failed to ${isNew.value ? 'create' : 'update'} the Docker file. Please try again.`;
  } finally {
    isLoading.value = false;
  }
};

const cancelEdit = () => {
  router.push('/studio/dockerfiles');
};

onMounted(() => {
  fetchDockerFile();
});
</script>

<style scoped>
.docker-file-edit {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

h1 {
  font-size: 2rem;
  margin-bottom: 20px;
  color: #2c3e50;
}

.editor-container {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.loading-container {
  text-align: center;
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 5px;
}

.loading, .error {
  margin-top: 15px;
  padding: 10px;
  border-radius: 5px;
  text-align: center;
}

.loading {
  background-color: #e1f5fe;
  color: #0288d1;
}

.error {
  background-color: #ffebee;
  color: #c62828;
}
</style>