<template>
    <div class="chat-input-container bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700">
        <div class="max-w-5xl mx-auto px-4 py-3">
            <div class="space-y-3">
                <div class="grid grid-cols-[auto_1fr_auto] gap-3 items-center">
                    <div class="flex items-center gap-2">
                        <select v-model="selectedConfigIdComputed"
                            class="p-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white border-gray-200 dark:border-gray-600">
                            <option v-for="config in userLLMConfigs" :key="config.id" :value="config.id">
                                {{ getConfigDescription(config) }}
                            </option>
                        </select>
                        <div class="flex items-center gap-1">
                            <input
                                type="checkbox"
                                id="enable-function-calling"
                                v-model="enableFunctionCalling"
                                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                            />
                            <label for="enable-function-calling" class="text-sm text-gray-700 dark:text-gray-300">
                                Enable Tools
                            </label>
                        </div>
                    </div>
                    <div class="relative">
                        <textarea v-model="userInputComputed" @input="autoResize"
                            class="w-full p-3 pr-12 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 border border-gray-200 dark:border-gray-600"
                            :style="{ height: textareaHeight + 'px', minHeight: '44px', maxHeight: '120px' }"
                            placeholder="Type your message here..."></textarea>
                        <button @click="handleSendMessage"
                            :disabled="isLoading || userInputComputed.trim() === '' || !selectedConfigIdComputed || !currentConversation"
                            class="absolute right-3 top-1/2 transform -translate-y-1/2 p-1.5 rounded-full chat-input-send-button hover:bg-gray-100 dark:hover:bg-gray-600">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M14 5l7 7m0 0l-7 7m7-7H3" />
                            </svg>
                        </button>
                    </div>
                    <InputToolbar @attach-file="handleAttachFile" @insert-image="handleInsertImage" />
                </div>

                <div v-if="enableFunctionCalling" class="agent-selector-container">
                    <div class="flex items-center justify-between mb-2">
                        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Select Agent</label>
                        <button @click="showAgentSidebar = !showAgentSidebar" class="text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">
                            {{ showAgentSidebar ? 'Hide Agents' : 'Manage Agents' }}
                        </button>
                    </div>
                    <select v-model="selectedAgentId" class="w-full p-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white border-gray-200 dark:border-gray-600">
                        <option value="">No Agent</option>
                        <option v-for="agent in agents" :key="agent.id" :value="agent.id">
                            {{ agent.name }}
                        </option>
                    </select>

                    <div class="quick-tools-container mt-3">
                        <div class="flex items-center justify-between mb-2">
                            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Quick Tools</label>
                            <span class="text-xs text-gray-500 dark:text-gray-400">Click to insert tool prompt</span>
                        </div>
                        <div class="quick-tools-grid">
                            <button
                                v-for="tool in quickTools"
                                :key="tool.id"
                                @click="addToolPrompt(tool)"
                                class="quick-tool-button"
                                :title="tool.description"
                            >
                                <i :class="getToolIcon(tool.name)"></i>
                                <span>{{ formatToolName(tool.name) }}</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div v-if="showAgentSidebar" class="agent-sidebar-container">
            <agent-sidebar />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, computed, PropType, ref, watch, nextTick } from 'vue';
import { useStore } from 'vuex';
import InputToolbar from './InputToolbar.vue';
import AgentSidebar from '../AgentSidebar.vue';

interface UserLLMConfig {
    id: string;
    name?: string;
    userId: string;
    providerId: string;
    apiKeyId: string;
    description?: string;
}

interface Conversation {
    id: string;
    title: string;
}

