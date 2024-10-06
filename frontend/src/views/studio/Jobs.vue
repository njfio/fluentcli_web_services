<template>
  <div class="container mx-auto px-4 py-8">
    <h2 class="text-2xl font-bold mb-4">Jobs</h2>
    <div class="overflow-x-auto bg-white shadow-md rounded-lg">
      <table class="min-w-full leading-normal">
        <thead>
          <tr>
            <th v-for="header in tableHeaders" :key="header" class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              {{ header }}
            </th>
            <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="job in sortedJobs" :key="job.id" class="hover:bg-gray-50">
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">{{ job.id }}</td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">{{ getDockerFileName(job.worker_type) }}</td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
              <a href="#" @click.prevent="handleConfigurationClick(job.config)" class="text-blue-600 hover:text-blue-800">
                {{ getConfigurationName(job.config) }}
              </a>
            </td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">{{ getPipelineName(job.pipeline_id) }}</td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">{{ job.status }}</td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
              <router-link :to="{ name: 'JobDetail', params: { id: job.id } }" class="text-indigo-600 hover:text-indigo-900 mr-2">
                View
              </router-link>
              <button @click="openJobEditor(job)" class="text-indigo-600 hover:text-indigo-900 mr-2">
                Edit
              </button>
              <button @click="job.id && deleteJob(job.id)" class="text-red-600 hover:text-red-900">
                Delete
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <JobEditor
      v-if="showEditor"
      :job="selectedJob"
      :dockerFiles="dockerFiles"
      :configurations="configurations"
      :pipelines="pipelines"
      :amberStores="amberStores"
      @save="handleSave"
      @cancel="showEditor = false"
    />
    <ConfigurationEditor
      v-if="editConfigurationId && editingConfiguration"
      :data="editingConfiguration"
      @save="handleSaveConfiguration"
      @cancel="closeConfigurationEditor"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import apiClient from '@/services/apiClient';
import JobEditor from '@/components/studio/editors/JobEditor.vue';
import ConfigurationEditor from '@/components/studio/editors/ConfigurationEditor.vue';

interface Job {
  id?: string;
  config: string;
  amber_id?: string | null;
  state_file_content?: string;
  data_path?: string;
  worker_type: string;
  triggers?: any;
  timers?: any;
  status: string;
  pipeline_id: string;
  results?: any;
  created_at?: string;
  updated_at?: string;
  started_at?: string;
  completed_at?: string;
}

interface Configuration {
  id?: string;
  name: string;
  data: any;
}

const route = useRoute();
const router = useRouter();
const jobs = ref<Job[]>([]);
const dockerFiles = ref<{ id: string; name: string }[]>([]);
const configurations = ref<{ id: string; name: string }[]>([]);
const pipelines = ref<{ id: string; name: string }[]>([]);
const amberStores = ref<{ id: string; name: string }[]>([]);
const showEditor = ref(false);
const selectedJob = ref<Job | null>(null);
const error = ref<string | null>(null);
const isLoading = ref(false);
const sortColumn = ref('id');
const sortDirection = ref<'asc' | 'desc'>('asc');
const editConfigurationId = ref<string | null>(null);
const editingConfiguration = ref<Configuration | null>(null);

const tableHeaders = ['ID', 'Worker Type', 'Configuration', 'Pipeline', 'Status'];

const sortedJobs = computed(() => {
  return [...jobs.value].sort((a, b) => {
    const aValue = a[sortColumn.value as keyof Job];
    const bValue = b[sortColumn.value as keyof Job];
    if (aValue < bValue) return sortDirection.value === 'asc' ? -1 : 1;
    if (aValue > bValue) return sortDirection.value === 'asc' ? 1 : -1;
    return 0;
  });
});

const fetchJobs = async () => {
  isLoading.value = true;
  error.value = null;
  try {
    const response = await apiClient.get('/jobs');
    jobs.value = response.data;
  } catch (err: any) {
    error.value = 'Failed to fetch jobs. Please try again.';
    console.error('Error fetching jobs:', err);
  } finally {
    isLoading.value = false;
  }
};

