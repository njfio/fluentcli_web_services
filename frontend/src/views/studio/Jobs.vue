<template>
  <div class="jobs">
    <div class="jobs-header">
      <h2>Jobs</h2>
      <button @click="showEditor = true" class="add-button">Create New Job</button>
    </div>
    
    <table v-if="jobs.length" class="job-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>Worker Type</th>
          <th>Configuration</th>
          <th>Pipeline</th>
          <th>Amber Store</th>
          <th>Status</th>
          <th>Created At</th>
          <th>Updated At</th>
          <th>Started At</th>
          <th>Completed At</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
<tr v-for="job in jobs" :key="job.id">
  <td><router-link :to="`/studio/jobs/${job.id}`">{{ job.id }}</router-link></td>
  <td>{{ getDockerFileName(job.worker_type) }}</td>
  <td>{{ getConfigurationName(job.config) }}</td>
  <td>{{ getPipelineName(job.pipeline_id) }}</td>
  <td>{{ getAmberStoreName(job.amber_id) }}</td>
  <td>{{ job.status }}</td>
  <td>{{ formatDate(job.created_at) }}</td>
  <td>{{ formatDate(job.updated_at) }}</td>
  <td>{{ formatDate(job.started_at) }}</td>
  <td>{{ formatDate(job.completed_at) }}</td>
  <td>
    <button @click="openJobEditor(job)" class="edit-button">Edit</button>
    <button @click="deleteJob(job.id!)" class="delete-button">Delete</button>
    <button @click="startJob(job.id!)" class="start-button">Start</button>
    <button @click="stopJob(job.id!)" class="stop-button">Stop</button>
    <router-link :to="`/studio/jobs/${job.id}/data`" class="data-button">Data</router-link>
    <router-link :to="`/studio/jobs/${job.id}/logs`" class="log-button">Logs</router-link>
  </td>
</tr>
      </tbody>
    </table>
    <p v-else class="no-jobs">No jobs available.</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="isLoading" class="loading">Loading...</p>

    <div v-if="showEditor" class="modal">
      <div class="modal-content">
        <JobEditor
          :job="selectedJob"
          :dockerFiles="dockerFiles"
          :configurations="configurations"
          :pipelines="pipelines"
          :amberStores="amberStores"
          @save="handleSave"
          @cancel="showEditor = false"
        />
      </div>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, onMounted } from 'vue';
import JobEditor from '@/components/studio/editors/JobEditor.vue';
import apiClient from '@/services/apiClient';
import { formatDate } from '@/utils/dateFormatter';

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

const jobs = ref<Job[]>([]);
const showEditor = ref(false);
const selectedJob = ref<Job | null>(null);
const error = ref<string | null>(null);
const isLoading = ref(false);
const openJobEditor = (job: Job) => {
  selectedJob.value = { ...job };
  showEditor.value = true;
};

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
}


onMounted(() => {
  fetchJobs();
  fetchRelatedData();
});
</script>


<style scoped>
.jobs {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.jobs-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.add-button {
  background-color: #3498db;
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.3s ease;
}

.add-button:hover {
  background-color: #2980b9;
}

.job-table {
  width: 100%;
  border-collapse: collapse;
}

.job-table th,
.job-table td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

.job-table th {
  background-color: #f2f2f2;
}

.edit-button, .delete-button, .start-button, .stop-button, .data-button, .log-button {
  background-color: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  margin-right: 5px;
  transition: color 0.3s ease;
}

.edit-button { color: #3498db; }
.edit-button:hover { color: #2980b9; }

.delete-button { color: #e74c3c; }
.delete-button:hover { color: #c0392b; }

.start-button { color: #2ecc71; }
.start-button:hover { color: #27ae60; }

.stop-button { color: #95a5a6; }
.stop-button:hover { color: #7f8c8d; }

.data-button { color: #f39c12; }
.data-button:hover { color: #d35400; }

.log-button { color: #9b59b6; }
.log-button:hover { color: #8e44ad; }

.no-jobs {
  text-align: center;
  color: #7f8c8d;
  margin-top: 50px;
}

.error {
  color: #e74c3c;
  margin-top: 10px;
}

.loading {
  color: #3498db;
  margin-top: 10px;
}

.modal {
  position: fixed;
  z-index: 1;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  overflow: auto;
  background-color: rgba(0,0,0,0.4);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-content {
  background-color: #fefefe;
  padding: 20px;
  border: 1px solid #888;
  width: 90%;
  max-width: 1200px;
  max-height: 90vh;
  overflow-y: auto;
  border-radius: 5px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}
</style>