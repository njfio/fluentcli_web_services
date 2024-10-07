<template>
  <div class="amber-stores h-screen flex flex-col">
    <div class="amber-stores-list flex-grow">
      <div class="amber-stores-header">
        <h1 class="text-2xl font-bold">Amber Stores</h1>
        <div class="flex items-center">
          <input v-model="searchQuery" type="text" placeholder="Search Amber Stores" class="search-input mr-4">
          <button @click="createNewAmberStore" class="add-button">
            <i class="fas fa-plus"></i> Create New Amber Store
          </button>
        </div>
      </div>

      <div v-if="loading" class="text-center py-4">
        <i class="fas fa-spinner fa-spin fa-2x"></i>
      </div>
      <div v-else-if="error" class="text-red-500 py-4">{{ error }}</div>
      <div v-else-if="filteredAmberStores.length" class="amber-store-table-container">
        <table class="amber-store-table">
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
            <tr v-for="amberStore in paginatedAmberStores" :key="amberStore.id" @click="selectAmberStore(amberStore.id)" :class="{ 'selected': selectedAmberStoreId === amberStore.id }">
              <td>{{ amberStore.name }}</td>
              <td>{{ amberStore.description }}</td>
              <td>{{ formatDate(amberStore.createdAt) }}</td>
              <td>{{ formatDate(amberStore.lastModified) }}</td>
              <td>
                <button @click.stop="editAmberStore(amberStore.id)" class="edit-button">
                  <i class="fas fa-edit"></i> Edit
                </button>
                <button @click.stop="deleteAmberStore(amberStore.id)" class="delete-button">
                  <i class="fas fa-trash"></i> Delete
                </button>
              </td>
            </tr>
          </tbody>
        </table>
        <div class="pagination">
          <button @click="prevPage" :disabled="currentPage === 1" class="pagination-button">Previous</button>
          <span>Page {{ currentPage }} of {{ totalPages }}</span>
          <button @click="nextPage" :disabled="currentPage === totalPages" class="pagination-button">Next</button>
        </div>
      </div>
      <div v-else class="no-amber-stores">
        <p>No Amber Stores available. Click the "Create New Amber Store" button to create one.</p>
      </div>
    </div>

    <!-- Detailed Amber Store View -->
    <div v-if="selectedAmberStore" class="amber-store-detail mt-8">
      <h2 class="text-xl font-bold mb-4">{{ selectedAmberStore.name }} Details</h2>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p><strong>Description:</strong> {{ selectedAmberStore.description }}</p>
          <p><strong>Created At:</strong> {{ formatDate(selectedAmberStore.createdAt) }}</p>
          <p><strong>Last Modified:</strong> {{ formatDate(selectedAmberStore.lastModified) }}</p>
        </div>
        <div>
          <h3 class="font-bold mb-2">Data:</h3>
          <pre class="bg-gray-100 p-4 rounded overflow-auto max-h-60">{{ JSON.stringify(selectedAmberStore.data, null, 2) }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted, watch } from 'vue';
import { useStore } from 'vuex';
import { useRouter, useRoute } from 'vue-router';
import { formatDate } from '@/utils/dateFormatter';
import { Store } from 'vuex';
import { RootState } from '@/store/types';
import { AmberStore } from '@/store/modules/studio';

