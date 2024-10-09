<template>
  <div class="chart-container">
    <Bar :data="themedChartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, computed } from 'vue';
import { mapGetters } from 'vuex';
import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ChartData, ChartOptions } from 'chart.js';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

export default defineComponent({
  name: 'BarChart',
  components: { Bar },
  props: {
    chartData: {
      type: Object as PropType<ChartData<'bar'>>,
      required: true
    }
  },
  computed: {
    ...mapGetters('theme', ['isDarkMode']),
    chartOptions(): ChartOptions<'bar'> {
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
    themedChartData(): ChartData<'bar'> {
      const data = { ...this.chartData };
      if (data.datasets) {
        data.datasets = data.datasets.map(dataset => ({
          ...dataset,
          borderColor: this.isDarkMode ? '#ffffff' : '#000000',
          backgroundColor: this.isDarkMode ? 'rgba(255, 255, 255, 0.2)' : 'rgba(0, 0, 0, 0.2)'
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