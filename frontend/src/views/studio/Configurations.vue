<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 py-8">
    <div class="container mx-auto px-4">
      <h2 class="text-3xl font-bold mb-6 text-gray-800">Configurations</h2>
      <div class="mb-4 relative">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search configurations..."
          class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
          @keydown.enter="handleEnterKey"
          @keydown.esc="clearSearch"
        />
        <div v-if="isSearching" class="absolute right-3 top-2">
          <div class="spinner-small"></div>
        </div>
      </div>
      <div v-if="loading" class="text-center py-8">
        <div class="spinner"></div>
        <p class="mt-4 text-gray-600">Loading configurations...</p>
      </div>
      <div v-else-if="error" class="text-center py-8">
        <p class="text-red-500">{{ error }}</p>
        <button @click="fetchConfigurations()" class="mt-4 bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700 transition-colors duration-300">Retry</button>
      </div>
      <div v-else>
        <div class="bg-white shadow-lg rounded-lg overflow-hidden">
          <div class="overflow-x-auto">
            <table v-if="filteredConfigurations.length" class="min-w-full">
              <thead>
                <tr class="bg-indigo-600 text-white">
                  <th v-for="column in columns" :key="column.key" @click="sort(column.key)" class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider cursor-pointer">
                    {{ column.label }}
                    <span v-if="sortKey === column.key">{{ sortOrder === 'asc' ? '▲' : '▼' }}</span>
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(config, index) in filteredConfigurations" :key="config.id" 
                    class="hover:bg-indigo-50 transition-colors duration-150 ease-in-out"
                    :class="{'bg-gray-50': index % 2 === 0}"
                    :style="{ animationDelay: `${index * 50}ms` }">
                  <td v-for="column in columns" :key="column.key" class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ config[column.key] }}</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <button @click="editConfiguration(config.id)" class="text-indigo-600 hover:text-indigo-900 mr-2 transition-colors duration-300">Edit</button>
                    <button @click="confirmDelete(config)" class="text-red-600 hover:text-red-900 transition-colors duration-300" :disabled="isDeleting">
                      {{ isDeleting && configToDelete?.id === config.id ? 'Deleting...' : 'Delete' }}
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <p v-if="!filteredConfigurations.length" class="text-center py-8 text-gray-600">No configurations found.</p>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="mt-6 flex justify-center">
          <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px" aria-label="Pagination">
            <button
              v-for="page in totalPages"
              :key="page"
              @click="() => changePage(page)"
              :class="[
                'relative inline-flex items-center px-4 py-2 border text-sm font-medium',
                currentPage === page
                  ? 'z-10 bg-indigo-50 border-indigo-500 text-indigo-600'
                  : 'bg-white border-gray-300 text-gray-500 hover:bg-gray-50',
                page === 1 ? 'rounded-l-md' : '',
                page === totalPages ? 'rounded-r-md' : '',
              ]"
            >
              {{ page }}
            </button>
          </nav>
        </div>
      </div>
      <button @click="createNewConfiguration" class="mt-6 bg-green-500 hover:bg-green-600 text-white px-6 py-2 rounded-md transition-colors duration-300">Create New Configuration</button>

      <!-- Confirmation Dialog -->
      <div v-if="showConfirmDialog" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center">
        <div class="bg-white p-8 rounded-lg shadow-xl max-w-md w-full">
          <h2 class="text-2xl font-bold mb-4 text-gray-800">Confirm Deletion</h2>
          <p class="mb-6 text-gray-600">Are you sure you want to delete the configuration "{{ configToDelete?.name }}"?</p>
          <div class="flex justify-end space-x-4">
            <button @click="cancelDelete" class="px-4 py-2 bg-gray-300 text-gray-800 rounded-md hover:bg-gray-400 transition-colors duration-300">Cancel</button>
            <button @click="deleteConfiguration" class="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors duration-300" :disabled="isDeleting">
              {{ isDeleting ? 'Deleting...' : 'Delete' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Success Message -->
      <div v-if="showSuccessMessage" class="fixed bottom-4 right-4 bg-green-500 text-white px-6 py-3 rounded-md shadow-lg">
        {{ successMessage }}
      </div>
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
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

tbody tr {
  animation: fadeIn 0.3s ease-out forwards;
  opacity: 0;
}

.spinner {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #4f46e5;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

.spinner-small {
  border: 2px solid #f3f3f3;
  border-top: 2px solid #4f46e5;
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
  .container {
    padding-left: 1rem;
    padding-right: 1rem;
  }

  table {
    font-size: 0.875rem;
  }

  th, td {
    padding: 0.5rem;
  }

  button {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }
}
</style>
