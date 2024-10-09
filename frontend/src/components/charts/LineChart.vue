<template>
  <div class="chart-container">
    <Line :data="themedChartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, computed } from 'vue';
import { mapGetters } from 'vuex';
import { Line } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale, ChartData, ChartOptions } from 'chart.js';

ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale);

export default defineComponent({
  name: 'LineChart',
  components: { Line },
  props: {
    chartData: {
      type: Object as PropType<ChartData<'line'>>,
      required: true
    }
  },
  computed: {
    ...mapGetters('theme', ['isDarkMode']),
    chartOptions(): ChartOptions<'line'> {
      return {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            labels: {
              color: this.isDarkMode ? '#ffffff' : '#000000'
            }
          }
        },
        scales: {
          x: {
            ticks: {
              color: this.isDarkMode ? '#ffffff' : '#000000'
            },
            grid: {
              color: this.isDarkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)'
            }
          },
          y: {
            ticks: {
              color: this.isDarkMode ? '#ffffff' : '#000000'
            },
            grid: {
              color: this.isDarkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)'
            }
          }
        }
      };
    },
    themedChartData(): ChartData<'line'> {
      const data = { ...this.chartData };
      if (data.datasets) {
        data.datasets = data.datasets.map(dataset => ({
          ...dataset,
          borderColor: this.isDarkMode ? '#ffffff' : '#000000',
          pointBackgroundColor: this.isDarkMode ? '#ffffff' : '#000000'
        }));
      }
      return data;
    }
  }
});
</script>

<style scoped>
.chart-container {
  position: relative;
  height: 300px;
}
</style>