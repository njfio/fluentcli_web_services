<template>
  <div class="pipelines">
    <h1 class="text-2xl font-bold mb-4 dark:text-white">Pipelines</h1>
    <div class="mb-4 flex justify-between items-center">
      <input v-model="searchQuery" type="text" placeholder="Search pipelines..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-white dark:border-gray-600" />
      <button @click="createNewPipeline"
        class="bg-primary-500 text-white px-4 py-2 rounded-lg hover:bg-primary-600 transition-colors duration-200">
        Create New Pipeline
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
              Last Modified</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
          <tr v-for="pipeline in filteredPipelines" :key="pipeline.id" class="dark:hover:bg-gray-700">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
              <router-link :to="{ name: 'PipelineEditor', params: { id: pipeline.id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300">
                <span :title="pipeline.name" class="truncate block max-w-xs">{{ pipeline.name }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              <span :title="pipeline.description" class="truncate block max-w-xs">{{ pipeline.description }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">{{
              formatDate(pipeline.createdAt) }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">{{
              formatDate(pipeline.lastModified) }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <router-link :to="{ name: 'PipelineEditor', params: { id: pipeline.id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300 mr-2">Edit</router-link>
              <button @click="deletePipeline(pipeline.id)"
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
  name: 'Pipelines',
  setup() {
    const store = useStore();
    const router = useRouter();
    const searchQuery = ref('');

    const pipelines = computed(() => store.getters['studio/getPipelines']);
    const filteredPipelines = computed(() => {
      return pipelines.value.filter((pipeline: any) =>
        pipeline.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        pipeline.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    onMounted(() => {
      store.dispatch('studio/fetchPipelines');
    });

    const createNewPipeline = () => {
      router.push({ name: 'PipelineEditor', params: { id: 'new' } });
    };

    const deletePipeline = async (id: string) => {
      if (confirm('Are you sure you want to delete this pipeline?')) {
        try {
          await store.dispatch('studio/deletePipeline', id);
          // Refresh the pipelines list after deletion
          await store.dispatch('studio/fetchPipelines');
        } catch (error) {
          console.error('Error deleting pipeline:', error);
          // Handle error (e.g., show an error message to the user)
        }
      }
    };

    return {
      searchQuery,
      filteredPipelines,
      createNewPipeline,
      deletePipeline,
      formatDate,
    };
  },
});
</script>

<style scoped>
.pipelines {
  @apply p-6 dark:bg-gray-900;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}
</style>