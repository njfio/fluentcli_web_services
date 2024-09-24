<template>
  <div class="amber-stores">
    <div class="amber-stores-header">
      <h1>Amber Stores</h1>
      <button @click="addAmberStore" class="add-button">
        <i class="fas fa-plus"></i> Add New Amber Store
      </button>
    </div>

    <!-- Amber Store Editor Modal -->
    <div v-if="showEditor" class="modal">
      <div class="modal-content">
        <AmberStoreEditor
          :data="selectedAmberStore"
          @save="handleSave"
          @cancel="closeEditor"
        />
      </div>
    </div>

    <!-- List of Amber Stores -->
    <div v-if="amberStores.length" class="amber-store-grid">
      <div v-for="amberStore in amberStores" :key="amberStore.id" class="amber-store-card">
        <h3>{{ amberStore.name }}</h3>
        <p>ID: {{ amberStore.id }}</p>
        <div class="amber-store-actions">
          <button @click="editAmberStore(amberStore)" class="edit-button">
            <i class="fas fa-edit"></i> Edit
          </button>
          <button @click="deleteAmberStore(amberStore.id)" class="delete-button">
            <i class="fas fa-trash"></i> Delete
          </button>
        </div>
      </div>
    </div>
    <div v-else class="no-amber-stores">
      <p>No amber stores available. Click the "Add New Amber Store" button to create one.</p>
    </div>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="isLoading" class="loading">Loading...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import * as yaml from 'js-yaml';

import AmberStoreEditor from '@/components/studio/editors/AmberStoreEditor.vue';
import apiClient from '@/services/apiClient';

interface AmberStore {
  id?: string;
  name: string;
  data: string;
  secure_key_hash: string;
}

const amberStores = ref<AmberStore[]>([]);
const showEditor = ref(false);
const selectedAmberStore = ref<AmberStore>({ name: '', data: '{}', secure_key_hash: '' });
const error = ref<string | null>(null);
const isLoading = ref(false);

const fetchAmberStores = async () => {
  isLoading.value = true;
  error.value = null;
  try {
    const response = await apiClient.get('/amber_store');
    amberStores.value = response.data;
  } catch (err: any) {
    error.value = 'Failed to fetch amber stores. Please try again.';
    console.error('Error fetching amber stores:', err);
  } finally {
    isLoading.value = false;
  }
};

const addAmberStore = () => {
  selectedAmberStore.value = { name: '', data: '{}', secure_key_hash: '' };
  showEditor.value = true;
};

const editAmberStore = (amberStore: AmberStore) => {
  try {
    selectedAmberStore.value = { 
      ...amberStore,
      data: typeof amberStore.data === 'string' 
        ? yaml.dump(yaml.load(amberStore.data) || {}, { indent: 2 })
        : yaml.dump(amberStore.data || {}, { indent: 2 })
    };
    showEditor.value = true;
  } catch (err) {
    console.error('Error parsing AmberStore data:', err);
    error.value = 'Invalid data format in AmberStore. Unable to edit.';
  }
};

const handleSave = async (amberStore: AmberStore) => {
  isLoading.value = true;
  error.value = null;
  try {
    if (amberStore.id) {
      await apiClient.put(`/amber_store/${amberStore.id}`, amberStore);
    } else {
      await apiClient.post('/amber_store', amberStore);
    }
    await fetchAmberStores();
    closeEditor();
  } catch (err: any) {
    error.value = 'Failed to save amber store. Please try again.';
    console.error('Error saving amber store:', err);
  } finally {
    isLoading.value = false;
  }
};

const deleteAmberStore = async (id: string | undefined) => {
  if (!id) {
    console.error('Cannot delete amber store: ID is undefined');
    return;
  }
  if (!confirm('Are you sure you want to delete this amber store?')) return;
  isLoading.value = true;
  error.value = null;
  try {
    await apiClient.delete(`/amber_store/${id}`);
    await fetchAmberStores();
  } catch (err: any) {
    error.value = 'Failed to delete amber store. Please try again.';
    console.error('Error deleting amber store:', err);
  } finally {
    isLoading.value = false;
  }
};

const closeEditor = () => {
  showEditor.value = false;
  selectedAmberStore.value = { name: '', data: '{}', secure_key_hash: '' };
};

onMounted(fetchAmberStores);
</script>

<style scoped>
.amber-stores {
  max-width: 1200px;
  margin: 0 auto;
}

.amber-stores-header {
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

.amber-store-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.amber-store-card {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.amber-store-card h3 {
  margin: 0 0 10px 0;
  font-size: 1.2rem;
}

.amber-store-card p {
  margin: 0 0 10px 0;
  color: #7f8c8d;
}

.amber-store-actions {
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

.no-amber-stores {
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