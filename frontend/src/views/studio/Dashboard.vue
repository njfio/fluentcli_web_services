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
          <h2 class="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200">Jobs Completed Over Time</h2>
          <LineChart v-if="jobsCompletedOverTimeData" :chart-data="jobsCompletedOverTimeData" />
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
    const jobsCompletedOverTimeData = ref<ChartData<'line'> | null>(null);
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
          { label: 'Active Pipelines', value: getActivePipelines(pipelines.value) },
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

    const getActivePipelines = (pipelines: Pipeline[]) => {
      return pipelines.filter(pipeline => pipeline.status === 'active').length;
    };

    const calculateSuccessRate = (jobs: Job[]) => {
      if (jobs.length === 0) return 'N/A';
      const successfulJobs = jobs.filter(job => job.status === 'completed').length;
      return `${((successfulJobs / jobs.length) * 100).toFixed(1)}%`;
    };

    const calculateAvgExecutionTime = (jobs: Job[]) => {
      const completedJobs = jobs.filter(job =>
        job.status === 'completed' && job.started_at && job.completed_at
      );
      if (completedJobs.length === 0) return 'N/A';

      const totalExecutionTime = completedJobs.reduce((sum, job) => {
        const start = new Date(job.started_at!).getTime();
        const end = new Date(job.completed_at!).getTime();
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
        running: jobs.value.filter(job => job.status === 'running').length,
        pending: jobs.value.filter(job => job.status === 'pending').length,
        archived: jobs.value.filter(job => job.status === 'archived').length,
      };

      jobStatusData.value = {
        labels: ['Completed', 'Running', 'Pending', 'Archived'],
        datasets: [{
          data: [statusCounts.completed, statusCounts.running, statusCounts.pending, statusCounts.archived],
          backgroundColor: ['#4CAF50', '#2196F3', '#FFC107', '#9E9E9E'],
        }],
      };

      // Jobs Completed Over Time (last 45 days)
      const today = new Date();
      const last45Days = Array.from({ length: 45 }, (_, i) => {
        const date = new Date(today);
        date.setDate(today.getDate() - i);
        return date.toISOString().split('T')[0];
      }).reverse();

      const completedJobsByDate = jobs.value
        .filter(job => job.status === 'completed' && job.completed_at)
        .reduce((acc, job) => {
          const date = new Date(job.completed_at!).toISOString().split('T')[0];
          acc[date] = (acc[date] || 0) + 1;
          return acc;
        }, {} as Record<string, number>);

      const completedJobCounts = last45Days.map(date => completedJobsByDate[date] || 0);

      jobsCompletedOverTimeData.value = {
        labels: last45Days,
        datasets: [{
          label: 'Jobs Completed',
          data: completedJobCounts,
          borderColor: isDarkMode.value ? '#A5B4FC' : '#3730A3',
          backgroundColor: isDarkMode.value ? 'rgba(165, 180, 252, 0.2)' : 'rgba(55, 48, 163, 0.2)',
          tension: 0.1,
        }],
      };

      updateChartColors();
    };

    const updateChartColors = () => {
      const resourceColors = [
        'rgba(255, 99, 132, 0.8)',   // Red
        'rgba(54, 162, 235, 0.8)',   // Blue
        'rgba(255, 206, 86, 0.8)',   // Yellow
        'rgba(75, 192, 192, 0.8)',   // Green
      ];

      // Resource Usage (placeholder data)
      resourceUsageData.value = {
        labels: ['CPU', 'Memory', 'Disk', 'Network'],
        datasets: [{
          label: 'Usage (%)',
          data: [65, 80, 45, 70],
          backgroundColor: resourceColors,
          borderColor: resourceColors.map(color => color.replace('0.8', '1')),
          borderWidth: 1
        }],
      };

      const pipelineColors = [
        'rgba(255, 99, 132, 0.8)',   // Red
        'rgba(54, 162, 235, 0.8)',   // Blue
        'rgba(255, 206, 86, 0.8)',   // Yellow
        'rgba(75, 192, 192, 0.8)',   // Green
        'rgba(153, 102, 255, 0.8)',  // Purple
        'rgba(255, 159, 64, 0.8)',   // Orange
      ];

      // Pipeline Execution Times (placeholder data)
      pipelineExecutionData.value = {
        labels: pipelines.value.slice(0, 6).map(pipeline => pipeline.name),
        datasets: [{
          label: 'Execution Time (minutes)',
          data: [10, 15, 8, 12, 7, 14],
          backgroundColor: pipelineColors,
          borderColor: pipelineColors.map(color => color.replace('0.8', '1')),
          borderWidth: 1
        }],
      };
    };

    // Watch for changes in dark mode and update chart colors
    watch(isDarkMode, () => {
      updateChartColors();
    });

    return {
      jobStatusData,
      jobsCompletedOverTimeData,
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