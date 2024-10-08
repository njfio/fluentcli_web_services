<template>
  <div class="jobs">
    <h1 class="text-2xl font-bold mb-4">Jobs</h1>
    <div class="mb-4 flex justify-between items-center">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search jobs..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
      />
      <button
        @click="createNewJob"
        class="bg-primary-500 text-white px-4 py-2 rounded-lg hover:bg-primary-600 transition-colors duration-200"
      >
        Create New Job
      </button>
    </div>
    <div class="overflow-x-auto shadow-md rounded-lg">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-primary-600">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">ID</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">Docker File</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Configuration</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Pipeline</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">Status</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">Created At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">Updated At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/12">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="job in filteredJobs" :key="job.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              <router-link :to="{ name: 'JobDetail', params: { id: job.id } }" class="text-primary-600 hover:text-primary-900">
                <span :title="job.id" class="truncate block max-w-xs">{{ job.id }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <router-link :to="{ name: 'DockerFileEditor', params: { id: job.worker_type } }" class="text-primary-600 hover:text-primary-900">
                <span :title="job.worker_type" class="truncate block max-w-xs">{{ job.worker_type }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <router-link :to="{ name: 'ConfigurationEditor', params: { id: job.config } }" class="text-primary-600 hover:text-primary-900">
                <span :title="job.config" class="truncate block max-w-xs">{{ job.config }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <router-link :to="{ name: 'PipelineEditor', params: { id: job.pipeline_id } }" class="text-primary-600 hover:text-primary-900">
                <span :title="job.pipeline_id" class="truncate block max-w-xs">{{ job.pipeline_id }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <span :class="getStatusClass(job.status)">{{ job.status }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ formatDate(job.createdAt) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ formatDate(job.updatedAt) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <router-link :to="{ name: 'JobDetail', params: { id: job.id } }" class="text-primary-600 hover:text-primary-900 mr-2">View</router-link>
              <button @click="deleteJob(job.id)" class="text-red-600 hover:text-red-900">Delete</button>
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

    const getStatusClass = (status: string) => {
      switch (status.toLowerCase()) {
        case 'completed':
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800';
        case 'running':
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800';
        case 'failed':
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800';
        default:
          return 'px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800';
      }
    };

    return {
      searchQuery,
      filteredJobs,
      createNewJob,
      deleteJob,
      getStatusClass,
      formatDate,
    };
  },
});
</script>

<style scoped>
.jobs {
  @apply p-6;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}
</style>