export default defineComponent({
    name: 'ChatInput',
    components: {
        InputToolbar,
        AgentSidebar,
    },
    props: {
        isSidebarOpen: {
            type: Boolean,
            required: true,
        },
        userLLMConfigs: {
            type: Array as PropType<UserLLMConfig[]>,
            required: true,
        },
        selectedConfigId: {
            type: String,
            required: true,
        },
        currentConversation: {
            type: Object as PropType<Conversation | null>,
            required: false,
            default: null,
        },
        isLoading: {
            type: Boolean,
            required: true,
        },
        userInput: {
            type: String,
            required: true,
        },
    },
    emits: {
        'update:selectedConfigId': (configId: string) => typeof configId === 'string',
        'update:userInput': (input: string) => typeof input === 'string',
        'send-message': () => true,
        'send-message-with-tools': (agentId: string) => typeof agentId === 'string' || agentId === null,
    },
    setup(props, { emit }) {
        const store = useStore();
        const isDarkMode = ref(true);
        const textareaHeight = ref(44);
        const enableFunctionCalling = ref(false);
        const selectedAgentId = ref('');
        const showAgentSidebar = ref(false);

        // Get agents from store
        const agents = computed(() => store.state.agent.agents);

        // Get tools from store
        const tools = computed(() => store.state.tool.tools);

        // Quick tools for the toolbar
        const quickTools = computed(() => {
            // Get the first 6 tools or fewer if there are less than 6
            return tools.value.slice(0, 6);
        });

        // Load agents if not already loaded
        if (agents.value.length === 0) {
            store.dispatch('agent/fetchAgents');
        }

        // Load tools if not already loaded
        if (tools.value.length === 0) {
            store.dispatch('tool/fetchTools');
        }

        // Format tool name for display (convert snake_case to Title Case)
        const formatToolName = (name: string) => {
            return name
                .split('_')
                .map(word => word.charAt(0).toUpperCase() + word.slice(1))
                .join(' ');
        };

        // Get icon for tool
        const getToolIcon = (toolName: string) => {
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

            return iconMap[toolName] || 'fas fa-tools';
        };

        // Add tool prompt to the user input
        const addToolPrompt = (tool: any) => {
            const currentInput = userInputComputed.value;
            const toolPrompt = `Use the ${formatToolName(tool.name)} tool to `;

            // If the input is empty or ends with a period, add the prompt directly
            if (currentInput.trim() === '' || currentInput.trim().endsWith('.')) {
                userInputComputed.value = currentInput.trim() + (currentInput.trim() ? ' ' : '') + toolPrompt;
            } else {
                // Otherwise, add a period and then the prompt
                userInputComputed.value = currentInput.trim() + '. ' + toolPrompt;
            }

            // Focus the textarea
            nextTick(() => {
                const textarea = document.querySelector('textarea');
                if (textarea) {
                    textarea.focus();
                    // Place cursor at the end
                    const length = textarea.value.length;
                    textarea.setSelectionRange(length, length);
                }
            });
        };

        const selectedConfigIdComputed = computed({
            get: () => props.selectedConfigId,
            set: (value: string) => emit('update:selectedConfigId', value)
        });

        const userInputComputed = computed({
            get: () => props.userInput,
            set: (value: string) => emit('update:userInput', value)
        });

        const handleSendMessage = () => {
            if (enableFunctionCalling.value && selectedAgentId.value) {
                emit('send-message-with-tools', selectedAgentId.value);
            } else {
                emit('send-message');
            }
        };

        const getConfigDescription = (config: UserLLMConfig) => {
            return config.description || config.name || `Config ${config.id.slice(0, 8)}`;
        };

        const handleAttachFile = () => {
            console.log('Attach file clicked');
        };

        const handleInsertImage = () => {
            console.log('Insert image clicked');
        };

        const autoResize = (e: Event) => {
            const textarea = e.target as HTMLTextAreaElement;
            textarea.style.height = '44px';
            const newHeight = Math.min(Math.max(textarea.scrollHeight, 44), 120);
            textarea.style.height = newHeight + 'px';
            textareaHeight.value = newHeight;
        };

        watch(userInputComputed, () => {
            nextTick(() => {
                autoResize({ target: document.querySelector('textarea') } as Event);
            });
        });

        return {
            isDarkMode,
            selectedConfigIdComputed,
            userInputComputed,
            handleSendMessage,
            getConfigDescription,
            handleAttachFile,
            handleInsertImage,
            autoResize,
            textareaHeight,
            enableFunctionCalling,
            selectedAgentId,
            showAgentSidebar,
            agents,
            tools,
            quickTools,
            formatToolName,
            getToolIcon,
            addToolPrompt
        };
    },
});
</script>

<style scoped>
.chat-input-container {
    width: 100%;
}

.chat-input-send-button {
    color: var(--chat-input-send-icon, #3b82f6);
    background-color: var(--chat-input-send-bg, transparent);
    transition: all 0.2s ease-in-out;
}

.chat-input-send-button:hover:not(:disabled) {
    color: var(--chat-input-send-icon-hover, #2563eb);
}

.chat-input-send-button:disabled {
    color: var(--chat-input-send-icon-disabled, #9ca3af);
    cursor: not-allowed;
}

/* Light theme variables */
:root {
    --chat-input-bg: #ffffff;
    --chat-input-text: #111827;
    --chat-input-send-icon: #3b82f6;
    --chat-input-send-icon-hover: #2563eb;
    --chat-input-send-icon-disabled: #9ca3af;
    --chat-input-send-bg: transparent;
    --chat-input-send-bg-hover: rgba(243, 244, 246, 0.5);
}

/* Dark theme variables */
.dark {
    --chat-input-bg: #1f2937;
    --chat-input-text: #f3f4f6;
    --chat-input-send-icon: #3b82f6;
    --chat-input-send-icon-hover: #60a5fa;
    --chat-input-send-icon-disabled: #6b7280;
    --chat-input-send-bg: transparent;
    --chat-input-send-bg-hover: rgba(55, 65, 81, 0.5);
}

select {
    appearance: none;
    background-image: url("data:image/svg+xml;charset=utf-8,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3E%3Cpath stroke='%236B7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3E%3C/svg%3E");
    background-position: right 0.5rem center;
    background-repeat: no-repeat;
    background-size: 1.5em 1.5em;
    padding-right: 2.5rem;
}

.dark select {
    background-image: url("data:image/svg+xml;charset=utf-8,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3E%3Cpath stroke='%239CA3AF' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3E%3C/svg%3E");
}

select:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.5);
}

.agent-selector-container {
    padding: 8px;
    background-color: #f9fafb;
    border-radius: 6px;
    border: 1px solid #e5e7eb;
}

.dark .agent-selector-container {
    background-color: #374151;
    border-color: #4b5563;
}

.agent-sidebar-container {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    z-index: 50;
    box-shadow: -2px 0 10px rgba(0, 0, 0, 0.1);
}

.quick-tools-container {
    margin-top: 12px;
}

.quick-tools-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
}

.quick-tool-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 8px;
    background-color: #f3f4f6;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease-in-out;
    font-size: 0.75rem;
    color: #4b5563;
}

.quick-tool-button i {
    font-size: 1rem;
    margin-bottom: 4px;
}

.quick-tool-button span {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
    text-align: center;
}

.quick-tool-button:hover {
    background-color: #e5e7eb;
    transform: translateY(-1px);
}

.dark .quick-tool-button {
    background-color: #374151;
    border-color: #4b5563;
    color: #d1d5db;
}

.dark .quick-tool-button:hover {
    background-color: #4b5563;
}
</style>
