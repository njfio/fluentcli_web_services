<template>
    <div class="chat-input-container bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700">
        <div class="max-w-5xl mx-auto px-4 py-3">
            <div class="space-y-3">
                <!-- Model Selection -->
                <div class="flex flex-wrap gap-2">
                    <button v-for="config in userLLMConfigs" :key="config.id" @click="toggleConfig(config.id)"
                        :class="['px-3 py-1.5 text-sm rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500',
                            selectedConfigIdsComputed.includes(config.id)
                                ? 'bg-blue-600 text-white'
                                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600']">
                        {{ getConfigDescription(config) }}
                    </button>
                </div>

                <!-- Input Area -->
                <div class="grid grid-cols-[1fr_auto] gap-3 items-center">
                    <div class="relative">
                        <textarea v-model="userInputComputed" @input="autoResize"
                            class="w-full p-3 pr-12 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 border border-gray-200 dark:border-gray-600"
                            :style="{ height: textareaHeight + 'px', minHeight: '44px', maxHeight: '120px' }"
                            placeholder="Type your message here..."></textarea>
                        <button @click="handleSendMessage"
                            :disabled="isLoading || userInputComputed.trim() === '' || selectedConfigIdsComputed.length === 0 || !currentConversation"
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

                <!-- Helper Text -->
                <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400">
                    <span>Press <kbd class="px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700">Enter</kbd> to
                        send</span>
                    <span>{{ selectedConfigIdsComputed.length }} model{{ selectedConfigIdsComputed.length !== 1 ? 's' :
                        '' }} selected</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, computed, PropType, ref, watch, nextTick } from 'vue';
import InputToolbar from './InputToolbar.vue';

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
    name: 'ChatArenaInput',
    components: {
        InputToolbar,
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
        selectedConfigIds: {
            type: Array as PropType<string[]>,
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
        'update:selectedConfigIds': (configIds: string[]) => Array.isArray(configIds),
        'update:userInput': (input: string) => typeof input === 'string',
        'send-message': () => true,
    },
    setup(props, { emit }) {
        const textareaHeight = ref(44);

        const selectedConfigIdsComputed = computed({
            get: () => props.selectedConfigIds,
            set: (value: string[]) => emit('update:selectedConfigIds', value)
        });

        const userInputComputed = computed({
            get: () => props.userInput,
            set: (value: string) => emit('update:userInput', value)
        });

        const toggleConfig = (configId: string) => {
            const newSelectedIds = [...selectedConfigIdsComputed.value];
            const index = newSelectedIds.indexOf(configId);
            if (index === -1) {
                newSelectedIds.push(configId);
            } else {
                newSelectedIds.splice(index, 1);
            }
            selectedConfigIdsComputed.value = newSelectedIds;
        };

        const handleSendMessage = () => {
            emit('send-message');
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
            selectedConfigIdsComputed,
            userInputComputed,
            handleSendMessage,
            getConfigDescription,
            handleAttachFile,
            handleInsertImage,
            autoResize,
            textareaHeight,
            toggleConfig,
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

kbd {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}
</style>
