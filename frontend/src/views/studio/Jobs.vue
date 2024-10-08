<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 py-8">
    <div class="container mx-auto px-4">
      <h2 class="text-3xl font-bold mb-6 text-gray-800">Jobs</h2>
      <div class="bg-white shadow-lg rounded-lg overflow-hidden">
        <div class="overflow-x-auto">
          <table class="min-w-full leading-normal">
            <thead>
              <tr class="bg-indigo-600 text-white">
                <th v-for="header in tableHeaders" :key="header" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wider">
                  {{ header }}
                </th>
                <th class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(job, index) in sortedJobs" :key="job.id" 
                  class="hover:bg-indigo-50 transition-colors duration-150 ease-in-out"
                  :class="{'bg-gray-50': index % 2 === 0}"
                  :style="{ animationDelay: `${index * 50}ms` }">
                <td class="px-5 py-5 border-b border-gray-200 text-sm">{{ job.id }}</td>
                <td class="px-5 py-5 border-b border-gray-200 text-sm">{{ getDockerFileName(job.worker_type) }}</td>
                <td class="px-5 py-5 border-b border-gray-200 text-sm">
                  <a href="#" @click.prevent="handleConfigurationClick(job.config)" class="text-indigo-600 hover:text-indigo-900 hover:underline">
                    {{ getConfigurationName(job.config) }}
                  </a>
                </td>
                <td class="px-5 py-5 border-b border-gray-200 text-sm">{{ getPipelineName(job.pipeline_id) }}</td>
                <td class="px-5 py-5 border-b border-gray-200 text-sm">
                  <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full"
                        :class="{
                          'bg-green-100 text-green-800': job.status === 'completed',
                          'bg-yellow-100 text-yellow-800': job.status === 'pending',
                          'bg-blue-100 text-blue-800': job.status === 'running',
                          'bg-red-100 text-red-800': job.status === 'failed'
                        }">
                    {{ job.status }}
                  </span>
                </td>
                <td class="px-5 py-5 border-b border-gray-200 text-sm">
                  <div class="flex space-x-2">
                    <router-link :to="{ name: 'JobDetail', params: { id: job.id } }" 
                                 class="text-indigo-600 hover:text-indigo-900 transition-colors duration-150">
                      View
                    </router-link>
                    <button @click="openJobEditor(job)" 
                            class="text-indigo-600 hover:text-indigo-900 transition-colors duration-150">
                      Edit
                    </button>
                    <button @click="job.id && deleteJob(job.id)" 
                            class="text-red-600 hover:text-red-900 transition-colors duration-150">
                      Delete
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
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
import { StudioConfiguration } from '@/store/modules/studio';

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

const route = useRoute();
const router = useRouter();
const jobs = ref<Job[]>([]);
const dockerFiles = ref<{ id: string; name: string }[]>([]);
const configurations = ref<StudioConfiguration[]>([]);
const pipelines = ref<{ id: string; name: string }[]>([]);
const amberStores = ref<{ id: string; name: string }[]>([]);
const showEditor = ref(false);
const selectedJob = ref<Job | null>(null);
const error = ref<string | null>(null);
const isLoading = ref(false);
const sortColumn = ref('id');
const sortDirection = ref<'asc' | 'desc'>('asc');
const editConfigurationId = ref<string | null>(null);
const editingConfiguration = ref<StudioConfiguration | null>(null);

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
    const response = await apiClient.fetchJobs();
    jobs.value = response.data;
  } catch (err: any) {
    error.value = 'Failed to fetch jobs. Please try again.';
    console.error('Error fetching jobs:', err);
  } finally {
    isLoading.value = false;
  }
};

const fetchRelatedData = async () => {
  const fetchData = async (fetchFunction: () => Promise<any>, errorMessage: string) => {
    try {
      const response = await fetchFunction();
      return response.data;
    } catch (err: any) {
      console.error(errorMessage, err);
      return [];
    }
  };

  dockerFiles.value = await fetchData(apiClient.fetchDockerFiles, 'Error fetching docker files:');
  configurations.value = await fetchData(apiClient.fetchConfigurations, 'Error fetching configurations:');
  pipelines.value = await fetchData(apiClient.fetchPipelines, 'Error fetching pipelines:');
  amberStores.value = await fetchData(apiClient.fetchAmberStores, 'Error fetching amber stores:');
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
      await apiClient.updateJob(job.id, job);
    } else {
      await apiClient.createJob(job);
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
    await apiClient.deleteJob(id);
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
      const response = await apiClient.fetchConfigurations();
      const configuration = response.data.find((config: StudioConfiguration) => config.id === configId);
      if (configuration) {
        editingConfiguration.value = configuration;
        console.log('Configuration loaded:', editingConfiguration.value);
      } else {
        console.error('Configuration not found');
      }
    } catch (error) {
      console.error('Error fetching configuration:', error);
    }
  }
};

const openConfigurationEditor = async (id: string) => {
  try {
    const response = await apiClient.fetchConfigurations();
    const configuration = response.data.find((config: StudioConfiguration) => config.id === id);
    if (configuration) {
      editingConfiguration.value = configuration;
      showEditor.value = false; // Close the job editor if it's open
      router.push({ query: { ...route.query, editConfiguration: id } });
      console.log('Opened Configuration Editor with:', configuration);
    } else {
      console.error('Configuration not found');
    }
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

const handleSaveConfiguration = async (configuration: StudioConfiguration) => {
  try {
    if (configuration.id) {
      await apiClient.updateConfiguration(configuration.id, configuration);
    } else {
      const response = await apiClient.createConfiguration(configuration);
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

<style scoped>
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

tbody tr {
  animation: fadeIn 0.3s ease-out forwards;
  opacity: 0;
}
</style>