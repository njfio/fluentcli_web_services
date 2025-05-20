<template>
  <div class="tool-call" :class="{ 'has-error': hasError }">
    <div class="tool-call-header">
      <div class="tool-icon">
        <i :class="toolIconClass"></i>
      </div>
      <div class="tool-info">
        <div class="tool-name">{{ toolCall.name }}</div>
        <div class="tool-status" :class="statusClass">
          <span v-if="toolCall.status === 'running'" class="loading-spinner">
            <i class="fas fa-spinner fa-spin"></i>
          </span>
          {{ statusText }}
        </div>
      </div>
      <div class="tool-actions">
        <button v-if="hasError" @click="$emit('retry')" class="retry-button" title="Retry tool execution">
          <i class="fas fa-redo"></i>
          Retry
        </button>
        <button v-if="toolCall.result" @click="copyResult" class="copy-button" title="Copy result to clipboard">
          <i class="fas fa-copy"></i>
        </button>
        <button @click="expanded = !expanded" class="expand-button" :title="expanded ? 'Collapse' : 'Expand'">
          <i :class="expanded ? 'fas fa-chevron-up' : 'fas fa-chevron-down'"></i>
        </button>
      </div>
    </div>

    <div v-if="expanded" class="tool-call-details">
      <div class="tool-call-arguments">
        <div class="section-header">
          <h4>Arguments</h4>
          <button @click="copyArguments" class="copy-button" title="Copy arguments to clipboard">
            <i class="fas fa-copy"></i>
          </button>
        </div>
        <pre><code>{{ formattedArguments }}</code></pre>
      </div>

      <div v-if="toolCall.result" class="tool-call-result">
        <div class="section-header">
          <h4>Result</h4>
          <button @click="copyResult" class="copy-button" title="Copy result to clipboard">
            <i class="fas fa-copy"></i>
          </button>
        </div>
        <tool-result-display :result="toolCall.result" />
      </div>

      <div v-if="hasError" class="tool-call-error">
        <div class="section-header">
          <h4>Error</h4>
          <button @click="copyError" class="copy-button" title="Copy error to clipboard">
            <i class="fas fa-copy"></i>
          </button>
        </div>
        <div class="error-message">{{ toolCall.error }}</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, ref, PropType } from 'vue';
import ToolResultDisplay from './ToolResultDisplay.vue';

export default defineComponent({
  name: 'ToolCallDisplay',
  components: {
    ToolResultDisplay
  },
  props: {
    toolCall: {
      type: Object as PropType<{
        id: string;
        name: string;
        arguments: any;
        status: string;
        result?: any;
        error?: string;
      }>,
      required: true
    }
  },
  emits: ['retry'],
  setup(props) {
    const expanded = ref(true);
    const copySuccess = ref(false);

    const hasError = computed(() => props.toolCall.status === 'error');

    const statusClass = computed(() => {
      return {
        'status-pending': props.toolCall.status === 'pending',
        'status-running': props.toolCall.status === 'running',
        'status-completed': props.toolCall.status === 'completed',
        'status-error': props.toolCall.status === 'error'
      };
    });

    const statusText = computed(() => {
      switch (props.toolCall.status) {
        case 'pending': return 'Pending';
        case 'running': return 'Running...';
        case 'completed': return 'Completed';
        case 'error': return 'Error';
        default: return 'Unknown';
      }
    });

    const toolIconClass = computed(() => {
      // Map tool names to Font Awesome icons
      const iconMap: Record<string, string> = {
        'get_weather': 'fas fa-cloud',
        'search_web': 'fas fa-search',
        'calculate': 'fas fa-calculator',
        'get_stock_price': 'fas fa-chart-line',
        'generate_image': 'fas fa-image',
        'code_interpreter': 'fas fa-code',
        'database_query': 'fas fa-database',
        'file_browser': 'fas fa-folder-open',
        'api_request': 'fas fa-globe'
      };

      return iconMap[props.toolCall.name] || 'fas fa-tools';
    });

    const formattedArguments = computed(() => {
      return JSON.stringify(props.toolCall.arguments, null, 2);
    });

    const formattedResult = computed(() => {
      if (typeof props.toolCall.result === 'object' && props.toolCall.result !== null) {
        return JSON.stringify(props.toolCall.result, null, 2);
      }
      return String(props.toolCall.result);
    });

    // Copy functions
    const copyToClipboard = async (text: string) => {
      try {
        await navigator.clipboard.writeText(text);
        copySuccess.value = true;
        setTimeout(() => {
          copySuccess.value = false;
        }, 2000);
      } catch (err) {
        console.error('Failed to copy text: ', err);
      }
    };

    const copyArguments = () => {
      copyToClipboard(formattedArguments.value);
    };

    const copyResult = () => {
      copyToClipboard(formattedResult.value);
    };

    const copyError = () => {
      if (props.toolCall.error) {
        copyToClipboard(props.toolCall.error);
      }
    };

    return {
      expanded,
      copySuccess,
      hasError,
      statusClass,
      statusText,
      toolIconClass,
      formattedArguments,
      formattedResult,
      copyArguments,
      copyResult,
      copyError
    };
  }
});
</script>

