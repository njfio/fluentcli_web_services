<template>
  <div class="jobs">
    <h2>Jobs</h2>
    <button @click="showEditor = true" class="create-button">Create New Job</button>
    <table v-if="jobs.length">
      <thead>
        <tr>
          <th>ID</th>
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
          <button @click="deleteJob(job.id!)" class="delete-button">Delete</button>
          <button @click="startJob(job.id!)" class="start-button">Start</button>
          <button @click="stopJob(job.id!)" class="stop-button">Stop</button>
        </td>
      </tr>
      </tbody>
    </table>
    <p v-else>No jobs available.</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="isLoading" class="loading">Loading...</p>

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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import JobEditor from '@/components/studio/editors/JobEditor.vue';
import apiClient from '@/services/apiClient';

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
}

const jobs = ref<Job[]>([]);
const showEditor = ref(false);
const selectedJob = ref<Job | null>(null);
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

const getDockerFileName = (id: string) => {
  const docker = dockerFiles.value.find(d => d.id === id);
  return docker ? docker.name : 'Unknown';
};

const getConfigurationName = (id: string) => {
  const config = configurations.value.find(c => c.id === id);
  return config ? config.name : 'Unknown';
};

const getPipelineName = (id: string) => {
  const pipeline = pipelines.value.find(p => p.id === id);
  return pipeline ? pipeline.name : 'Unknown';
};

const getAmberStoreName = (id: string | null | undefined) => {
  if (!id) return 'N/A';
  const amber = amberStores.value.find(a => a.id === id);
  return amber ? amber.name : 'Unknown';
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
.jobs {
  padding: 20px;
}

table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 20px;
}

th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

th {
  background-color: #f2f2f2;
}

.create-button {
  background-color: #4CAF50;
  color: white;
  padding: 10px 20px;
  border: none;
  cursor: pointer;
}

.edit-button, .delete-button, .start-button, .stop-button {
  margin-right: 5px;
  padding: 5px 10px;
  cursor: pointer;
}

.edit-button {
  background-color: #008CBA;
  color: white;
}

.delete-button {
  background-color: #f44336;
  color: white;
}

.start-button {
  background-color: #4CAF50;
  color: white;
}

.stop-button {
  background-color: #555555;
  color: white;
}

.error {
  color: red;
}

.loading {
  color: #888;
}
</style>