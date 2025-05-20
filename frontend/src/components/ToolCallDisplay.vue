<template>
  <div class="tool-call" :class="{ 'has-error': hasError }">
    <div class="tool-call-header">
      <div class="tool-icon">
        <i :class="toolIconClass"></i>
      </div>
      <div class="tool-info">
        <div class="tool-name">{{ toolCall.name }}</div>
        <div class="tool-status" :class="statusClass">{{ statusText }}</div>
      </div>
      <div class="tool-actions">
        <button v-if="hasError" @click="$emit('retry')" class="retry-button">
          <i class="fas fa-redo"></i>
          Retry
        </button>
        <button @click="expanded = !expanded" class="expand-button">
          <i :class="expanded ? 'fas fa-chevron-up' : 'fas fa-chevron-down'"></i>
        </button>
      </div>
    </div>
    
    <div v-if="expanded" class="tool-call-details">
      <div class="tool-call-arguments">
        <h4>Arguments</h4>
        <pre><code>{{ formattedArguments }}</code></pre>
      </div>
      
      <div v-if="toolCall.result" class="tool-call-result">
        <h4>Result</h4>
        <tool-result-display :result="toolCall.result" />
      </div>
      
      <div v-if="hasError" class="tool-call-error">
        <h4>Error</h4>
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
        'generate_image': 'fas fa-image'
      };
      
      return iconMap[props.toolCall.name] || 'fas fa-tools';
    });
    
    const formattedArguments = computed(() => {
      return JSON.stringify(props.toolCall.arguments, null, 2);
    });
    
    return {
      expanded,
      hasError,
      statusClass,
      statusText,
      toolIconClass,
      formattedArguments
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
}

.tool-info {
  flex: 1;
}

.tool-name {
  font-weight: 600;
  color: #2d3748;
}

.tool-status {
  font-size: 0.875rem;
  color: #718096;
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

.tool-actions {
  display: flex;
  gap: 8px;
}

.retry-button, .expand-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 0.875rem;
}

.retry-button {
  color: #3182ce;
}

.retry-button:hover {
  background-color: #ebf8ff;
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

.tool-call-arguments h4, .tool-call-result h4, .tool-call-error h4 {
  margin-top: 0;
  margin-bottom: 8px;
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
}

.error-message {
  color: #e53e3e;
  background-color: #fff5f5;
  padding: 12px;
  border-radius: 4px;
  border-left: 3px solid #e53e3e;
}
</style>
