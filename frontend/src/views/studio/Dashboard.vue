<template>
  <div class="dashboard">
    <h1 class="text-3xl font-bold mb-8 text-gray-800 dark:text-gray-200">Dashboard</h1>

    <div v-if="isLoading" class="flex justify-center items-center h-64">
      <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-indigo-500"></div>
    </div>

    <div v-else-if="error"
      class="bg-red-100 dark:bg-red-900 border-l-4 border-red-500 text-red-700 dark:text-red-200 p-4 mb-8" role="alert">
      <p class="font-bold">Error</p>
      <p>{{ error }}</p>
    </div>

    <template v-else>
      <!-- Summary Section -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div v-for="(metric, index) in summaryMetrics" :key="index"
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 transition-all duration-300 transform hover:scale-105"
          :style="{ animationDelay: `${index * 100}ms` }">
          <h3 class="text-lg font-semibold mb-2 text-gray-700 dark:text-gray-300">{{ metric.label }}</h3>
          <p class="text-3xl font-bold text-indigo-600 dark:text-indigo-400">{{ metric.value }}</p>
        </div>
      </div>

      <!-- Charts Section -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 transition-all duration-300 transform hover:scale-105">
          <h2 class="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200">Job Status Distribution</h2>
          <PieChart v-if="jobStatusData" :chart-data="jobStatusData" />
        </div>
        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 transition-all duration-300 transform hover:scale-105">
          <h2 class="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200">Jobs Created Over Time</h2>
          <LineChart v-if="jobsOverTimeData" :chart-data="jobsOverTimeData" />
        </div>
        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 transition-all duration-300 transform hover:scale-105">
          <h2 class="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200">Resource Usage</h2>
          <BarChart v-if="resourceUsageData" :chart-data="resourceUsageData" />
        </div>
        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 transition-all duration-300 transform hover:scale-105">
          <h2 class="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200">Pipeline Execution Times</h2>
          <BarChart v-if="pipelineExecutionData" :chart-data="pipelineExecutionData" />
        </div>
      </div>
    </template>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref, computed, watch } from 'vue';
import { useStore } from 'vuex';
import PieChart from '@/components/charts/PieChart.vue';
import LineChart from '@/components/charts/LineChart.vue';
import BarChart from '@/components/charts/BarChart.vue';
import { ChartData } from 'chart.js';
import { Job, Pipeline } from '@/store/modules/studio';

interface SummaryMetric {
  label: string;
  value: string | number;
}

export default defineComponent({
  name: 'Dashboard',
  components: {
    PieChart,
    LineChart,
    BarChart,
  },
  setup() {
    const store = useStore();
    const jobStatusData = ref<ChartData<'pie'> | null>(null);
    const jobsOverTimeData = ref<ChartData<'line'> | null>(null);
    const resourceUsageData = ref<ChartData<'bar'> | null>(null);
    const pipelineExecutionData = ref<ChartData<'bar'> | null>(null);
    const summaryMetrics = ref<SummaryMetric[]>([]);
    const isLoading = ref(true);
    const error = ref<string | null>(null);

    const jobs = computed<Job[]>(() => store.getters['studio/getJobs']);
    const pipelines = computed<Pipeline[]>(() => store.getters['studio/getPipelines']);
    const isDarkMode = computed(() => store.getters['theme/isDarkMode']);

    onMounted(async () => {
      try {
        // Fetch data from the store
        await store.dispatch('studio/fetchAllData');

        // Set up summary metrics
        summaryMetrics.value = [
          { label: 'Total Jobs', value: jobs.value.length },
          { label: 'Active Pipelines', value: pipelines.value.length },
          { label: 'Success Rate', value: calculateSuccessRate(jobs.value) },
          { label: 'Avg. Execution Time', value: calculateAvgExecutionTime(jobs.value) },
        ];

        // Set up chart data
        setupChartData();
        isLoading.value = false;
      } catch (err) {
        console.error('Error fetching dashboard data:', err);
        error.value = 'Failed to load dashboard data. Please try again later.';
        isLoading.value = false;
      }
    });

    const calculateSuccessRate = (jobs: Job[]) => {
      const successfulJobs = jobs.filter(job => job.status === 'completed').length;
      return `${((successfulJobs / jobs.length) * 100).toFixed(1)}%`;
    };

    const calculateAvgExecutionTime = (jobs: Job[]) => {
      const completedJobs = jobs.filter(job => job.status === 'completed');
      if (completedJobs.length === 0) return 'N/A';

      const totalExecutionTime = completedJobs.reduce((sum, job) => {
        const start = new Date(job.createdAt).getTime();
        const end = new Date(job.updatedAt).getTime();
        return sum + (end - start);
      }, 0);

      const avgExecutionTimeMs = totalExecutionTime / completedJobs.length;
      const avgExecutionTimeSec = Math.round(avgExecutionTimeMs / 1000);
      const minutes = Math.floor(avgExecutionTimeSec / 60);
      const seconds = avgExecutionTimeSec % 60;

      return `${minutes}m ${seconds}s`;
    };

    const setupChartData = () => {
      // Job Status Distribution
      const statusCounts = {
        completed: jobs.value.filter(job => job.status === 'completed').length,
        inProgress: jobs.value.filter(job => job.status === 'in_progress').length,
        failed: jobs.value.filter(job => job.status === 'failed').length,
      };

      jobStatusData.value = {
        labels: ['Completed', 'In Progress', 'Failed'],
        datasets: [{
          data: [statusCounts.completed, statusCounts.inProgress, statusCounts.failed],
          backgroundColor: ['#4CAF50', '#2196F3', '#F44336'],
        }],
      };

      updateChartColors();
    };

    const updateChartColors = () => {
      // Jobs Created Over Time
      jobsOverTimeData.value = {
        labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
        datasets: [{
          label: 'Jobs Created',
          data: [12, 19, 3, 5, 2, 3],
          borderColor: isDarkMode.value ? '#A5B4FC' : '#3730A3',
          backgroundColor: isDarkMode.value ? 'rgba(165, 180, 252, 0.2)' : 'rgba(55, 48, 163, 0.2)',
          tension: 0.1,
        }],
      };

      // Resource Usage (placeholder data)
      resourceUsageData.value = {
        labels: ['CPU', 'Memory', 'Disk'],
        datasets: [{
          label: 'Usage (%)',
          data: [65, 80, 45],
          backgroundColor: ['#FFA726', '#66BB6A', '#29B6F6'],
        }],
      };

      // Pipeline Execution Times (placeholder data)
      pipelineExecutionData.value = {
        labels: pipelines.value.slice(0, 4).map(pipeline => pipeline.name),
        datasets: [{
          label: 'Execution Time (minutes)',
          data: [10, 15, 8, 12],
          backgroundColor: isDarkMode.value ? '#B39DDB' : '#7E57C2',
        }],
      };
    };

    // Watch for changes in dark mode and update chart colors
    watch(isDarkMode, () => {
      updateChartColors();
    });

    return {
      jobStatusData,
      jobsOverTimeData,
      resourceUsageData,
      pipelineExecutionData,
      summaryMetrics,
      isLoading,
      error,
    };
  },
});
</script>

<style scoped>
.dashboard {
  @apply p-6;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dashboard>div {
  animation: fadeInUp 0.5s ease-out forwards;
}
</style>