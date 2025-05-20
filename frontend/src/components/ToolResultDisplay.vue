<template>
  <div class="tool-result">
    <div v-if="isImage" class="image-result">
      <img :src="result.url" :alt="result.alt || 'Tool result'" />
      <div v-if="result.caption" class="image-caption">{{ result.caption }}</div>
    </div>

    <div v-else-if="isTable" class="table-result">
      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, index) in tableRows" :key="index">
              <td v-for="header in tableHeaders" :key="header">{{ formatTableCell(row[header]) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-if="tableRows.length > 10" class="table-info">
        Showing {{ tableRows.length }} rows
      </div>
    </div>

    <div v-else-if="isCode" class="code-result">
      <div class="code-header">
        <span class="code-language">{{ result.language || 'code' }}</span>
        <button @click="copyCode" class="copy-button" title="Copy code to clipboard">
          <i class="fas fa-copy"></i>
        </button>
      </div>
      <pre><code>{{ result.code }}</code></pre>
    </div>

    <div v-else-if="isHTML" class="html-result" v-html="result.html"></div>

    <div v-else-if="isLink" class="link-result">
      <a :href="result.url" target="_blank" rel="noopener noreferrer">
        <i class="fas fa-external-link-alt"></i>
        {{ result.title || result.url }}
      </a>
      <p v-if="result.description" class="link-description">{{ result.description }}</p>
    </div>

    <div v-else-if="isText" class="text-result">
      <p>{{ result.text }}</p>
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
             (('type' in props.result && props.result.type === 'image' && 'url' in props.result) ||
              ('url' in props.result && typeof props.result.url === 'string' &&
               props.result.url.match(/\.(jpeg|jpg|gif|png|webp|svg)$/i)));
    });

    const isTable = computed(() => {
      return Array.isArray(props.result) &&
             props.result.length > 0 &&
             typeof props.result[0] === 'object';
    });

    const isCode = computed(() => {
      return typeof props.result === 'object' &&
             props.result !== null &&
             (('type' in props.result && props.result.type === 'code' && 'code' in props.result) ||
              ('code' in props.result && typeof props.result.code === 'string'));
    });

    const isHTML = computed(() => {
      return typeof props.result === 'object' &&
             props.result !== null &&
             (('type' in props.result && props.result.type === 'html' && 'html' in props.result) ||
              ('html' in props.result && typeof props.result.html === 'string'));
    });

    const isLink = computed(() => {
      return typeof props.result === 'object' &&
             props.result !== null &&
             (('type' in props.result && props.result.type === 'link' && 'url' in props.result) ||
              ('url' in props.result && !isImage.value));
    });

    const isText = computed(() => {
      return typeof props.result === 'object' &&
             props.result !== null &&
             (('type' in props.result && props.result.type === 'text' && 'text' in props.result) ||
              ('text' in props.result && typeof props.result.text === 'string'));
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

    const formatTableCell = (value: any) => {
      if (value === null || value === undefined) return '';
      if (typeof value === 'object') return JSON.stringify(value);
      return String(value);
    };

    const copyCode = () => {
      if (isCode.value && props.result.code) {
        navigator.clipboard.writeText(props.result.code)
          .catch(err => console.error('Failed to copy code: ', err));
      }
    };

    return {
      isImage,
      isTable,
      isCode,
      isHTML,
      isLink,
      isText,
      tableHeaders,
      tableRows,
      formattedResult,
      formatTableCell,
      copyCode
    };
  }
});
</script>

<style scoped>
.tool-result {
  width: 100%;
}

/* Image result styles */
.image-result {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.image-result img {
  max-width: 100%;
  border-radius: 4px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.image-caption {
  margin-top: 8px;
  font-size: 0.875rem;
  color: #4a5568;
  text-align: center;
  font-style: italic;
}

/* Table result styles */
.table-result {
  margin-bottom: 8px;
}

.table-container {
  overflow-x: auto;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  margin-bottom: 4px;
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

.table-info {
  font-size: 0.75rem;
  color: #718096;
  text-align: right;
}

/* Code result styles */
.code-result {
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  overflow: hidden;
}

.code-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: #f7fafc;
  border-bottom: 1px solid #e2e8f0;
}

.code-language {
  font-size: 0.75rem;
  font-weight: 600;
  color: #4a5568;
  text-transform: uppercase;
}

.copy-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  color: #4a5568;
  transition: background-color 0.2s ease-in-out;
}

.copy-button:hover {
  background-color: #edf2f7;
}

.code-result pre, .json-result pre {
  background-color: #f8f9fa;
  padding: 12px;
  border-radius: 0 0 4px 4px;
  overflow-x: auto;
  font-size: 0.875rem;
  margin: 0;
}

/* HTML result styles */
.html-result {
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  padding: 12px;
  background-color: #ffffff;
}

/* Link result styles */
.link-result {
  padding: 12px;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  background-color: #f8fafc;
}

.link-result a {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #3182ce;
  text-decoration: none;
  font-weight: 500;
}

.link-result a:hover {
  text-decoration: underline;
}

.link-description {
  margin-top: 8px;
  font-size: 0.875rem;
  color: #4a5568;
}

/* Text result styles */
.text-result {
  padding: 12px;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  background-color: #ffffff;
}

.text-result p {
  margin: 0;
  font-size: 0.875rem;
  line-height: 1.5;
}

/* Dark mode styles */
.dark .table-result th {
  background-color: #2d3748;
}

.dark .table-result tr:nth-child(even) {
  background-color: #2d3748;
}

.dark .table-result th, .dark .table-result td {
  border-color: #4a5568;
}

.dark .code-header {
  background-color: #2d3748;
  border-color: #4a5568;
}

.dark .code-result, .dark .json-result pre, .dark .html-result, .dark .link-result, .dark .text-result {
  border-color: #4a5568;
  background-color: #1a202c;
  color: #e2e8f0;
}

.dark .copy-button:hover {
  background-color: #4a5568;
}

.dark .link-result a {
  color: #63b3ed;
}

.dark .image-caption, .dark .table-info, .dark .link-description {
  color: #a0aec0;
}
</style>
