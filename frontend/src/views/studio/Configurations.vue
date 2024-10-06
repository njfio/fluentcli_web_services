<template>
  <div class="configurations">
    <div class="configurations-header">
      <h1>Configurations</h1>
      <button @click="showEditor = true" class="add-button">
        <i class="fas fa-plus"></i> Create New Configuration
      </button>
    </div>

    <!-- Configuration Editor Modal -->
    <div v-if="showEditor" class="modal">
      <div class="modal-content">
        <ConfigurationEditor
          :data="selectedConfiguration"
          @save="handleSave"
          @cancel="showEditor = false"
        />
      </div>
    </div>

    <!-- List of Configurations -->
    <div v-if="configurations.length" class="configuration-grid">
      <div v-for="configuration in configurations" :key="configuration.id" class="configuration-card">
        <h3>{{ configuration.name }}</h3>
        <div class="configuration-actions">
          <button @click="editConfiguration(configuration)" class="edit-button">
            <i class="fas fa-edit"></i> Edit
          </button>
          <button @click="configuration.id && deleteConfiguration(configuration.id)" class="delete-button">
            <i class="fas fa-trash"></i> Delete
          </button>
        </div>
      </div>
    </div>
    <div v-else class="no-configurations">
      <p>No configurations available. Click the "Create New Configuration" button to create one.</p>
    </div>
    </div>  
</template>
  
  <script setup lang="ts">
  import { ref, onMounted, watch } from 'vue';
  import ConfigurationEditor from '@/components/studio/editors/ConfigurationEditor.vue';
  import apiClient from '@/services/apiClient';
  import { useRoute } from 'vue-router';

const route = useRoute();
  
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
  
const openEditorForId = async (id: string) => {
  try {
    const response = await apiClient.get(`/configurations/${id}`);
    selectedConfiguration.value = response.data;
    showEditor.value = true;
  } catch (error) {
    console.error('Failed to fetch configuration:', error);
  }
};

onMounted(async () => {
  await fetchConfigurations();
  if (route.params.id) {
    await openEditorForId(route.params.id as string);
  }
});

watch(() => route.params.id, async (newId) => {
  if (newId) {
    await openEditorForId(newId as string);
  } else {
    showEditor.value = false;
  }
});
  </script>
  
<style scoped>
.configurations {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.configurations-header {
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

.configuration-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.configuration-card {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.configuration-card h3 {
  margin: 0 0 10px 0;
  font-size: 1.2rem;
}

.configuration-actions {
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

.no-configurations {
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
