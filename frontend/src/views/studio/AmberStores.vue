<template>
    <div class="amber-stores">
      <h2>Amber Stores</h2>
      <button @click="showEditor = true" class="create-button">Create New Amber Store</button>
      <table v-if="amberStores.length">
        <thead>
          <tr>
            <th>ID</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="amberStore in amberStores" :key="amberStore.id">
            <td>{{ amberStore.id }}</td>
            <td>
              <button @click="editAmberStore(amberStore)" class="edit-button">Edit</button>
              <button @click="amberStore.id && deleteAmberStore(amberStore.id)" class="delete-button">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-else>No amber stores available.</p>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-if="isLoading" class="loading">Loading...</p>
  
      <AmberStoreEditor
        v-if="showEditor"
        :data="selectedAmberStore"
        @save="handleSave"
        @cancel="showEditor = false"
      />
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import AmberStoreEditor from '@/components/studio/editors/AmberStoreEditor.vue';
  import apiClient from '@/services/apiClient';
  
  interface AmberStore {
    id?: string;
    data: any;
  }
  
  const amberStores = ref<AmberStore[]>([]);
  const showEditor = ref(false);
  const selectedAmberStore = ref<AmberStore>({ data: {} });
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
  
  const editAmberStore = (amberStore: AmberStore) => {
    selectedAmberStore.value = { ...amberStore };
    showEditor.value = true;
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
      showEditor.value = false;
    } catch (err: any) {
      error.value = 'Failed to save amber store. Please try again.';
      console.error('Error saving amber store:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  const deleteAmberStore = async (id: string) => {
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
  
  onMounted(fetchAmberStores);
  </script>
  
  <style scoped>
  .amber-stores {
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