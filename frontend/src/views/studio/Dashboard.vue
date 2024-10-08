<template>
  <div class="dashboard">
    <h1 class="text-2xl font-bold mb-6">Dashboard</h1>
    
    <!-- Summary Section -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
      <div v-for="(metric, index) in summaryMetrics" :key="index" class="bg-white p-4 rounded-lg shadow">
        <h3 class="text-lg font-semibold mb-2">{{ metric.label }}</h3>
        <p class="text-3xl font-bold">{{ metric.value }}</p>
      </div>
    </div>

    <!-- Charts Section -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Job Status Distribution</h2>
        <PieChart v-if="jobStatusData" :chart-data="jobStatusData" />
      </div>
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Jobs Created Over Time</h2>
        <LineChart v-if="jobsOverTimeData" :chart-data="jobsOverTimeData" />
      </div>
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Resource Usage</h2>
        <BarChart v-if="resourceUsageData" :chart-data="resourceUsageData" />
      </div>
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Pipeline Execution Times</h2>
        <BarChart v-if="pipelineExecutionData" :chart-data="pipelineExecutionData" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref, computed } from 'vue';
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

    const jobs = computed<Job[]>(() => store.getters['studio/getJobs']);
    const pipelines = computed<Pipeline[]>(() => store.getters['studio/getPipelines']);

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
      } catch (error) {
        console.error('Error fetching dashboard data:', error);
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

      // Jobs Created Over Time (placeholder data)
      jobsOverTimeData.value = {
        labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
        datasets: [{
          label: 'Jobs Created',
          data: [12, 19, 3, 5, 2, 3],
          borderColor: '#2196F3',
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
          backgroundColor: '#7E57C2',
        }],
      };
    };

    return {
      jobStatusData,
      jobsOverTimeData,
      resourceUsageData,
      pipelineExecutionData,
      summaryMetrics,
    };
  },
});
</script>

<style scoped>
.dashboard {
  padding: 20px;
}
</style>