<style scoped>
.tool-call {
  background-color: #f5f7f9;
  border-radius: 8px;
  margin: 8px 0;
  overflow: hidden;
  border: 1px solid #e1e4e8;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  transition: box-shadow 0.2s ease-in-out;
}

.tool-call:hover {
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.tool-call.has-error {
  border-color: #f56565;
}

.tool-call-header {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background-color: #edf2f7;
  border-bottom: 1px solid #e1e4e8;
}

.tool-icon {
  margin-right: 12px;
  font-size: 18px;
  color: #4a5568;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.tool-info {
  flex: 1;
  min-width: 0;
}

.tool-name {
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 2px;
}

.tool-status {
  font-size: 0.875rem;
  color: #718096;
  display: flex;
  align-items: center;
  gap: 4px;
}

.tool-status.status-pending {
  color: #718096;
}

.tool-status.status-running {
  color: #3182ce;
}

.tool-status.status-completed {
  color: #38a169;
}

.tool-status.status-error {
  color: #e53e3e;
}

.loading-spinner {
  display: inline-block;
  margin-right: 4px;
}

.tool-actions {
  display: flex;
  gap: 8px;
}

.retry-button, .copy-button, .expand-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 0.875rem;
  transition: background-color 0.2s ease-in-out;
}

.retry-button {
  color: #3182ce;
}

.retry-button:hover {
  background-color: #ebf8ff;
}

.copy-button {
  color: #4a5568;
}

.copy-button:hover {
  background-color: #edf2f7;
}

.expand-button {
  color: #718096;
}

.expand-button:hover {
  background-color: #edf2f7;
}

.tool-call-details {
  padding: 16px;
}

.tool-call-arguments, .tool-call-result, .tool-call-error {
  margin-bottom: 16px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.section-header h4 {
  margin: 0;
  font-size: 0.875rem;
  color: #4a5568;
}

.tool-call-arguments pre, .tool-call-result pre {
  background-color: #f8f9fa;
  padding: 12px;
  border-radius: 4px;
  overflow-x: auto;
  font-size: 0.875rem;
  margin: 0;
  border: 1px solid #e2e8f0;
}

.error-message {
  color: #e53e3e;
  background-color: #fff5f5;
  padding: 12px;
  border-radius: 4px;
  border-left: 3px solid #e53e3e;
  font-size: 0.875rem;
}

/* Dark mode styles */
.dark .tool-call {
  background-color: #2d3748;
  border-color: #4a5568;
}

.dark .tool-call-header {
  background-color: #1a202c;
  border-color: #4a5568;
}

.dark .tool-name {
  color: #e2e8f0;
}

.dark .tool-icon {
  color: #a0aec0;
}

.dark .tool-call-arguments pre, .dark .tool-call-result pre {
  background-color: #1a202c;
  border-color: #4a5568;
  color: #e2e8f0;
}

.dark .error-message {
  background-color: rgba(229, 62, 62, 0.1);
  border-color: #e53e3e;
}

.dark .retry-button:hover, .dark .copy-button:hover, .dark .expand-button:hover {
  background-color: #2d3748;
}
</style>
