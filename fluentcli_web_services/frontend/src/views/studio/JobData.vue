<template>
  <div class="job-data">
    <h2>Job Data</h2>
    <pre v-if="jobData" v-html="highlightedData"></pre>
    <p v-else-if="error">{{ error }}</p>
    <p v-else>Loading job data...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import apiClient from '@/services/apiClient';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('json', json);

const route = useRoute();
const jobData = ref<any>(null);
const error = ref<string | null>(null);

const fetchJobData = async () => {
  try {
    const response = await apiClient.fetchJobData(route.params.id as string);
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
.job-data {
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