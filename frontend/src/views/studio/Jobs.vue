<template>
  <div class="jobs">
    <h1 class="text-2xl font-bold mb-4 dark:text-white">Jobs</h1>
    <div class="mb-4 flex justify-between items-center">
      <input v-model="searchQuery" type="text" placeholder="Search jobs..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 dark:bg-gray-700 dark:text-white dark:border-gray-600" />
      <button @click="createNewJob"
        class="bg-primary-500 text-white px-4 py-2 rounded-lg hover:bg-primary-600 transition-colors duration-200">
        Create New Job
      </button>
    </div>
    <div class="overflow-x-auto shadow-md rounded-lg">
      <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead class="bg-primary-600 dark:bg-primary-800">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">ID
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">
              Docker File</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Configuration</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Pipeline</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">
              Status</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">
              Created At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">
              Updated At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">
              Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
          <tr v-for="job in filteredJobs" :key="job.id" class="dark:hover:bg-gray-700">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
              <router-link :to="{ name: 'JobDetail', params: { id: job.id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300">
                <span :title="job.id" class="truncate block max-w-xs">{{ job.id }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              <router-link :to="{ name: 'DockerFileEditor', params: { id: job.worker_type } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300">
                <span :title="job.worker_type" class="truncate block max-w-xs">{{ job.worker_type }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              <router-link :to="{ name: 'ConfigurationEditor', params: { id: job.config } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300">
                <span :title="job.config" class="truncate block max-w-xs">{{ job.config }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              <router-link :to="{ name: 'PipelineEditor', params: { id: job.pipeline_id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300">
                <span :title="job.pipeline_id" class="truncate block max-w-xs">{{ job.pipeline_id }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              <span :class="getStatusClass(job.status)">{{ job.status }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              {{ formatDate(job.created_at) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300">
              {{ formatDate(job.updated_at) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <router-link :to="{ name: 'JobDetail', params: { id: job.id } }"
                class="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300 mr-2">View</router-link>
              <button @click="deleteJob(job.id)"
                class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300 mr-2">Delete</button>
              <button v-if="canStartOrRestartJob(job)" @click="startOrRestartJob(job)"
                class="text-green-600 hover:text-green-900 dark:text-green-400 dark:hover:text-green-300">
                {{ job.status === 'completed' ? 'Restart' : 'Start' }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { formatDate } from '@/utils/dateFormatter';

export default defineComponent({
  name: 'Jobs',
  setup() {
    const store = useStore();
    const router = useRouter();
    const searchQuery = ref('');

    const jobs = computed(() => store.getters['studio/getJobs']);
    const filteredJobs = computed(() => {
      return jobs.value.filter((job: any) =>
        job.id.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        job.worker_type.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        job.config.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        job.pipeline_id.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        job.status.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    onMounted(() => {
      store.dispatch('studio/fetchJobs');
    });

    const createNewJob = () => {
      router.push({ name: 'CreateJob' });
    };

    const deleteJob = async (id: string) => {
      if (confirm('Are you sure you want to delete this job?')) {
        try {
          await store.dispatch('studio/deleteJob', id);
          // Refresh the jobs list after deletion
          await store.dispatch('studio/fetchJobs');
        } catch (error) {
          console.error('Error deleting job:', error);
          // Handle error (e.g., show an error message to the user)
        }
      }
    };

    const startOrRestartJob = async (job: any) => {
      try {
        if (job.status === 'completed') {
          await store.dispatch('studio/restartJob', job.id);
        } else {
          await store.dispatch('studio/startJob', job.id);
        }
        // Refresh the jobs list after starting/restarting the job
        await store.dispatch('studio/fetchJobs');
      } catch (error) {
        console.error('Error starting/restarting job:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const canStartOrRestartJob = (job: any) => {
      return job.status !== 'running' || job.status === 'completed';
    };

    const getStatusClass = (status: string) => {
      const baseClasses = 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full';
      switch (status.toLowerCase()) {
        case 'completed':
          return `${baseClasses} bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100`;
        case 'running':
          return `${baseClasses} bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-100`;
        case 'failed':
          return `${baseClasses} bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-100`;
        default:
          return `${baseClasses} bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300`;
      }
    };

    return {
      searchQuery,
      filteredJobs,
      createNewJob,
      deleteJob,
      startOrRestartJob,
      canStartOrRestartJob,
      getStatusClass,
      formatDate,
    };
  },
});
</script>

<style scoped>
.jobs {
  @apply p-6 dark:bg-gray-900;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}
</style>