const fetchRelatedData = async () => {
  try {
    const [dockerResponse, configResponse, pipelineResponse, amberResponse] = await Promise.all([
      apiClient.get('/docker_files'),
      apiClient.get('/configurations'),
      apiClient.get('/pipelines'),
      apiClient.get('/amber_store')
    ]);
    dockerFiles.value = dockerResponse.data;
    configurations.value = configResponse.data;
    pipelines.value = pipelineResponse.data;
    amberStores.value = amberResponse.data;
  } catch (err: any) {
    console.error('Error fetching related data:', err);
  }
};

const openJobEditor = (job: Job) => {
  selectedJob.value = { ...job };
  showEditor.value = true;
};

const handleSave = async (job: Job) => {
  isLoading.value = true;
  error.value = null;
  try {
    if (job.id) {
      await apiClient.put(`/jobs/${job.id}`, job);
    } else {
      await apiClient.post('/jobs', job);
    }
    await fetchJobs();
    showEditor.value = false;
  } catch (err: any) {
    error.value = 'Failed to save job. Please try again.';
    console.error('Error saving job:', err);
  } finally {
    isLoading.value = false;
  }
};

const deleteJob = async (id: string) => {
  if (!confirm('Are you sure you want to delete this job?')) return;
  isLoading.value = true;
  error.value = null;
  try {
    await apiClient.delete(`/jobs/${id}`);
    await fetchJobs();
  } catch (err: any) {
    error.value = 'Failed to delete job. Please try again.';
    console.error('Error deleting job:', err);
  } finally {
    isLoading.value = false;
  }
};



const getDockerFileName = (id: string) => {
  const docker = dockerFiles.value.find(d => d.id === id);
  return docker ? docker.name : 'Unknown';
};

const handleConfigurationClick = async (configId: string) => {
  if (configId) {
    console.log(`Configuration ID clicked: ${configId}`);
    editConfigurationId.value = configId;
    try {
      const response = await apiClient.get(`/configurations/${configId}`);
      editingConfiguration.value = response.data;
      console.log('Configuration loaded:', editingConfiguration.value);
    } catch (error) {
      console.error('Error fetching configuration:', error);
    }
  }
};

const openConfigurationEditor = async (id: string) => {
  try {
    const response = await apiClient.get(`/configurations/${id}`);
    editingConfiguration.value = response.data;
    showEditor.value = false; // Close the job editor if it's open
    router.push({ query: { ...route.query, editConfiguration: id } });
    console.log('Opened Configuration Editor with:', response.data);
  } catch (error) {
    console.error('Error fetching configuration:', error);
  }
};

const closeConfigurationEditor = () => {
  editConfigurationId.value = null;
  editingConfiguration.value = null;
  router.replace({ query: {} });
  console.log('Configuration Editor closed.');
};

const handleSaveConfiguration = async (configuration: Configuration) => {
  try {
    if (configuration.id) {
      await apiClient.put(`/configurations/${configuration.id}`, configuration);
    } else {
      const response = await apiClient.post('/configurations', configuration);
      configuration.id = response.data.id;
    }
    await fetchJobs();
    closeConfigurationEditor();
    console.log('Configuration saved:', configuration);
  } catch (error) {
    console.error('Error updating configuration:', error);
  }
};

const getConfigurationName = (id: string) => {
  const config = configurations.value.find(c => c.id === id);
  return config ? config.name : 'Unknown';
};

const getPipelineName = (id: string) => {
  const pipeline = pipelines.value.find(p => p.id === id);
  return pipeline ? pipeline.name : 'Unknown';
};

onMounted(async () => {
  await fetchJobs();
  await fetchRelatedData();
  const { editConfiguration } = route.query;
  if (typeof editConfiguration === 'string') {
    console.log(`Initial route query editConfiguration: ${editConfiguration}`);
    await openConfigurationEditor(editConfiguration);
  }
});

watch(
  () => editConfigurationId.value,
  (newValue) => {
    console.log('editConfigurationId changed:', newValue);
    console.log('editingConfiguration:', editingConfiguration.value);
  }
);
</script>