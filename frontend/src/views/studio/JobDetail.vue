<template>
  <div class="job-detail bg-gray-100 min-h-screen py-8">
    <div class="max-w-4xl mx-auto bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4 bg-indigo-600 text-white">
        <h2 class="text-2xl font-bold">Job Details</h2>
      </div>
      <div v-if="job" class="p-6">
        <div class="grid grid-cols-2 gap-4">
          <div v-for="(value, key) in jobDetails" :key="key" class="mb-4">
            <p class="font-semibold">{{ formatLabel(key) }}:</p>
            <p v-if="isLinkableField(key)" class="text-blue-600 hover:text-blue-800">
              <router-link :to="getLinkForField(key, value)">
                {{ getDisplayValue(value) }}
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
      <div v-else class="p-6">Loading job details...</div>
    </div>
    <JobDataModal v-if="showJobDataModal && job?.id" :jobId="job.id" @close="showJobDataModal = false" />
    <JobLogsModal v-if="showJobLogsModal && job?.id" :jobId="job.id" @close="showJobLogsModal = false" />
  </div>
</template>


<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import apiClient from '@/services/apiClient';
import JobDataModal from '@/components/JobDataModal.vue';
import JobLogsModal from '@/components/JobLogsModal.vue';

interface Job {
  id?: string;
  config: string;
  amber_id?: string | null;
  state_file_content?: string;
  worker_type: string;
  status: string;
  pipeline_id: string;
}

const route = useRoute();
const job = ref<Job | null>(null);
const showJobDataModal = ref(false);
const showJobLogsModal = ref(false);

const jobDetails = computed(() => {
  if (!job.value) return {};
  const { id, worker_type, config, pipeline_id, amber_id, status } = job.value;
  return { id, worker_type, config, pipeline_id, amber_id, status };
});

const fetchJobDetails = async () => {
  try {
    const response = await apiClient.get(`/jobs/${route.params.id}`);
    job.value = response.data;
  } catch (error) {
    console.error('Error fetching job details:', error);
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
      return { name: 'Jobs', query: { editDockerFile: value } };
    case 'config':
      return { name: 'ConfigurationEditor', params: { id: value } };
    case 'pipeline_id':
      return { name: 'PipelineEditor', params: { id: value }, query: { returnToJobDetails: 'true' } };
    case 'amber_id':
      return { name: 'Jobs', query: { editAmberStore: value } };
    default:
      return { name: 'JobDetail', params: { id: value } };
  }
};

const getDisplayValue = (value: any) => {
  // Here you would fetch the name of the related item based on its ID
  // For now, we'll just return the ID
  return value;
};

const showJobData = () => {
  showJobDataModal.value = true;
};

const showJobLogs = () => {
  showJobLogsModal.value = true;
};

onMounted(fetchJobDetails);
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