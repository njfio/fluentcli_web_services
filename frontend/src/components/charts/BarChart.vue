<template>
  <div class="chart-container">
    <canvas ref="chartCanvas"></canvas>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, ref, PropType } from 'vue';
import Chart from 'chart.js/auto';
import { ChartData } from 'chart.js';

export default defineComponent({
  name: 'BarChart',
  props: {
    chartData: {
      type: Object as PropType<ChartData<'bar'>>,
      required: true,
    },
  },
  setup(props) {
    const chartCanvas = ref<HTMLCanvasElement | null>(null);
    let chart: Chart | null = null;

    const createChart = () => {
      if (chartCanvas.value) {
        chart = new Chart(chartCanvas.value, {
          type: 'bar',
          data: props.chartData,
          options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
              legend: {
                display: false,
              },
            },
          },
        });
      }
    };

    const updateChart = () => {
      if (chart) {
        chart.data = props.chartData;
        chart.update();
      }
    };

    onMounted(() => {
      createChart();
    });

    watch(() => props.chartData, () => {
      updateChart();
    }, { deep: true });

    return {
      chartCanvas,
    };
  },
});
</script>

<style scoped>
.chart-container {
  position: relative;
  height: 300px;
  width: 100%;
}
</style>