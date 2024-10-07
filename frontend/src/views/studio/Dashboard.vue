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
        <PieChart :chart-data="jobStatusData" />
      </div>
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Jobs Created Over Time</h2>
        <LineChart :chart-data="jobsOverTimeData" />
      </div>
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Resource Usage</h2>
        <BarChart :chart-data="resourceUsageData" />
      </div>
      <div class="bg-white p-4 rounded-lg shadow">
        <h2 class="text-lg font-semibold mb-4">Pipeline Execution Times</h2>
        <BarChart :chart-data="pipelineExecutionData" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import PieChart from '@/components/charts/PieChart.vue';
import LineChart from '@/components/charts/LineChart.vue';
import BarChart from '@/components/charts/BarChart.vue';

interface ChartData {
  labels: string[];
  datasets: {
    label?: string;
    data: number[];
    backgroundColor?: string | string[];
    borderColor?: string;
    tension?: number;
  }[];
}

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
    const jobStatusData = ref<ChartData | null>(null);
    const jobsOverTimeData = ref<ChartData | null>(null);
    const resourceUsageData = ref<ChartData | null>(null);
    const pipelineExecutionData = ref<ChartData | null>(null);
    const summaryMetrics = ref<SummaryMetric[]>([]);

    onMounted(async () => {
      // Fetch data from the store or API
      await store.dispatch('studio/fetchDashboardData');
      
      // Set up summary metrics
      summaryMetrics.value = [
        { label: 'Total Jobs', value: 150 },
        { label: 'Active Pipelines', value: 12 },
        { label: 'Success Rate', value: '95%' },
        { label: 'Avg. Execution Time', value: '5m 30s' },
      ];

      // Set up chart data
      jobStatusData.value = {
        labels: ['Completed', 'In Progress', 'Failed'],
        datasets: [{
          data: [30, 50, 20],
          backgroundColor: ['#4CAF50', '#2196F3', '#F44336'],
        }],
      };

      jobsOverTimeData.value = {
        labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
        datasets: [{
          label: 'Jobs Created',
          data: [12, 19, 3, 5, 2, 3],
          borderColor: '#2196F3',
          tension: 0.1,
        }],
      };

      resourceUsageData.value = {
        labels: ['CPU', 'Memory', 'Disk'],
        datasets: [{
          label: 'Usage (%)',
          data: [65, 80, 45],
          backgroundColor: ['#FFA726', '#66BB6A', '#29B6F6'],
        }],
      };

      pipelineExecutionData.value = {
        labels: ['Pipeline 1', 'Pipeline 2', 'Pipeline 3', 'Pipeline 4'],
        datasets: [{
          label: 'Execution Time (minutes)',
          data: [10, 15, 8, 12],
          backgroundColor: '#7E57C2',
        }],
      };
    });

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