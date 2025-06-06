<template>
  <div class="job-detail bg-gray-50 dark:bg-gray-900 min-h-screen py-8">
    <div class="max-w-6xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden">
      <div class="px-6 py-4 bg-primary-600 dark:bg-primary-800 text-white flex justify-between items-center">
        <h2 class="text-2xl font-bold">Job Details</h2>
        <div class="flex items-center">
          <button v-if="canStartOrRestartJob" @click="startOrRestartJob"
            class="mr-4 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded">
            {{ job && job.status === 'completed' ? 'Restart Job' : 'Start Job' }}
          </button>
          <div v-if="job" class="text-sm font-medium px-3 py-1 rounded-full" :class="getStatusClass(job.status)">
            {{ job.status }}
          </div>
        </div>
      </div>
      <div v-if="loading" class="p-6 text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600 mx-auto"></div>
        <p class="mt-4 text-gray-600 dark:text-gray-300">Loading job details...</p>
      </div>
      <div v-else-if="error" class="p-6 text-center">
        <p class="text-red-600 dark:text-red-400">{{ error }}</p>
      </div>
      <div v-else-if="job" class="p-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div v-for="(value, key) in jobDetails" :key="key"
            class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg shadow-sm border border-gray-200 dark:border-gray-600">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-300 mb-1">{{ formatLabel(key) }}</p>
            <p v-if="isLinkableField(key)"
              class="text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 font-semibold">
              <router-link :to="getLinkForField(key, value)">
                {{ getLinkedItemName(key, value) }}
              </router-link>
            </p>
            <p v-else class="font-semibold text-gray-800 dark:text-gray-200">{{ formatValue(key, value) }}</p>
          </div>
        </div>
        <div class="mt-8">
          <div
            class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg shadow-sm border border-gray-200 dark:border-gray-600 mb-4">
            <h3 @click="toggleStateFileContent"
              class="font-semibold mb-2 cursor-pointer flex items-center text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300">
              <span class="mr-2">State File Content</span>
              <svg :class="{ 'rotate-180': showStateFileContent }" class="w-4 h-4 transition-transform"
                xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd"
                  d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                  clip-rule="evenodd" />
              </svg>
            </h3>
            <div v-if="showStateFileContent" class="mt-2">
              <pre
                class="bg-white dark:bg-gray-800 p-4 rounded overflow-x-auto text-sm border border-gray-200 dark:border-gray-600"
                v-html="highlightedStateFileContent"></pre>
            </div>
          </div>
          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg shadow-sm border border-gray-200 dark:border-gray-600">
            <h3 @click="toggleJobLogs"
              class="font-semibold mb-2 cursor-pointer flex items-center text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300">
              <span class="mr-2">Job Logs</span>
              <svg :class="{ 'rotate-180': showJobLogs }" class="w-4 h-4 transition-transform"
                xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd"
                  d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                  clip-rule="evenodd" />
              </svg>
            </h3>
            <div v-if="showJobLogs" class="mt-2">
              <pre
                class="bg-white dark:bg-gray-800 p-4 rounded overflow-x-auto text-sm border border-gray-200 dark:border-gray-600"
                v-html="highlightedJobLogs"></pre>
            </div>
          </div>
        </div>
      </div>
      <div v-else class="p-6 text-center">
        <p class="text-gray-600 dark:text-gray-300">No job details found.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useStore } from 'vuex';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';
import apiClient from '@/services/apiClient';

hljs.registerLanguage('json', json);

const route = useRoute();
const store = useStore();
const loading = ref(false);
const error = ref<string | null>(null);
const showStateFileContent = ref(false);
const showJobLogs = ref(false);
const jobLogs = ref<string>('');

const job = computed(() => {
  const currentJob = store.getters['studio/getCurrentJob'];
  console.log('Computed job:', currentJob);
  return currentJob;
});

const jobDetails = computed(() => {
  if (!job.value) return {};
  const { id, worker_type, config, pipeline_id, amber_id, status, created_at, updated_at } = job.value;
  const details = { id, worker_type, config, pipeline_id, amber_id, status, created_at, updated_at };
  console.log('Computed jobDetails:', details);
  return details;
});

const canStartOrRestartJob = computed(() => {
  return job.value && (job.value.status !== 'running' || job.value.status === 'completed');
});

const highlightedStateFileContent = computed(() => {
  if (!job.value || !job.value.state_file_content) return '';
  try {
    const jsonContent = typeof job.value.state_file_content === 'string'
      ? JSON.parse(job.value.state_file_content)
      : job.value.state_file_content;
    const formattedContent = JSON.stringify(jsonContent, null, 2);
    return hljs.highlight(formattedContent, { language: 'json' }).value;
  } catch (error) {
    console.error('Error parsing or highlighting JSON:', error);
    return job.value.state_file_content;
  }
});

const highlightedJobLogs = computed(() => {
  if (!jobLogs.value) return '';
  try {
    const jsonContent = typeof jobLogs.value === 'string'
      ? JSON.parse(jobLogs.value)
      : jobLogs.value;
    const formattedContent = JSON.stringify(jsonContent, null, 2);
    return hljs.highlight(formattedContent, { language: 'json' }).value;
  } catch (error) {
    console.error('Error parsing or highlighting job logs JSON:', error);
    return jobLogs.value;
  }
});

