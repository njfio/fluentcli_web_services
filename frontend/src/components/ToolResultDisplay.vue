<template>
  <div class="tool-result">
    <div v-if="isImage" class="image-result">
      <img :src="result.url" :alt="result.alt || 'Tool result'" />
    </div>
    <div v-else-if="isTable" class="table-result">
      <table>
        <thead>
          <tr>
            <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, index) in tableRows" :key="index">
            <td v-for="header in tableHeaders" :key="header">{{ row[header] }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else-if="isCode" class="code-result">
      <pre><code>{{ result.code }}</code></pre>
    </div>
    <div v-else class="json-result">
      <pre><code>{{ formattedResult }}</code></pre>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, PropType } from 'vue';

export default defineComponent({
  name: 'ToolResultDisplay',
  props: {
    result: {
      type: [Object, Array, String, Number, Boolean] as PropType<any>,
      required: true
    }
  },
  setup(props) {
    const isImage = computed(() => {
      return typeof props.result === 'object' && 
             props.result !== null && 
             'type' in props.result && 
             props.result.type === 'image' && 
             'url' in props.result;
    });
    
    const isTable = computed(() => {
      return Array.isArray(props.result) && 
             props.result.length > 0 && 
             typeof props.result[0] === 'object';
    });
    
    const isCode = computed(() => {
      return typeof props.result === 'object' && 
             props.result !== null && 
             'type' in props.result && 
             props.result.type === 'code' && 
             'code' in props.result;
    });
    
    const tableHeaders = computed(() => {
      if (!isTable.value) return [];
      
      // Get all unique keys from all objects in the array
      const headers = new Set<string>();
      for (const row of props.result) {
        for (const key in row) {
          headers.add(key);
        }
      }
      
      return Array.from(headers);
    });
    
    const tableRows = computed(() => {
      if (!isTable.value) return [];
      return props.result;
    });
    
    const formattedResult = computed(() => {
      if (typeof props.result === 'object' && props.result !== null) {
        return JSON.stringify(props.result, null, 2);
      }
      return String(props.result);
    });
    
    return {
      isImage,
      isTable,
      isCode,
      tableHeaders,
      tableRows,
      formattedResult
    };
  }
});
</script>

<style scoped>
.tool-result {
  width: 100%;
}

.image-result img {
  max-width: 100%;
  border-radius: 4px;
}

.table-result {
  overflow-x: auto;
}

.table-result table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
}

.table-result th, .table-result td {
  padding: 8px 12px;
  text-align: left;
  border: 1px solid #e2e8f0;
}

.table-result th {
  background-color: #f7fafc;
  font-weight: 600;
}

.table-result tr:nth-child(even) {
  background-color: #f7fafc;
}

.code-result pre, .json-result pre {
  background-color: #f8f9fa;
  padding: 12px;
  border-radius: 4px;
  overflow-x: auto;
  font-size: 0.875rem;
  margin: 0;
}
</style>
