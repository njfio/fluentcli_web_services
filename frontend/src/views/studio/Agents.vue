<template>
  <div class="container mx-auto py-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Agents</h1>
      <button
        @click="showCreateAgentPanel = true"
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2"
      >
        <i class="fas fa-plus"></i>
        <span>Create Agent</span>
      </button>
    </div>

    <div v-if="loading" class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
    </div>

    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-6">
      <span class="block sm:inline">{{ error }}</span>
    </div>

    <div v-else-if="agents.length === 0" class="bg-white dark:bg-gray-800 rounded-lg shadow p-8 text-center">
      <div class="text-gray-500 dark:text-gray-400 mb-4">
        <i class="fas fa-robot text-5xl mb-4"></i>
        <h3 class="text-xl font-semibold mb-2">No Agents Found</h3>
        <p>Create your first agent to start using AI tools in your conversations.</p>
      </div>
      <button
        @click="showCreateAgentPanel = true"
        class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
      >
        Create Agent
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="agent in agents"
        :key="agent.id"
        class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden hover:shadow-md transition-shadow duration-200"
      >
        <div class="p-6">
          <div class="flex items-start justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-blue-600 dark:text-blue-300">
                <i :class="getAgentIcon(agent)"></i>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ agent.name }}</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ agent.description }}</p>
              </div>
            </div>
            <div class="flex gap-2">
              <button
                @click="editAgent(agent)"
                class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
                title="Edit"
              >
                <i class="fas fa-edit"></i>
              </button>
              <button
                @click="confirmDeleteAgent(agent)"
                class="text-red-500 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300"
                title="Delete"
              >
                <i class="fas fa-trash"></i>
              </button>
            </div>
          </div>

          <div class="mt-4">
            <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Tools</h4>
            <div class="flex flex-wrap gap-2">
              <span
                v-for="toolId in agent.tools"
                :key="toolId"
                class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded-full"
              >
                {{ getToolName(toolId) }}
              </span>
            </div>
          </div>

          <div class="mt-4 pt-4 border-t border-gray-100 dark:border-gray-700">
            <button
              @click="selectAgent(agent.id)"
              :class="[
                'w-full py-2 rounded-md text-sm font-medium',
                agent.id === selectedAgentId
                  ? 'bg-blue-600 text-white hover:bg-blue-700'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600'
              ]"
            >
              {{ agent.id === selectedAgentId ? 'Selected' : 'Select Agent' }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Edit Agent Panel -->
    <agent-config-panel
      v-if="showCreateAgentPanel"
      :agent="editingAgent"
      @save="saveAgent"
      @cancel="cancelAgentEdit"
    />

    <!-- Delete Confirmation Dialog -->
    <div v-if="showDeleteConfirmation" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 max-w-md w-full">
        <h3 class="text-lg font-semibold mb-4 text-gray-900 dark:text-white">Delete {{ deletingAgent?.name }}?</h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">This action cannot be undone.</p>
        <div class="flex justify-end gap-4">
          <button
            @click="showDeleteConfirmation = false"
            class="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600"
          >
            Cancel
          </button>
          <button
            @click="deleteAgent"
            class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import AgentConfigPanel from '@/components/AgentConfigPanel.vue';
import { Agent } from '@/types/agent';

export default defineComponent({
  name: 'Agents',
  components: {
    AgentConfigPanel
  },
  setup() {
    const store = useStore();

    const showCreateAgentPanel = ref(false);
    const showDeleteConfirmation = ref(false);
    const editingAgent = ref<Agent | undefined>(undefined);
    const deletingAgent = ref<Agent | null>(null);

    const agents = computed(() => store.state.agent.agents);
    const selectedAgentId = computed(() => store.state.agent.selectedAgentId);
    const loading = computed(() => store.state.agent.loading);
    const error = computed(() => store.state.agent.error);
    const tools = computed(() => store.state.tool.tools);

    onMounted(async () => {
      // Load agents and tools
      await store.dispatch('agent/fetchAgents');
      await store.dispatch('tool/fetchTools');

      // Initialize from localStorage
      store.dispatch('agent/initializeFromLocalStorage');
    });

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

    const getToolName = (toolId: string) => {
      const tool = tools.value.find((t: any) => t.id === toolId);
      if (!tool) return toolId;

      return tool.name
        .split('_')
        .map((word: string) => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
    };

    const selectAgent = (id: string) => {
      store.dispatch('agent/selectAgent', id);
    };

    const editAgent = (agent: Agent) => {
      editingAgent.value = agent;
      showCreateAgentPanel.value = true;
    };

    const confirmDeleteAgent = (agent: Agent) => {
      deletingAgent.value = agent;
      showDeleteConfirmation.value = true;
    };

    const saveAgent = async (agentData: any) => {
      try {
        if (editingAgent.value) {
          await store.dispatch('agent/updateAgent', {
            id: editingAgent.value.id,
            agentData
          });
        } else {
          await store.dispatch('agent/createAgent', agentData);
        }
        showCreateAgentPanel.value = false;
        editingAgent.value = undefined;
      } catch (error) {
        console.error('Error saving agent:', error);
      }
    };

    const cancelAgentEdit = () => {
      showCreateAgentPanel.value = false;
      editingAgent.value = undefined;
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

    return {
      agents,
      selectedAgentId,
      loading,
      error,
      showCreateAgentPanel,
      showDeleteConfirmation,
      editingAgent,
      deletingAgent,
      getAgentIcon,
      getToolName,
      selectAgent,
      editAgent,
      confirmDeleteAgent,
      saveAgent,
      cancelAgentEdit,
      deleteAgent
    };
  }
});
</script>
