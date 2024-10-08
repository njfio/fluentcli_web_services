<template>
  <div class="job-detail bg-gray-100 min-h-screen py-8">
    <div class="max-w-4xl mx-auto bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4 bg-indigo-600 text-white">
        <h2 class="text-2xl font-bold">Job Details</h2>
      </div>
      <div v-if="loading" class="p-6 text-center">
        <p class="text-gray-600">Loading job details...</p>
      </div>
      <div v-else-if="error" class="p-6 text-center">
        <p class="text-red-600">{{ error }}</p>
      </div>
      <div v-else-if="job" class="p-6">
        <div class="grid grid-cols-2 gap-4">
          <div v-for="(value, key) in jobDetails" :key="key" class="mb-4">
            <p class="font-semibold">{{ formatLabel(key) }}:</p>
            <p v-if="isLinkableField(key)" class="text-blue-600 hover:text-blue-800">
              <router-link :to="getLinkForField(key, value)">
                {{ getLinkedItemName(key, value) }}
              </router-link>
            </p>
            <p v-else>{{ formatValue(key, value) }}</p>
          </div>
        </div>
        <div class="mt-6">
          <h3 @click="toggleStateFileContent" class="font-semibold mb-2 cursor-pointer flex items-center">
            <span class="mr-2">State File Content:</span>
            <svg :class="{'rotate-180': showStateFileContent}" class="w-4 h-4 transition-transform" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </h3>
          <pre v-if="showStateFileContent" class="bg-gray-100 p-4 rounded overflow-x-auto" v-html="highlightedStateFileContent"></pre>
        </div>
        <div class="mt-6">
          <h3 @click="toggleJobLogs" class="font-semibold mb-2 cursor-pointer flex items-center">
            <span class="mr-2">Job Logs:</span>
            <svg :class="{'rotate-180': showJobLogs}" class="w-4 h-4 transition-transform" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </h3>
          <pre v-if="showJobLogs" class="bg-gray-100 p-4 rounded overflow-x-auto" v-html="highlightedJobLogs"></pre>
        </div>
      </div>
      <div v-else class="p-6 text-center">
        <p class="text-gray-600">No job details found.</p>
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
  const { id, worker_type, config, pipeline_id, amber_id, status } = job.value;
  const details = { id, worker_type, config, pipeline_id, amber_id, status };
  console.log('Computed jobDetails:', details);
  return details;
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

const toggleStateFileContent = () => {
  showStateFileContent.value = !showStateFileContent.value;
};

const toggleJobLogs = () => {
  showJobLogs.value = !showJobLogs.value;
  if (showJobLogs.value && !jobLogs.value) {
    fetchJobLogs();
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
.hljs-string { color: #008000; }
.hljs-number { color: #0000ff; }
.hljs-boolean { color: #b22222; }
.hljs-null { color: #808080; }
.hljs-attr { color: #7f0055; }
</style>