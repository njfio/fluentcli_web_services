<template>
  <div class="state-files">
    <h1 class="text-2xl font-bold mb-4">State Files</h1>
    <div v-if="loading" class="text-center py-4">
      <p>Loading state files...</p>
    </div>
    <div v-else-if="error" class="text-center py-4 text-red-600">
      <p>Error: {{ error }}</p>
    </div>
    <div v-else-if="jobsWithStateFiles.length === 0" class="text-center py-4">
      <p>No state files found.</p>
    </div>
    <table v-else class="min-w-full bg-white">
      <thead>
        <tr>
          <th class="py-2 px-4 border-b">Job ID</th>
          <th class="py-2 px-4 border-b">Content Size (bytes)</th>
          <th class="py-2 px-4 border-b">State File Content</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="job in jobsWithStateFiles" :key="job.id">
          <td class="py-2 px-4 border-b">
            <router-link :to="{ name: 'JobDetail', params: { id: job.id } }" class="text-blue-500 hover:text-blue-700">
              {{ job.id }}
            </router-link>
          </td>
          <td class="py-2 px-4 border-b">{{ getContentSize(job.state_file_content) }}</td>
          <td class="py-2 px-4 border-b">
            <div class="relative">
              <button @click="toggleExpand(job.id)" class="text-blue-500 hover:text-blue-700">
                {{ isExpanded(job.id) ? 'Collapse' : 'Expand' }}
              </button>
              <pre
                v-if="isExpanded(job.id)"
                class="whitespace-pre-wrap mt-2 p-2 bg-gray-100 rounded"
                v-html="getHighlightedJSON(job.state_file_content)"
              ></pre>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { Job } from '@/store/modules/studio';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';

hljs.registerLanguage('json', json);

export default defineComponent({
  name: 'StateFiles',
  setup() {
    console.log('StateFiles setup function called');
    const store = useStore();
    const router = useRouter();
    const loading = ref(true);
    const error = ref<string | null>(null);
    const expandedJobs = ref<Set<string>>(new Set());

    const jobsWithStateFiles = computed(() => {
      console.log('Computing jobsWithStateFiles');
      const jobs: Job[] = store.getters['studio/getJobs'];
      console.log('Jobs from store:', jobs);
      return jobs.filter(job => job.state_file_content != null);
    });

    const getContentSize = (content: any): number => {
      if (typeof content === 'string') {
        return new Blob([content]).size;
      } else if (typeof content === 'object') {
        return new Blob([JSON.stringify(content)]).size;
      }
      return 0;
    };

    const toggleExpand = (jobId: string) => {
      if (expandedJobs.value.has(jobId)) {
        expandedJobs.value.delete(jobId);
      } else {
        expandedJobs.value.add(jobId);
      }
    };

    const isExpanded = (jobId: string): boolean => {
      return expandedJobs.value.has(jobId);
    };

    const highlightJSON = (content: any): string => {
      if (!content) return 'No content available';
      
      let jsonString: string;
      if (typeof content === 'string') {
        try {
          // If it's a string, try to parse it as JSON
          JSON.parse(content);
          jsonString = content;
        } catch (e) {
          // If parsing fails, it's not a valid JSON string
          return `Invalid JSON: ${content}`;
        }
      } else if (typeof content === 'object') {
        // If it's already an object, stringify it
        try {
          jsonString = JSON.stringify(content, null, 2);
        } catch (e) {
          return `Error stringifying object: ${e}`;
        }
      } else {
        return `Unsupported content type: ${typeof content}`;
      }

      try {
        return hljs.highlight(jsonString, { language: 'json' }).value;
      } catch (e) {
        console.error('Error highlighting JSON:', e);
        return jsonString; // Return non-highlighted but formatted JSON if highlighting fails
      }
    };

    const getHighlightedJSON = computed(() => {
      return (content: any) => highlightJSON(content);
    });

    onMounted(async () => {
      console.log('StateFiles component mounted');
      try {
        console.log('Dispatching fetchJobs action');
        await store.dispatch('studio/fetchJobs');
        console.log('fetchJobs action completed');
        const jobs = store.getters['studio/getJobs'];
        console.log('Jobs after fetching:', jobs);
      } catch (err) {
        console.error('Error fetching jobs:', err);
        error.value = err instanceof Error ? err.message : 'An unknown error occurred';
      } finally {
        loading.value = false;
      }
    });

    return {
      jobsWithStateFiles,
      loading,
      error,
      getContentSize,
      toggleExpand,
      isExpanded,
      getHighlightedJSON,
    };
  },
});
</script>

<style scoped>
.state-files {
  padding: 20px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 8px;
  border-bottom: 1px solid #ddd;
}

th {
  background-color: #f2f2f2;
  font-weight: bold;
}

pre {
  max-height: none;
  overflow-y: visible;
  background-color: #f8f8f8;
  padding: 8px;
  border-radius: 4px;
}
</style>

<style>
/* Add these styles for syntax highlighting */
.hljs-string { color: #008000; }
.hljs-number { color: #0000ff; }
.hljs-boolean { color: #b22222; }
.hljs-null { color: #808080; }
.hljs-attr { color: #7f0055; }
</style>