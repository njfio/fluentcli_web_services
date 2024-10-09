<template>
  <div class="chart-container">
    <Pie :data="themedChartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, computed } from 'vue';
import { mapGetters } from 'vuex';
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
  computed: {
    ...mapGetters('theme', ['isDarkMode']),
    chartOptions(): ChartOptions<'pie'> {
      return {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            labels: {
              color: this.isDarkMode ? '#ffffff' : '#000000'
            }
          }
        }
      };
    },
    themedChartData(): ChartData<'pie'> {
      const data = { ...this.chartData };
      if (data.datasets) {
        data.datasets = data.datasets.map(dataset => ({
          ...dataset,
          borderColor: this.isDarkMode ? '#ffffff' : '#000000'
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