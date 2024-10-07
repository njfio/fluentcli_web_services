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
          <h3 class="font-semibold mb-2">State File Content:</h3>
          <pre class="bg-gray-100 p-4 rounded">{{ job.state_file_content }}</pre>
        </div>
        <div class="mt-6 flex space-x-4">
          <button @click="showJobData" class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded">
            Show Job Data
          </button>
          <button @click="showJobLogs" class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded">
            Show Job Logs
          </button>
        </div>
      </div>
      <div v-else class="p-6 text-center">
        <p class="text-gray-600">No job details found.</p>
      </div>
    </div>
    <JobDataModal v-if="showJobDataModal && job?.id" :jobId="job.id" @close="showJobDataModal = false" />
    <JobLogsModal v-if="showJobLogsModal && job?.id" :jobId="job.id" @close="showJobLogsModal = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import { useStore } from 'vuex';
import JobDataModal from '@/components/JobDataModal.vue';
import JobLogsModal from '@/components/JobLogsModal.vue';

const route = useRoute();
const store = useStore();
const showJobDataModal = ref(false);
const showJobLogsModal = ref(false);
const loading = ref(false);
const error = ref<string | null>(null);

const job = computed(() => store.getters['studio/getCurrentJob']);

const jobDetails = computed(() => {
  if (!job.value) return {};
  const { id, worker_type, config, pipeline_id, amber_id, status } = job.value;
  return { id, worker_type, config, pipeline_id, amber_id, status };
});

const fetchJobDetails = async () => {
  if (!route.params.id || route.params.id === 'true') {
    error.value = 'Invalid job ID';
    return;
  }
  loading.value = true;
  error.value = null;
  try {
    await store.dispatch('studio/fetchJobById', route.params.id);
  } catch (err: any) {
    error.value = `Error fetching job details: ${err.message}`;
    console.error('Error fetching job details:', err);
  } finally {
    loading.value = false;
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

const showJobData = () => {
  showJobDataModal.value = true;
};

const showJobLogs = () => {
  showJobLogsModal.value = true;
};

onMounted(async () => {
  await store.dispatch('studio/fetchAllData');
  await fetchJobDetails();
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