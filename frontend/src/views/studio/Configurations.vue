<template>
  <div class="configurations">
    <h1 class="text-2xl font-bold mb-4">Configurations</h1>
    <div class="mb-4 relative">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search configurations..."
        class="w-full px-3 py-2 border border-gray-300 rounded-md"
        @keydown.enter="handleEnterKey"
        @keydown.esc="clearSearch"
      />
      <div v-if="isSearching" class="absolute right-3 top-2">
        <div class="spinner-small"></div>
      </div>
    </div>
    <div v-if="loading" class="text-center py-4">
      <div class="spinner"></div>
      <p class="mt-2">Loading configurations...</p>
    </div>
    <div v-else-if="error" class="text-center py-4">
      <p class="text-red-500">{{ error }}</p>
      <button @click="fetchConfigurations()" class="mt-2 bg-blue-500 text-white px-4 py-2 rounded">Retry</button>
    </div>
    <div v-else>
      <div class="overflow-x-auto">
        <table v-if="filteredConfigurations.length" class="min-w-full bg-white border border-gray-300">
          <thead>
            <tr>
              <th v-for="column in columns" :key="column.key" @click="sort(column.key)" class="py-2 px-4 border-b cursor-pointer">
                {{ column.label }}
                <span v-if="sortKey === column.key">{{ sortOrder === 'asc' ? '▲' : '▼' }}</span>
              </th>
              <th class="py-2 px-4 border-b">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="config in filteredConfigurations" :key="config.id">
              <td v-for="column in columns" :key="column.key" class="py-2 px-4 border-b">{{ config[column.key] }}</td>
              <td class="py-2 px-4 border-b">
                <button @click="editConfiguration(config.id)" class="bg-blue-500 text-white px-2 py-1 rounded mr-2">Edit</button>
                <button @click="confirmDelete(config)" class="bg-red-500 text-white px-2 py-1 rounded" :disabled="isDeleting">
                  {{ isDeleting && configToDelete?.id === config.id ? 'Deleting...' : 'Delete' }}
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <p v-if="!filteredConfigurations.length" class="text-center py-4">No configurations found.</p>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="mt-4 flex justify-center">
        <button
          v-for="page in totalPages"
          :key="page"
          @click="() => changePage(page)"
          :class="['mx-1 px-3 py-1 rounded', currentPage === page ? 'bg-blue-500 text-white' : 'bg-gray-200']"
        >
          {{ page }}
        </button>
      </div>
    </div>
    <button @click="createNewConfiguration" class="mt-4 bg-green-500 text-white px-4 py-2 rounded">Create New Configuration</button>

    <!-- Confirmation Dialog -->
    <div v-if="showConfirmDialog" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center">
      <div class="bg-white p-5 rounded-lg shadow-lg">
        <h2 class="text-xl font-bold mb-4">Confirm Deletion</h2>
        <p>Are you sure you want to delete the configuration "{{ configToDelete?.name }}"?</p>
        <div class="mt-4 flex justify-end">
          <button @click="cancelDelete" class="bg-gray-300 text-black px-4 py-2 rounded mr-2">Cancel</button>
          <button @click="deleteConfiguration" class="bg-red-500 text-white px-4 py-2 rounded" :disabled="isDeleting">
            {{ isDeleting ? 'Deleting...' : 'Delete' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Success Message -->
    <div v-if="showSuccessMessage" class="fixed bottom-4 right-4 bg-green-500 text-white px-4 py-2 rounded shadow-lg">
      {{ successMessage }}
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, computed, ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import { useStore } from 'vuex';

interface Configuration {
  id: string;
  name: string;
  description: string;
  type: string;
  status: string;
}

interface Column {
  key: keyof Configuration;
  label: string;
}

function debounce<T extends (...args: any[]) => any>(func: T, wait: number): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;
  return (...args: Parameters<T>) => {
    if (timeout !== null) {
      clearTimeout(timeout);
    }
    timeout = setTimeout(() => func(...args), wait);
  };
}

export default defineComponent({
  name: 'Configurations',
  setup() {
    console.log('Configurations component setup function starting');
    const router = useRouter();
    const store = useStore();
    const loading = ref(false);
    const error = ref('');
    const showConfirmDialog = ref(false);
    const configToDelete = ref<Configuration | null>(null);
    const showSuccessMessage = ref(false);
    const successMessage = ref('');
    const currentPage = ref(1);
    const itemsPerPage = 10;
    const sortKey = ref<keyof Configuration>('name');
    const sortOrder = ref<'asc' | 'desc'>('asc');
    const searchQuery = ref('');
    const isDeleting = ref(false);
    const isSearching = ref(false);

    const columns: Column[] = [
      { key: 'id', label: 'ID' },
      { key: 'name', label: 'Name' },
      { key: 'description', label: 'Description' },
      { key: 'type', label: 'Type' },
      { key: 'status', label: 'Status' },
    ];

    const configurations = computed<Configuration[]>(() => {
      const configs = store.getters['studio/getConfigurations'];
      console.log('Computed configurations:', configs);
      return configs;
    });

    const filteredConfigurations = computed(() => {
      return sortedConfigurations.value.filter(config =>
        config.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        config.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    const sortedConfigurations = computed(() => {
      return [...configurations.value].sort((a, b) => {
        if (a[sortKey.value] < b[sortKey.value]) return sortOrder.value === 'asc' ? -1 : 1;
        if (a[sortKey.value] > b[sortKey.value]) return sortOrder.value === 'asc' ? 1 : -1;
        return 0;
      });
    });

    const totalItems = computed(() => filteredConfigurations.value.length);
    const totalPages = computed(() => Math.ceil(totalItems.value / itemsPerPage));

    const fetchConfigurations = async (page = 1) => {
      loading.value = true;
      error.value = '';
      console.log('Fetching configurations...');
      try {
        await store.dispatch('studio/fetchConfigurations', { page, itemsPerPage });
        console.log('Configurations fetched successfully');
      } catch (err) {
        error.value = 'Failed to fetch configurations. Please try again.';
        console.error('Error fetching configurations:', err);
      } finally {
        loading.value = false;
      }
    };

    const changePage = (page: number) => {
      currentPage.value = page;
      fetchConfigurations(page);
    };

    const sort = (key: keyof Configuration) => {
      if (sortKey.value === key) {
        sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
      } else {
        sortKey.value = key;
        sortOrder.value = 'asc';
      }
    };

    onMounted(() => {
      console.log('Configurations component mounted');
      fetchConfigurations();
    });

    const editConfiguration = (id: string) => {
      console.log('Editing configuration:', id);
      router.push({ name: 'ConfigurationEditor', params: { id } });
    };

    const confirmDelete = (config: Configuration) => {
      configToDelete.value = config;
      showConfirmDialog.value = true;
    };

    const cancelDelete = () => {
      configToDelete.value = null;
      showConfirmDialog.value = false;
    };

    const deleteConfiguration = async () => {
      if (configToDelete.value) {
        console.log('Deleting configuration:', configToDelete.value.id);
        isDeleting.value = true;
        try {
          await store.dispatch('studio/deleteConfiguration', configToDelete.value.id);
          await fetchConfigurations(currentPage.value);
          showConfirmDialog.value = false;
          configToDelete.value = null;
          showSuccessMessage.value = true;
          successMessage.value = 'Configuration deleted successfully';
          setTimeout(() => {
            showSuccessMessage.value = false;
          }, 3000);
        } catch (err) {
          console.error('Error deleting configuration:', err);
          error.value = 'Failed to delete configuration. Please try again.';
        } finally {
          isDeleting.value = false;
        }
      }
    };

    const createNewConfiguration = () => {
      console.log('Creating new configuration');
      router.push({ name: 'NewConfiguration' });
    };

    const debouncedSearch = debounce(() => {
      isSearching.value = true;
      currentPage.value = 1;
      fetchConfigurations().finally(() => {
        isSearching.value = false;
      });
    }, 300);

    watch(searchQuery, () => {
      debouncedSearch();
    });

    const handleEnterKey = () => {
      debouncedSearch();
    };

    const clearSearch = () => {
      searchQuery.value = '';
      debouncedSearch();
    };

    console.log('Raw configurations state:', store.state.studio.configurations);

    return {
      configurations,
      filteredConfigurations,
      loading,
      error,
      editConfiguration,
      confirmDelete,
      cancelDelete,
      deleteConfiguration,
      createNewConfiguration,
      showConfirmDialog,
      configToDelete,
      fetchConfigurations,
      showSuccessMessage,
      successMessage,
      currentPage,
      totalPages,
      changePage,
      columns,
      sort,
      sortKey,
      sortOrder,
      searchQuery,
      isDeleting,
      isSearching,
      handleEnterKey,
      clearSearch,
    };
  },
});
</script>

<style scoped>
.configurations {
  padding: 20px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 8px;
  border-bottom: 1px solid #ddd;
}

th {
  background-color: #f2f2f2;
  font-weight: bold;
}

button {
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover:not(:disabled) {
  opacity: 0.8;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinner {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

.spinner-small {
  border: 2px solid #f3f3f3;
  border-top: 2px solid #3498db;
  border-radius: 50%;
  width: 20px;
  height: 20px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@media (max-width: 640px) {
  .configurations {
    padding: 10px;
  }

  table {
    font-size: 14px;
  }

  th, td {
    padding: 6px;
  }

  button {
    padding: 4px 8px;
    font-size: 12px;
  }
}
</style>
