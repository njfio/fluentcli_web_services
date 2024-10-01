<template>
  <div class="modal">
    <div class="modal-content">
      <h3>Job Logs</h3>
      <pre v-if="jobLogs" v-html="highlightedLogs"></pre>
      <p v-else-if="error">{{ error }}</p>
      <p v-else>Loading job logs...</p>
      <button @click="$emit('close')">Close</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import apiClient from '@/services/apiClient';
import hljs from 'highlight.js/lib/core';
import plaintext from 'highlight.js/lib/languages/plaintext';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('plaintext', plaintext);

const props = defineProps<{
  jobId: string;
}>();

const jobLogs = ref<string | null>(null);
const error = ref<string | null>(null);

const fetchJobLogs = async () => {
  try {
    const response = await apiClient.get(`/jobs/${props.jobId}/logs`);
    jobLogs.value = JSON.stringify(response.data, null, 2);
    console.log('Fetched job logs:', jobLogs.value);
  } catch (err) {
    error.value = 'Failed to fetch job logs';
    console.error('Error fetching job logs:', err);
  }
};

const highlightedLogs = computed(() => {
  if (!jobLogs.value) return '';
  try {
    return hljs.highlight(jobLogs.value, { language: 'json' }).value;
  } catch (err) {
    console.error('Error highlighting logs:', err);
    return jobLogs.value;
  }
});

onMounted(fetchJobLogs);
</script>

<style scoped>
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
  width: 80%;
  max-width: 800px;
  max-height: 80vh;
  overflow-y: auto;
}

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  background-color: #f8f8f8;
  border: 1px solid #ddd;
  padding: 10px;
  border-radius: 4px;
  max-height: 600px;
  overflow-y: auto;
}
</style>