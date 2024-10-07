<template>
  <div>
    <Bar :data="chartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Bar } from 'vue-chartjs';
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale,
  ChartData
} from 'chart.js';

ChartJS.register(
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale
);

export default defineComponent({
  name: 'BarChart',
  components: { Bar },
  props: {
    chartData: {
      type: Object as PropType<ChartData<'bar', number[], unknown>>,
      required: true
    }
  },
  setup() {
    const chartOptions = {
      responsive: true,
      maintainAspectRatio: false
    };

    return { chartOptions };
  }
});
</script>