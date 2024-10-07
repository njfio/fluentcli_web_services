<template>
  <div>
    <Pie :data="chartData" :options="chartOptions" />
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Pie } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, CategoryScale, ChartData } from 'chart.js';

ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);

export default defineComponent({
  name: 'PieChart',
  components: { Pie },
  props: {
    chartData: {
      type: Object as PropType<ChartData<'pie', number[], unknown>>,
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