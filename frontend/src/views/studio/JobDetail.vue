<template>
  <div class="job-detail">
    <h2>Job Details</h2>
    <div v-if="job" class="job-info">
      <p><strong>ID:</strong> {{ job.id }}</p>
      <p><strong>Worker Type:</strong> {{ job.worker_type }}</p>
      <p><strong>Configuration:</strong> {{ job.config }}</p>
      <p><strong>Pipeline:</strong> {{ job.pipeline_id }}</p>
      <p><strong>Amber Store:</strong> {{ job.amber_id || 'N/A' }}</p>
      <p><strong>Status:</strong> {{ job.status }}</p>
      <p><strong>Created At:</strong> {{ formatDate(job.created_at) }}</p>
      <p><strong>Updated At:</strong> {{ formatDate(job.updated_at) }}</p>
      <p><strong>Started At:</strong> {{ formatDate(job.started_at) }}</p>
      <p><strong>Completed At:</strong> {{ formatDate(job.completed_at) }}</p>
      <div>
        <strong>State File Content:</strong>
<pre v-html="highlightJSON(job.state_file_content)"></pre>
      </div>
      <div>
        <strong>Results:</strong>
<pre v-html="highlightJSON(job.results)"></pre>
      </div>
    </div>
    <div v-else class="loading">Loading job details...</div>
    <div class="job-actions">
      <button @click="showJobData = true" class="data-button">Get Job Data</button>
      <button @click="showJobLogs = true" class="log-button">Get Job Logs</button>
    </div>

    <JobDataModal v-if="showJobData" :jobId="route.params.id as string" @close="showJobData = false" />
    <JobLogsModal v-if="showJobLogs" :jobId="route.params.id as string" @close="showJobLogs = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import apiClient from '@/services/apiClient';
import { formatDate } from '@/utils/dateFormatter';
import JobDataModal from '@/components/JobDataModal.vue';
import JobLogsModal from '@/components/JobLogsModal.vue';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('json', json);

const highlightJSON = (data: any) => {
  try {
    const formatted = typeof data === 'string' ? JSON.parse(data) : data;
    return hljs.highlight(JSON.stringify(formatted, null, 2), { language: 'json' }).value;
  } catch {
    return JSON.stringify(data, null, 2);
  }
};
const route = useRoute();
const job = ref<any>(null);
const showJobData = ref(false);
const showJobLogs = ref(false);

const fetchJobDetails = async () => {
  try {
    const response = await apiClient.get(`/jobs/${route.params.id}`);
    job.value = response.data;
  } catch (error) {
    console.error('Error fetching job details:', error);
  }
};


onMounted(() => {
  fetchJobDetails();
});
</script>


<style scoped>
.job-detail {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.job-info p {
  margin-bottom: 10px;
}

.job-actions {
  margin-top: 20px;
}

.edit-button, .data-button, .log-button {
  background-color: #3498db;
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 1rem;
  margin-right: 10px;
  transition: background-color 0.3s ease;
}

.edit-button:hover, .data-button:hover, .log-button:hover {
  background-color: #2980b9;
}

.loading {
  text-align: center;
  color: #7f8c8d;
  margin-top: 50px;
}


pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  background-color: #f8f8f8;
  border: 1px solid #ddd;
  padding: 10px;
  border-radius: 4px;
  max-height: 400px;
  overflow-y: auto;
}
</style>