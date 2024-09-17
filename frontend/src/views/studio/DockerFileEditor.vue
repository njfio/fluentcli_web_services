<!-- frontend/src/views/studio/DockerFileEdit.vue -->
<template>
    <div v-if="dockerFile" class="docker-file-edit">
      <h1>Edit Docker File</h1>
      <DockerFileEditor
        :dockerFile="dockerFile"
        @save="updateDockerFile"
        @cancel="cancelEdit"
      />
      <p v-if="isLoading" class="loading">Updating Docker file...</p>
    </div>
    <div v-else>
      <p>Loading Docker file details...</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
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
  
  const fetchDockerFile = async () => {
    try {
      const response = await apiClient.get(`/docker_files/${route.params.id}`);
      dockerFile.value = response.data;
    } catch (error: any) {
      console.error('Failed to load Docker file details:', error);
    }
  };
  
  const updateDockerFile = async (updatedDockerFile: DockerFile) => {
    if (!dockerFile.value) return;
    isLoading.value = true;
    try {
      await apiClient.put(`/docker_files/${dockerFile.value.id}`, updatedDockerFile);
      router.push(`/studio/dockerfiles/${dockerFile.value.id}`);
    } catch (error: any) {
      console.error('Failed to update Docker file:', error);
      alert('Failed to update the Docker file. Please try again.');
    } finally {
      isLoading.value = false;
    }
  };
  
  const cancelEdit = () => {
    if (dockerFile.value) {
      router.push(`/studio/dockerfiles/${dockerFile.value.id}`);
    } else {
      router.push('/studio/dockerfiles');
    }
  };
  
  onMounted(() => {
    fetchDockerFile();
  });
  </script>
  
  <style scoped>
  .docker-file-edit {
    padding: 20px;
  }
  
  .loading {
    color: #3498db;
    margin-top: 10px;
  }
  </style>