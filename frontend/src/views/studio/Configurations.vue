<template>
  <div class="configurations">
    <h1 class="text-2xl font-bold mb-4">Configurations</h1>
    <div class="mb-4 flex justify-between items-center">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search configurations..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
      />
      <button
        @click="createNewConfiguration"
        class="bg-primary-500 text-white px-4 py-2 rounded-lg hover:bg-primary-600 transition-colors duration-200"
      >
        Create New Configuration
      </button>
    </div>
    <div class="overflow-x-auto shadow-md rounded-lg">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-primary-600">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/4">Name</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/4">Description</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Type</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Created At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="config in filteredConfigurations" :key="config.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              <router-link :to="{ name: 'ConfigurationEditor', params: { id: config.id } }" class="text-primary-600 hover:text-primary-900">
                <span :title="config.name" class="truncate block max-w-xs">{{ config.name }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <span :title="config.description" class="truncate block max-w-xs">{{ config.description }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ config.type }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ formatDate(config.createdAt) }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <router-link :to="{ name: 'ConfigurationEditor', params: { id: config.id } }" class="text-primary-600 hover:text-primary-900 mr-2">Edit</router-link>
              <button @click="deleteConfiguration(config.id)" class="text-red-600 hover:text-red-900">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { formatDate } from '@/utils/dateFormatter';

export default defineComponent({
  name: 'Configurations',
  setup() {
    const store = useStore();
    const router = useRouter();
    const searchQuery = ref('');

    const configurations = computed(() => store.getters['studio/getConfigurations']);
    const filteredConfigurations = computed(() => {
      return configurations.value.filter((config: any) =>
        config.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        config.description.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        config.type.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    onMounted(() => {
      store.dispatch('studio/fetchConfigurations');
    });

    const createNewConfiguration = () => {
      router.push({ name: 'ConfigurationEditor', params: { id: 'new' } });
    };

    const deleteConfiguration = async (id: string) => {
      if (confirm('Are you sure you want to delete this configuration?')) {
        try {
          await store.dispatch('studio/deleteConfiguration', id);
          // Refresh the configurations list after deletion
          await store.dispatch('studio/fetchConfigurations');
        } catch (error) {
          console.error('Error deleting configuration:', error);
          // Handle error (e.g., show an error message to the user)
        }
      }
    };

    return {
      searchQuery,
      filteredConfigurations,
      createNewConfiguration,
      deleteConfiguration,
      formatDate,
    };
  },
});
</script>

<style scoped>
.configurations {
  @apply p-6;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}
</style>
