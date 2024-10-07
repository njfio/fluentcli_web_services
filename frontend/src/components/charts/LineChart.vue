<template>
  <div>
    <Line :data="chartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Line } from 'vue-chartjs';
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  LineElement,
  LinearScale,
  PointElement,
  CategoryScale,
  ChartData
} from 'chart.js';

ChartJS.register(
  Title,
  Tooltip,
  Legend,
  LineElement,
  LinearScale,
  PointElement,
  CategoryScale
);

export default defineComponent({
  name: 'LineChart',
  components: { Line },
  props: {
    chartData: {
      type: Object as PropType<ChartData<'line', number[], unknown>>,
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