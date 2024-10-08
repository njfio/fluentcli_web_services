<template>
  <div class="chart-container">
    <Pie :data="chartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Pie } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, ChartData, ChartOptions } from 'chart.js';

ChartJS.register(Title, Tooltip, Legend, ArcElement);

export default defineComponent({
  name: 'PieChart',
  components: { Pie },
  props: {
    chartData: {
      type: Object as PropType<ChartData<'pie'>>,
      required: true
    }
  },
  setup() {
    const chartOptions: ChartOptions<'pie'> = {
      responsive: true,
      maintainAspectRatio: false
    };

    return { chartOptions };
  }
});
</script>

<style scoped>
.chart-container {
  position: relative;
  height: 300px;
}
</style>