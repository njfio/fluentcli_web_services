<template>
  <div class="job-logs">
    <h2>Job Logs</h2>
    <pre v-if="jobLogs" v-html="highlightedLogs"></pre>
    <p v-else-if="error">{{ error }}</p>
    <p v-else>Loading job logs...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import apiClient from '@/services/apiClient';
import hljs from 'highlight.js/lib/core';
import plaintext from 'highlight.js/lib/languages/plaintext';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('plaintext', plaintext);

const route = useRoute();
const jobLogs = ref<string | null>(null);
const error = ref<string | null>(null);

const fetchJobLogs = async () => {
  try {
    const response = await apiClient.fetchJobLogs(route.params.id as string);
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
.job-logs {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
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