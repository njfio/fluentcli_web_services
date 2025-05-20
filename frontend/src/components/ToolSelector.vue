<template>
  <div class="tool-selector">
    <div v-if="loading" class="loading-indicator">
      <i class="fas fa-spinner fa-spin"></i>
      Loading tools...
    </div>
    
    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>
    
    <div v-else-if="tools.length === 0" class="empty-state">
      No tools available
    </div>
    
    <div v-else class="tool-list">
      <div v-for="tool in tools" :key="tool.id" class="tool-item">
        <input
          type="checkbox"
          :id="`tool-${tool.id}`"
          :value="tool.id"
          v-model="selectedToolIds"
          @change="updateModelValue"
        />
        <label :for="`tool-${tool.id}`" class="tool-label">
          <div class="tool-icon">
            <i :class="getToolIcon(tool)"></i>
          </div>
          <div class="tool-info">
            <div class="tool-name">{{ tool.name }}</div>
            <div class="tool-description">{{ tool.description }}</div>
          </div>
        </label>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, watch, onMounted, PropType } from 'vue';
import { useStore } from 'vuex';
import { Tool } from '../types/tool';

export default defineComponent({
  name: 'ToolSelector',
  props: {
    modelValue: {
      type: Array as PropType<string[]>,
      default: () => []
    }
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    const store = useStore();
    
    const selectedToolIds = ref<string[]>([]);
    
    const tools = computed(() => store.state.tool.tools);
    const loading = computed(() => store.state.tool.loading);
    const error = computed(() => store.state.tool.error);
    
    // Initialize selected tools from props
    onMounted(() => {
      selectedToolIds.value = [...props.modelValue];
      
      // Fetch tools if not already loaded
      if (tools.value.length === 0) {
        store.dispatch('tool/fetchTools');
      }
    });
    
    // Update selected tools when props change
    watch(() => props.modelValue, (newValue) => {
      selectedToolIds.value = [...newValue];
    });
    
    const updateModelValue = () => {
      emit('update:modelValue', selectedToolIds.value);
    };
    
    const getToolIcon = (tool: Tool) => {
      // Map tool names to Font Awesome icons
      const iconMap: Record<string, string> = {
        'get_weather': 'fas fa-cloud',
        'search_web': 'fas fa-search',
        'calculate': 'fas fa-calculator',
        'get_stock_price': 'fas fa-chart-line',
        'generate_image': 'fas fa-image'
      };
      
      return iconMap[tool.name] || 'fas fa-tools';
    };
    
    return {
      tools,
      loading,
      error,
      selectedToolIds,
      updateModelValue,
      getToolIcon
    };
  }
});
</script>

<style scoped>
.tool-selector {
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  max-height: 300px;
  overflow-y: auto;
}

.loading-indicator, .empty-state, .error-message {
  padding: 16px;
  text-align: center;
  color: #718096;
}

.loading-indicator i {
  margin-right: 8px;
}

.error-message {
  color: #e53e3e;
}

.tool-list {
  padding: 8px;
}

.tool-item {
  margin-bottom: 8px;
}

.tool-item:last-child {
  margin-bottom: 0;
}

.tool-item input[type="checkbox"] {
  display: none;
}

.tool-label {
  display: flex;
  align-items: center;
  padding: 12px;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.tool-label:hover {
  background-color: #f7fafc;
}

.tool-item input[type="checkbox"]:checked + .tool-label {
  background-color: #ebf8ff;
  border-color: #4299e1;
}

.tool-icon {
  margin-right: 12px;
  font-size: 1.25rem;
  color: #4a5568;
}

.tool-info {
  flex: 1;
}

.tool-name {
  font-weight: 500;
  color: #2d3748;
  margin-bottom: 4px;
}

.tool-description {
  font-size: 0.875rem;
  color: #718096;
}
</style>
