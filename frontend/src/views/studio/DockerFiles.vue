<template>
  <div class="docker-files">
    <h1 class="text-2xl font-bold mb-4 dark:text-white">Docker Files</h1>
    <div class="mb-4 flex justify-between items-center">
      <input v-model="searchQuery" type="text" placeholder="Search docker files..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-white dark:border-gray-600" />
      <button @click="createNewDockerFile"
        class="bg-primary-500 text-white px-4 py-2 rounded-lg hover:bg-primary-600 transition-colors duration-200">
        Create New Docker File
      </button>
    </div>
    <div class="overflow-x-auto shadow-md rounded-lg">
      <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead class="bg-primary-600 dark:bg-primary-800">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/4">
              Name</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/4">
              Description</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Created At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Updated At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
          <tr v-for="dockerFile in filteredDockerFiles" :key="dockerFile.id" class="dark:hover:bg-gray-700">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
              <router-link :to="{ name: 'DockerFileEditor', params: { id: dockerFile.id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300">
                <span :title="dockerFile.name" class="truncate block max-w-xs">{{ dockerFile.name }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              <span :title="dockerFile.description" class="truncate block max-w-xs">{{ dockerFile.description }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              {{ formatDate(dockerFile.createdAt) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              {{ formatDate(dockerFile.updatedAt) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <router-link :to="{ name: 'DockerFileEditor', params: { id: dockerFile.id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300 mr-2">Edit</router-link>
              <button @click="deleteDockerFile(dockerFile.id)"
                class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300">Delete</button>
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
  name: 'DockerFiles',
  setup() {
    const store = useStore();
    const router = useRouter();
    const searchQuery = ref('');

    const dockerFiles = computed(() => store.getters['studio/getDockerFiles']);
    const filteredDockerFiles = computed(() => {
      return dockerFiles.value.filter((dockerFile: any) =>
        dockerFile.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        dockerFile.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    onMounted(() => {
      store.dispatch('studio/fetchDockerFiles');
    });

    const createNewDockerFile = () => {
      router.push({ name: 'DockerFileEditor', params: { id: 'new' } });
    };

    const deleteDockerFile = async (id: string) => {
      if (confirm('Are you sure you want to delete this Docker file?')) {
        try {
          await store.dispatch('studio/deleteDockerFile', id);
          // Refresh the Docker files list after deletion
          await store.dispatch('studio/fetchDockerFiles');
        } catch (error) {
          console.error('Error deleting Docker file:', error);
          // Handle error (e.g., show an error message to the user)
        }
      }
    };

    return {
      searchQuery,
      filteredDockerFiles,
      createNewDockerFile,
      deleteDockerFile,
      formatDate,
    };
  },
});
</script>

<style scoped>
.docker-files {
  @apply p-6 dark:bg-gray-900;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}
</style>
