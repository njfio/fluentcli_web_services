<template>
    <div class="configurations">
      <h2>Configurations</h2>
      <button @click="showEditor = true" class="create-button">Create New Configuration</button>
      <table v-if="configurations.length">
        <thead>
          <tr>
            <th>Name</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="configuration in configurations" :key="configuration.id">
            <td>{{ configuration.name }}</td>
            <td>
              <button @click="editConfiguration(configuration)" class="edit-button">Edit</button>
              <button @click="configuration.id && deleteConfiguration(configuration.id)" class="delete-button">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-else>No configurations available.</p>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-if="isLoading" class="loading">Loading...</p>
  
      <ConfigurationEditor
        v-if="showEditor"
        :data="selectedConfiguration"
        @save="handleSave"
        @cancel="showEditor = false"
      />
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import ConfigurationEditor from '@/components/studio/editors/ConfigurationEditor.vue';
  import apiClient from '@/services/apiClient';
  
  interface Configuration {
    id?: string;
    name: string;
    data: any;
  }
  
  const configurations = ref<Configuration[]>([]);
  const showEditor = ref(false);
  const selectedConfiguration = ref<Configuration>({ name: '', data: {} });
  const error = ref<string | null>(null);
  const isLoading = ref(false);
  
  const fetchConfigurations = async () => {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await apiClient.get('/configurations');
      configurations.value = response.data;
    } catch (err: any) {
      error.value = 'Failed to fetch configurations. Please try again.';
      console.error('Error fetching configurations:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  const editConfiguration = (configuration: Configuration) => {
    selectedConfiguration.value = { ...configuration };
    showEditor.value = true;
  };
  
  const handleSave = async (configuration: Configuration) => {
    isLoading.value = true;
    error.value = null;
    try {
      if (configuration.id) {
        await apiClient.put(`/configurations/${configuration.id}`, configuration);
      } else {
        await apiClient.post('/configurations', configuration);
      }
      await fetchConfigurations();
      showEditor.value = false;
    } catch (err: any) {
      error.value = 'Failed to save configuration. Please try again.';
      console.error('Error saving configuration:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  const deleteConfiguration = async (id: string) => {
    if (!confirm('Are you sure you want to delete this configuration?')) return;
    isLoading.value = true;
    error.value = null;
    try {
      await apiClient.delete(`/configurations/${id}`);
      await fetchConfigurations();
    } catch (err: any) {
      error.value = 'Failed to delete configuration. Please try again.';
      console.error('Error deleting configuration:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  onMounted(fetchConfigurations);
  </script>
  
  <style scoped>
  .configurations {
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