export default defineComponent({
  name: 'AmberStores',
  setup() {
    const store = useStore<Store<RootState>>();
    const router = useRouter();
    const route = useRoute();
    const loading = ref(false);
    const error = ref('');
    const searchQuery = ref('');
    const currentPage = ref(1);
    const itemsPerPage = 10;
    const selectedAmberStoreId = ref<string | null>(null);

    const amberStores = computed(() => {
      console.log('Computing amberStores:', store.getters['studio/getAmberStores']);
      return store.getters['studio/getAmberStores'] as AmberStore[];
    });

    const filteredAmberStores = computed(() => {
      return amberStores.value.filter(store =>
        store.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        store.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    const totalPages = computed(() => Math.ceil(filteredAmberStores.value.length / itemsPerPage));

    const paginatedAmberStores = computed(() => {
      const start = (currentPage.value - 1) * itemsPerPage;
      const end = start + itemsPerPage;
      return filteredAmberStores.value.slice(start, end);
    });

    const selectedAmberStore = computed(() => {
      return amberStores.value.find(store => store.id === selectedAmberStoreId.value);
    });

    onMounted(async () => {
      console.log('AmberStores component mounted');
      await fetchAmberStores();
      if (route.params.id) {
        selectedAmberStoreId.value = route.params.id as string;
      }
    });

    watch(() => route.params.id, (newId) => {
      if (newId) {
        selectedAmberStoreId.value = newId as string;
      }
    });

    const fetchAmberStores = async () => {
      loading.value = true;
      error.value = '';
      try {
        console.log('Fetching Amber Stores...');
        await store.dispatch('studio/fetchAmberStores');
        console.log('Amber Stores fetched successfully:', store.getters['studio/getAmberStores']);
      } catch (err: any) {
        console.error('Error fetching Amber Stores:', err);
        if (err.response) {
          console.error('Error response:', err.response.data);
          console.error('Error status:', err.response.status);
          console.error('Error headers:', err.response.headers);
        } else if (err.request) {
          console.error('Error request:', err.request);
        } else {
          console.error('Error message:', err.message);
        }
        error.value = `Error fetching Amber Stores: ${err.message || 'Unknown error'}`;
      } finally {
        loading.value = false;
      }
    };

    const createNewAmberStore = () => {
      console.log('Creating new Amber Store');
      router.push({ name: 'AmberStoreEditor' });
    };

    const editAmberStore = (id: string) => {
      console.log('Editing Amber Store:', id);
      router.push({ name: 'AmberStoreEditor', params: { id } });
    };

    const deleteAmberStore = async (id: string) => {
      if (confirm('Are you sure you want to delete this Amber Store?')) {
        try {
          console.log('Deleting Amber Store:', id);
          await store.dispatch('studio/deleteAmberStore', id);
          await fetchAmberStores();
        } catch (err: any) {
          console.error('Error deleting Amber Store:', err);
          error.value = `Error deleting Amber Store: ${err.message || 'Unknown error'}`;
        }
      }
    };

    const selectAmberStore = (id: string) => {
      console.log('Selecting Amber Store:', id);
      selectedAmberStoreId.value = id;
      router.push({ name: 'AmberStores', params: { id } });
    };

    const prevPage = () => {
      if (currentPage.value > 1) {
        currentPage.value--;
      }
    };

    const nextPage = () => {
      if (currentPage.value < totalPages.value) {
        currentPage.value++;
      }
    };

    return {
      amberStores,
      filteredAmberStores,
      paginatedAmberStores,
      loading,
      error,
      searchQuery,
      currentPage,
      totalPages,
      selectedAmberStoreId,
      selectedAmberStore,
      createNewAmberStore,
      editAmberStore,
      deleteAmberStore,
      selectAmberStore,
      prevPage,
      nextPage,
      formatDate,
    };
  },
});
</script>

<style scoped>
.amber-stores {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.amber-stores-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.search-input {
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 1rem;
}

.add-button, .pagination-button {
  background-color: #3498db;
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.3s ease;
}

.add-button:hover, .pagination-button:hover {
  background-color: #2980b9;
}

.amber-store-table-container {
  overflow-x: auto;
}

.amber-store-table {
  width: 100%;
  border-collapse: collapse;
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.amber-store-table th,
.amber-store-table td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid #e0e0e0;
}

.amber-store-table th {
  background-color: #f5f5f5;
  font-weight: bold;
}

.amber-store-table tr:hover {
  background-color: #f9f9f9;
  cursor: pointer;
}

.amber-store-table tr.selected {
  background-color: #e8f0fe;
}

.edit-button,
.delete-button {
  background-color: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  margin-right: 10px;
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

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 20px;
}

.pagination-button {
  margin: 0 10px;
}

.pagination-button:disabled {
  background-color: #bdc3c7;
  cursor: not-allowed;
}

.amber-store-detail {
  background-color: #f9f9f9;
  border: 1px solid #e0e0e0;
  border-radius: 5px;
  padding: 20px;
}
</style>