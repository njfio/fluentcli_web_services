<template>
  <div class="modal">
    <div class="modal-content">
      <h3>Job Data</h3>
      <pre v-if="jobData" v-html="highlightedData"></pre>
      <p v-else-if="error">{{ error }}</p>
      <p v-else>Loading job data...</p>
      <button @click="$emit('close')">Close</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import apiClient from '@/services/apiClient';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('json', json);

const props = defineProps<{
  jobId: string;
}>();

const jobData = ref<any>(null);
const error = ref<string | null>(null);

const fetchJobData = async () => {
  try {
    const response = await apiClient.get(`/jobs/${props.jobId}/data`);
    jobData.value = response.data;
  } catch (err) {
    error.value = 'Failed to fetch job data';
    console.error('Error fetching job data:', err);
  }
};

const highlightedData = computed(() => {
  if (!jobData.value) return '';
  const formatted = JSON.stringify(jobData.value, null, 2);
  return hljs.highlight(formatted, { language: 'json' }).value;
});

onMounted(fetchJobData);
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