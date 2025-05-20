<template>
  <div class="agent-config-panel">
    <div class="panel-header">
      <h3>{{ agent ? 'Edit Agent' : 'Create Agent' }}</h3>
      <button @click="$emit('cancel')" class="close-button">
        <i class="fas fa-times"></i>
      </button>
    </div>

    <div class="panel-body">
      <div class="form-group">
        <label for="agent-name">Name</label>
        <input id="agent-name" v-model="agentName" type="text" />
        <div v-if="nameError" class="error-message">{{ nameError }}</div>
      </div>

      <div class="form-group">
        <label for="agent-description">Description</label>
        <textarea id="agent-description" v-model="agentDescription"></textarea>
        <div v-if="descriptionError" class="error-message">{{ descriptionError }}</div>
      </div>

      <div class="form-group">
        <label for="agent-icon">Icon</label>
        <select id="agent-icon" v-model="agentIcon">
          <option value="robot">Robot</option>
          <option value="brain">Brain</option>
          <option value="search">Search</option>
          <option value="code">Code</option>
          <option value="database">Database</option>
        </select>
      </div>

      <div class="form-group">
        <label for="agent-system-prompt">System Prompt</label>
        <textarea id="agent-system-prompt" v-model="systemPrompt" rows="4"></textarea>
        <div class="help-text">Instructions for the agent's behavior</div>
      </div>

      <div class="form-group">
        <label>Tools</label>
        <tool-selector v-model="selectedTools" />
        <div v-if="toolsError" class="error-message">{{ toolsError }}</div>
      </div>
    </div>

    <div class="panel-footer">
      <button @click="saveAgent" :disabled="!isValid" class="save-button">
        Save
      </button>
      <button @click="$emit('cancel')" class="cancel-button">
        Cancel
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, PropType, onMounted } from 'vue';
import { Agent } from '../types/agent';
import ToolSelector from './ToolSelector.vue';

export default defineComponent({
  name: 'AgentConfigPanel',
  components: {
    ToolSelector
  },
  props: {
    agent: {
      type: Object as PropType<Agent>,
      default: null
    }
  },
  emits: ['save', 'cancel'],
  setup(props, { emit }) {
    const agentName = ref('');
    const agentDescription = ref('');
    const agentIcon = ref('robot');
    const systemPrompt = ref('');
    const selectedTools = ref<string[]>([]);

    // Validation errors
    const nameError = ref('');
    const descriptionError = ref('');
    const toolsError = ref('');

    // Initialize form with agent data if editing
    onMounted(() => {
      if (props.agent) {
        agentName.value = props.agent.name;
        agentDescription.value = props.agent.description;
        agentIcon.value = props.agent.icon || 'robot';
        systemPrompt.value = props.agent.system_prompt || '';
        selectedTools.value = [...props.agent.tools];
      }
    });

    // Validate form
    const validateForm = () => {
      let isValid = true;

      // Validate name
      if (!agentName.value.trim()) {
        nameError.value = 'Name is required';
        isValid = false;
      } else {
        nameError.value = '';
      }

      // Validate description
      if (!agentDescription.value.trim()) {
        descriptionError.value = 'Description is required';
        isValid = false;
      } else {
        descriptionError.value = '';
      }

      // Validate tools
      if (selectedTools.value.length === 0) {
        toolsError.value = 'At least one tool is required';
        isValid = false;
      } else {
        toolsError.value = '';
      }

      return isValid;
    };

    const isValid = computed(() => {
      return agentName.value.trim() !== '' &&
             agentDescription.value.trim() !== '' &&
             selectedTools.value.length > 0;
    });

    const saveAgent = () => {
      if (!validateForm()) return;

      const agentData = {
        name: agentName.value.trim(),
        description: agentDescription.value.trim(),
        icon: agentIcon.value,
        system_prompt: systemPrompt.value.trim(),
        tools: selectedTools.value
      };

      console.log('AgentConfigPanel: Saving agent with data:', agentData);
      emit('save', agentData);
    };

    return {
      agentName,
      agentDescription,
      agentIcon,
      systemPrompt,
      selectedTools,
      nameError,
      descriptionError,
      toolsError,
      isValid,
      saveAgent
    };
  }
});
</script>

<style scoped>
.agent-config-panel {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  width: 400px;
  background-color: white;
  box-shadow: -2px 0 10px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  z-index: 100;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #e2e8f0;
}

.panel-header h3 {
  margin: 0;
  font-size: 1.25rem;
  color: #2d3748;
}

.close-button {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.25rem;
  color: #718096;
}

.close-button:hover {
  color: #4a5568;
}

.panel-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #4a5568;
}

.form-group input, .form-group textarea, .form-group select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  font-size: 0.875rem;
}

.form-group textarea {
  resize: vertical;
  min-height: 80px;
}

.help-text {
  margin-top: 4px;
  font-size: 0.75rem;
  color: #718096;
}

.error-message {
  margin-top: 4px;
  font-size: 0.75rem;
  color: #e53e3e;
}

.panel-footer {
  padding: 16px;
  border-top: 1px solid #e2e8f0;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.save-button, .cancel-button {
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
}

.save-button {
  background-color: #4299e1;
  color: white;
  border: none;
}

.save-button:hover:not(:disabled) {
  background-color: #3182ce;
}

.save-button:disabled {
  background-color: #a0aec0;
  cursor: not-allowed;
}

.cancel-button {
  background-color: #edf2f7;
  color: #4a5568;
  border: 1px solid #e2e8f0;
}

.cancel-button:hover {
  background-color: #e2e8f0;
}
</style>
