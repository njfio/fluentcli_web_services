<template>
  <div class="state-files">
    <h1 class="text-2xl font-bold mb-6">State Files</h1>
    <div class="mb-4 flex justify-between items-center">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search state files..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
      />
    </div>
    <div class="overflow-x-auto shadow-md rounded-lg">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-primary-600">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Job ID</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Worker Type</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Status</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Size</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Created At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <template v-for="job in filteredJobs" :key="job.id">
            <tr>
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                <router-link :to="{ name: 'JobDetail', params: { id: job.id } }" class="text-primary-600 hover:text-primary-900">
                  <span :title="job.id" class="truncate block max-w-xs">{{ job.id }}</span>
                </router-link>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <span :title="job.worker_type" class="truncate block max-w-xs">{{ job.worker_type }}</span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <span :class="getStatusClass(job.status)">{{ job.status }}</span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {{ formatFileSize(getStateFileSize(job.state_file_content)) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ formatDate(job.createdAt) }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                <button @click="toggleStateFile(job.id)" class="text-primary-600 hover:text-primary-900 mr-2">
                  {{ expandedStateFiles.includes(job.id) ? 'Hide' : 'Show' }} State File
                </button>
                <button @click="deleteJob(job.id)" class="text-red-600 hover:text-red-900">Delete</button>
              </td>
            </tr>
            <tr v-if="expandedStateFiles.includes(job.id)">
              <td colspan="6" class="px-6 py-4">
                <div class="max-w-full overflow-x-auto">
                  <pre class="p-4 text-sm overflow-x-auto"><code class="yaml" v-html="highlightJSON(job.state_file_content)"></code></pre>
                </div>
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { formatDate } from '@/utils/dateFormatter';
import hljs from 'highlight.js/lib/core';
import yaml from 'highlight.js/lib/languages/yaml';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('yaml', yaml);

export default defineComponent({
  name: 'StateFiles',
  setup() {
    const store = useStore();
    const searchQuery = ref('');
    const expandedStateFiles = ref<string[]>([]);

    const jobs = computed(() => store.getters['studio/getJobs']);
    const filteredJobs = computed(() => {
      return jobs.value.filter((job: any) =>
        job.id.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        job.worker_type.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        job.status.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    onMounted(() => {
      store.dispatch('studio/fetchJobs');
    });

    const deleteJob = async (id: string) => {
      if (confirm('Are you sure you want to delete this job and its associated state file?')) {
        try {
          await store.dispatch('studio/deleteJob', id);
          // Refresh the jobs list after deletion
          await store.dispatch('studio/fetchJobs');
        } catch (error) {
          console.error('Error deleting job:', error);
          // Handle error (e.g., show an error message to the user)
        }
      }
    };

    const getStatusClass = (status: string) => {
      switch (status.toLowerCase()) {
        case 'completed':
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800';
        case 'running':
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800';
        case 'failed':
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800';
        default:
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800';
      }
    };

    const toggleStateFile = (jobId: string) => {
      const index = expandedStateFiles.value.indexOf(jobId);
      if (index === -1) {
        expandedStateFiles.value.push(jobId);
      } else {
        expandedStateFiles.value.splice(index, 1);
      }
    };

    const highlightJSON = (json: any) => {
      const jsonString = JSON.stringify(json, null, 2);
      return hljs.highlight(jsonString, { language: 'yaml' }).value;
    };

    const getStateFileSize = (stateFileContent: any) => {
      return JSON.stringify(stateFileContent).length;
    };

    const formatFileSize = (bytes: number) => {
      if (bytes === 0) return '0 Bytes';
      const k = 1024;
      const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    };

    return {
      searchQuery,
      filteredJobs,
      deleteJob,
      getStatusClass,
      formatDate,
      expandedStateFiles,
      toggleStateFile,
      highlightJSON,
      getStateFileSize,
      formatFileSize,
    };
  },
});
</script>

<style scoped>
.state-files {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.overflow-x-auto {
  max-width: 100%;
  overflow-x: auto;
}
</style>