const fetchJobDetails = async () => {
  console.log('Fetching job details for ID:', route.params.id);
  if (!route.params.id || route.params.id === 'true') {
    error.value = 'Invalid job ID';
    return;
  }
  loading.value = true;
  error.value = null;
  try {
    await store.dispatch('studio/fetchJobById', route.params.id);
    console.log('Job details fetched:', store.getters['studio/getCurrentJob']);
  } catch (err: any) {
    error.value = `Error fetching job details: ${err.message}`;
    console.error('Error fetching job details:', err);
  } finally {
    loading.value = false;
  }
};

const fetchJobLogs = async () => {
  if (!job.value || !job.value.id) return;
  try {
    const response = await apiClient.getJobLogs(job.value.id);
    jobLogs.value = typeof response.data === 'string' ? response.data : JSON.stringify(response.data);
  } catch (err: any) {
    console.error('Error fetching job logs:', err);
    jobLogs.value = `Error fetching job logs: ${err.message}`;
  }
};

const formatLabel = (key: string) => {
  return key.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
};

const formatValue = (key: string, value: any) => {
  if (key.includes('_at') && value) {
    return new Date(value).toLocaleString();
  }
  return value || 'N/A';
};

const isLinkableField = (key: string) => {
  return ['worker_type', 'config', 'pipeline_id', 'amber_id'].includes(key);
};

const getLinkForField = (key: string, value: any) => {
  if (!value) return '';
  switch (key) {
    case 'worker_type':
      return { name: 'DockerFileEditor', params: { id: value }, query: { returnToJobDetails: route.params.id } };
    case 'config':
      return { name: 'ConfigurationEditor', params: { id: value }, query: { returnToJobDetails: route.params.id } };
    case 'pipeline_id':
      return { name: 'PipelineEditor', params: { id: value }, query: { returnToJobDetails: route.params.id } };
    case 'amber_id':
      return { name: 'AmberStoreEditor', params: { id: value }, query: { returnToJobDetails: route.params.id } };
    default:
      return { name: 'JobDetail', params: { id: value } };
  }
};

const getLinkedItemName = (key: string, value: any) => {
  switch (key) {
    case 'worker_type':
      return store.getters['studio/getDockerFiles'].find((df: any) => df.id === value)?.name || value;
    case 'config':
      return store.getters['studio/getConfigurations'].find((c: any) => c.id === value)?.name || value;
    case 'pipeline_id':
      return store.getters['studio/getPipelines'].find((p: any) => p.id === value)?.name || value;
    case 'amber_id':
      return store.getters['studio/getAmberStores'].find((as: any) => as.id === value)?.name || value;
    default:
      return value;
  }
};

const getStatusClass = (status: string) => {
  switch (status.toLowerCase()) {
    case 'running':
      return 'bg-green-500 dark:bg-green-600';
    case 'completed':
      return 'bg-blue-500 dark:bg-blue-600';
    case 'failed':
      return 'bg-red-500 dark:bg-red-600';
    default:
      return 'bg-gray-500 dark:bg-gray-600';
  }
};

const toggleStateFileContent = () => {
  showStateFileContent.value = !showStateFileContent.value;
};

const toggleJobLogs = () => {
  showJobLogs.value = !showJobLogs.value;
  if (showJobLogs.value && !jobLogs.value) {
    fetchJobLogs();
  }
};

const startOrRestartJob = async () => {
  if (!job.value || !job.value.id) return;
  try {
    if (job.value.status === 'completed') {
      await store.dispatch('studio/restartJob', job.value.id);
    } else {
      await store.dispatch('studio/startJob', job.value.id);
    }
    await fetchJobDetails(); // Refresh job details after starting/restarting the job
  } catch (error) {
    console.error('Error starting/restarting job:', error);
    // Handle error (e.g., show an error message to the user)
  }
};

onMounted(async () => {
  console.log('JobDetail component mounted');
  await store.dispatch('studio/fetchAllData');
  console.log('All data fetched');
  await fetchJobDetails();
});

watch(() => route.params.id, async (newId, oldId) => {
  console.log(`Route param 'id' changed from ${oldId} to ${newId}`);
  if (newId !== oldId) {
    await fetchJobDetails();
    jobLogs.value = '';
    showJobLogs.value = false;
  }
});

watch(job, (newJob, oldJob) => {
  console.log('Job computed property changed:', newJob);
  if (newJob && !oldJob) {
    console.log('Job details loaded successfully');
  }
});

</script>

<style scoped>
pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  max-height: none;
}

h3.cursor-pointer:hover {
  color: #4a5568;
}

.transition-transform {
  transition: transform 0.2s ease-in-out;
}
</style>

<style>
/* Add these styles for syntax highlighting */
.hljs-string {
  color: #008000;
}

.hljs-number {
  color: #0000ff;
}

.hljs-boolean {
  color: #b22222;
}

.hljs-null {
  color: #808080;
}

.hljs-attr {
  color: #7f0055;
}

/* Dark mode syntax highlighting */
.dark .hljs-string {
  color: #7ec699;
}

.dark .hljs-number {
  color: #79b6f2;
}

.dark .hljs-boolean {
  color: #ff8b50;
}

.dark .hljs-null {
  color: #c678dd;
}

.dark .hljs-attr {
  color: #e5c07b;
}
</style>