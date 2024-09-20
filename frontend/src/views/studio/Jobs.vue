<template>
  <div class="jobs">
    <h2>Jobs</h2>
    <button @click="showEditor = true" class="create-button">Create New Job</button>
    <table v-if="jobs.length">
      <thead>
        <tr>
          <th>URI</th>
          <th>Worker Type</th>
          <th>Configuration</th>
          <th>Pipeline</th>
          <th>Amber Store</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="job in jobs" :key="job.id">
          <td>{{ job.id }}</td>
          <td>{{ getDockerFileName(job.worker_type) }}</td>
          <td>{{ getConfigurationName(job.config) }}</td>
          <td>{{ getPipelineName(job.pipeline_id) }}</td>
          <td>{{ getAmberStoreName(job.amber_id) }}</td>
          <td>{{ job.status }}</td>
          <td>
            <button @click="editJob(job)" class="edit-button">Edit</button>
            <button @click="job.id && deleteJob(job.id)" class="delete-button">Delete</button>
            <button @click="job.id && startJob(job.id)" class="start-button">Start</button>
            <button @click="job.id && stopJob(job.id)" class="stop-button">Stop</button>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-else>No jobs available.</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="isLoading" class="loading">Loading...</p>

    <JobEditor
      v-if="showEditor"
      :data="selectedJob"
      :dockerFiles="dockerFiles"
      :configurations="configurations"
      :pipelines="pipelines"
      :amberStores="amberStores"
      @save="handleSave"
      @cancel="showEditor = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import JobEditor from '@/components/studio/editors/JobEditor.vue';
import apiClient from '@/services/apiClient';

interface Job {
    id?: string;
  config: any;
  amber_id?: string | null | undefined;
  state_file_content?: string | null;
  data_path?: string;
  worker_type: string;
  triggers?: any;
  timers?: any;
  status: string;
  pipeline_id: string;
  results?: any;
}

const jobs = ref<Job[]>([]);
const showEditor = ref(false);
const selectedJob = ref<Job>({
  worker_type: '',
  config: {},
  data_path: '',
  amber_id: null,
  status: '',
  pipeline_id: '',
  state_file_content: null,
  triggers: null,
  timers: null,
  results: null
});
const error = ref<string | null>(null);
const isLoading = ref(false);

const dockerFiles = ref<{ id: string; name: string }[]>([]);
const configurations = ref<{ id: string; name: string }[]>([]);
const pipelines = ref<{ id: string; name: string }[]>([]);
const amberStores = ref<{ id: string; name: string }[]>([]);

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
    const [dockerFilesRes, configurationsRes, pipelinesRes, amberStoresRes] = await Promise.all([
      apiClient.get<{ id: string; name: string }[]>('/docker_files'),
      apiClient.get<{ id: string; name: string }[]>('/configurations'),
      apiClient.get<{ id: string; name: string }[]>('/pipelines'),
      apiClient.get<{ id: string; name: string }[]>('/amber_store')
    ]);
    dockerFiles.value = dockerFilesRes.data;
    configurations.value = configurationsRes.data;
    pipelines.value = pipelinesRes.data;
    amberStores.value = amberStoresRes.data;
  } catch (err: any) {
    console.error('Error fetching related data:', err);
  }
};

const getDockerFileName = (id: string) => {
  const dockerFile = dockerFiles.value.find(df => df.id === id);
  return dockerFile ? dockerFile.name : 'Unknown';
};

const getConfigurationName = (id: string) => {
  const configuration = configurations.value.find(c => c.id === id);
  return configuration ? configuration.name : 'Unknown';
};

const getPipelineName = (id: string) => {
  const pipeline = pipelines.value.find(p => p.id === id);
  return pipeline ? pipeline.name : 'Unknown';
};

const getAmberStoreName = (id: string | null | undefined) => {
  if (!id) return 'N/A';
  const amberStore = amberStores.value.find(as => as.id === id);
  return amberStore ? amberStore.name : 'Unknown';
};

const editJob = (job: Job) => {
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
      const newJob = {
        config: job.config,
        amber_id: job.amber_id,
        state_file_content: job.state_file_content,
        data_path: job.data_path,
        worker_type: job.worker_type,
        triggers: job.triggers,
        timers: job.timers,
        status: job.status,
        pipeline_id: job.pipeline_id,
        results: job.results
      };
      console.log('Job data being sent:', newJob);
      await apiClient.post('/jobs', newJob);
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

const startJob = async (id: string) => {
  isLoading.value = true;
  error.value = null;
  try {
    await apiClient.post(`/jobs/${id}/start`);
    await fetchJobs();
  } catch (err: any) {
    error.value = 'Failed to start job. Please try again.';
    console.error('Error starting job:', err);
  } finally {
    isLoading.value = false;
  }
};

const stopJob = async (id: string) => {
  isLoading.value = true;
  error.value = null;
  try {
    await apiClient.post(`/jobs/${id}/stop`);
    await fetchJobs();
  } catch (err: any) {
    error.value = 'Failed to stop job. Please try again.';
    console.error('Error stopping job:', err);
  } finally {
    isLoading.value = false;
  }
};

onMounted(() => {
  fetchJobs();
  fetchRelatedData();
});
</script>

<style scoped>
/* ... (keep existing styles) ... */
</style>