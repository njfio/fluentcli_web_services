<template>
  <div class="agent-sidebar">
    <div class="sidebar-header">
      <h2>Agents</h2>
      <button @click="showCreateAgentPanel = true" class="create-button">
        <i class="fas fa-plus"></i>
        New Agent
      </button>
    </div>

    <div v-if="loading" class="loading-indicator">
      <i class="fas fa-spinner fa-spin"></i>
      Loading agents...
    </div>

    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-else-if="agents.length === 0" class="empty-state">
      <i class="fas fa-robot"></i>
      <p>No agents found</p>
      <p>Create an agent to get started</p>
    </div>

    <div v-else class="agent-list">
      <div
        v-for="agent in agents"
        :key="agent.id"
        class="agent-item"
        :class="{ 'selected': agent.id === selectedAgentId }"
        @click="selectAgent(agent.id)"
      >
        <div class="agent-icon">
          <i :class="getAgentIcon(agent)"></i>
        </div>
        <div class="agent-info">
          <div class="agent-name">{{ agent.name }}</div>
          <div class="agent-description">{{ agent.description }}</div>
        </div>
        <div class="agent-actions">
          <button @click.stop="editAgent(agent)" class="edit-button">
            <i class="fas fa-edit"></i>
          </button>
          <button @click.stop="confirmDeleteAgent(agent)" class="delete-button">
            <i class="fas fa-trash"></i>
          </button>
        </div>
      </div>
    </div>

    <agent-config-panel
      v-if="showCreateAgentPanel"
      @save="createAgent"
      @cancel="showCreateAgentPanel = false"
    />

    <agent-config-panel
      v-if="showEditAgentPanel"
      :agent="editingAgent"
      @save="updateAgent"
      @cancel="showEditAgentPanel = false"
    />

    <div v-if="showDeleteConfirmation" class="confirmation-dialog">
      <div class="confirmation-dialog-content">
        <h3>Delete {{ deletingAgent?.name }}?</h3>
        <p>This action cannot be undone.</p>
        <div class="confirmation-actions">
          <button @click="deleteAgent" class="confirm-button">Delete</button>
          <button @click="showDeleteConfirmation = false" class="cancel-button">Cancel</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, ref } from 'vue';
import { useStore } from 'vuex';
import { Agent } from '../types/agent';
import AgentConfigPanel from './AgentConfigPanel.vue';

export default defineComponent({
  name: 'AgentSidebar',
  components: {
    AgentConfigPanel
  },
  setup() {
    const store = useStore();

    const showCreateAgentPanel = ref(false);
    const showEditAgentPanel = ref(false);
    const showDeleteConfirmation = ref(false);
    const editingAgent = ref<Agent | undefined>(undefined);
    const deletingAgent = ref<Agent | null>(null);

    const agents = computed(() => store.state.agent.agents);
    const selectedAgentId = computed(() => store.state.agent.selectedAgentId);
    const loading = computed(() => store.state.agent.loading);
    const error = computed(() => store.state.agent.error);

    // Load agents when component is mounted
    store.dispatch('agent/fetchAgents');

    // Initialize from localStorage
    store.dispatch('agent/initializeFromLocalStorage');

    const selectAgent = (id: string) => {
      store.dispatch('agent/selectAgent', id);
    };

    const editAgent = (agent: Agent) => {
      editingAgent.value = agent;
      showEditAgentPanel.value = true;
    };

    const confirmDeleteAgent = (agent: Agent) => {
      deletingAgent.value = agent;
      showDeleteConfirmation.value = true;
    };

    const createAgent = async (agentData: any) => {
      try {
        await store.dispatch('agent/createAgent', agentData);
        showCreateAgentPanel.value = false;
      } catch (error) {
        console.error('Error creating agent:', error);
      }
    };

    const updateAgent = async (agentData: any) => {
      if (!editingAgent.value) return;

      try {
        await store.dispatch('agent/updateAgent', {
          id: editingAgent.value.id,
          agentData
        });
        showEditAgentPanel.value = false;
        editingAgent.value = undefined;
      } catch (error) {
        console.error('Error updating agent:', error);
      }
    };

    const deleteAgent = async () => {
      if (!deletingAgent.value) return;

      try {
        await store.dispatch('agent/deleteAgent', deletingAgent.value.id);
        showDeleteConfirmation.value = false;
        deletingAgent.value = null;
      } catch (error) {
        console.error('Error deleting agent:', error);
      }
    };

    const getAgentIcon = (agent: Agent) => {
      const iconMap: Record<string, string> = {
        'robot': 'fas fa-robot',
        'brain': 'fas fa-brain',
        'search': 'fas fa-search',
        'code': 'fas fa-code',
        'database': 'fas fa-database'
      };

      return agent.icon && iconMap[agent.icon] ? iconMap[agent.icon] : 'fas fa-robot';
    };

    return {
      agents,
      selectedAgentId,
      loading,
      error,
      showCreateAgentPanel,
      showEditAgentPanel,
      showDeleteConfirmation,
      editingAgent,
      deletingAgent,
      selectAgent,
      editAgent,
      confirmDeleteAgent,
      createAgent,
      updateAgent,
      deleteAgent,
      getAgentIcon
    };
  }
});
</script>

<style scoped>
.agent-sidebar {
  width: 300px;
  height: 100%;
  background-color: #f8f9fa;
  border-right: 1px solid #e2e8f0;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #e2e8f0;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: #2d3748;
}

.create-button {
  background-color: #4299e1;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 8px 12px;
  font-size: 0.875rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}

.create-button:hover {
  background-color: #3182ce;
}

.loading-indicator, .empty-state, .error-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px 16px;
  color: #718096;
  text-align: center;
}

.loading-indicator i, .empty-state i {
  font-size: 2rem;
  margin-bottom: 16px;
}

.error-message {
  color: #e53e3e;
}

.agent-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.agent-item {
  display: flex;
  align-items: center;
  padding: 12px;
  border-radius: 6px;
  margin-bottom: 8px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.agent-item:hover {
  background-color: #edf2f7;
}

.agent-item.selected {
  background-color: #ebf8ff;
  border-left: 3px solid #4299e1;
}

.agent-icon {
  margin-right: 12px;
  font-size: 1.25rem;
  color: #4a5568;
}

.agent-info {
  flex: 1;
  min-width: 0;
}

.agent-name {
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.agent-description {
  font-size: 0.875rem;
  color: #718096;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.agent-actions {
  display: flex;
  gap: 8px;
  opacity: 0;
  transition: opacity 0.2s;
}

.agent-item:hover .agent-actions {
  opacity: 1;
}

.edit-button, .delete-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  font-size: 0.875rem;
}

.edit-button {
  color: #4a5568;
}

.edit-button:hover {
  background-color: #edf2f7;
}

.delete-button {
  color: #e53e3e;
}

.delete-button:hover {
  background-color: #fff5f5;
}

.confirmation-dialog {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.confirmation-dialog-content {
  background-color: white;
  border-radius: 8px;
  padding: 24px;
  width: 400px;
  max-width: 90%;
}

.confirmation-dialog-content h3 {
  margin-top: 0;
  color: #2d3748;
}

.confirmation-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}

.confirm-button, .cancel-button {
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
}

.confirm-button {
  background-color: #e53e3e;
  color: white;
  border: none;
}

.confirm-button:hover {
  background-color: #c53030;
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
