<template>
    <div class="chat-input-container bg-gray-800 border-t border-gray-700">
        <div class="max-w-5xl mx-auto px-4 py-3">
            <div class="grid grid-cols-[auto_1fr_auto] gap-3 items-center">
                <select v-model="selectedConfigIdComputed"
                    class="p-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 chat-input-select">
                    <option v-for="config in userLLMConfigs" :key="config.id" :value="config.id">
                        {{ getConfigDescription(config) }}
                    </option>
                </select>
                <div class="relative">
                    <textarea v-model="userInputComputed" @input="autoResize"
                        class="w-full p-3 pr-12 bg-gray-700 text-white rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
                        :style="{ height: textareaHeight + 'px', minHeight: '44px', maxHeight: '120px' }"
                        placeholder="Type your message here..."></textarea>
                    <button @click="handleSendMessage"
                        :disabled="isLoading || userInputComputed.trim() === '' || !selectedConfigIdComputed || !currentConversation"
                        class="absolute right-3 top-1/2 transform -translate-y-1/2 p-1.5 rounded-full chat-input-send-button hover:bg-gray-600">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M14 5l7 7m0 0l-7 7m7-7H3" />
                        </svg>
                    </button>
                </div>
                <InputToolbar @attach-file="handleAttachFile" @insert-image="handleInsertImage" />
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
    name: 'ChatInput',
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
    },
    setup(props, { emit }) {
        const isDarkMode = ref(true);
        const textareaHeight = ref(44);

        const selectedConfigIdComputed = computed({
            get: () => props.selectedConfigId,
            set: (value: string) => emit('update:selectedConfigId', value)
        });

        const userInputComputed = computed({
            get: () => props.userInput,
            set: (value: string) => emit('update:userInput', value)
        });

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
            isDarkMode,
            selectedConfigIdComputed,
            userInputComputed,
            handleSendMessage,
            getConfigDescription,
            handleAttachFile,
            handleInsertImage,
            autoResize,
            textareaHeight,
        };
    },
});
</script>

<style scoped>
.chat-input-container {
    background-color: var(--chat-input-bg, #1f2937);
    width: 100%;
}

.chat-input-select {
    background-color: var(--chat-input-select-bg, #374151);
    color: var(--chat-input-text, #f3f4f6);
    border-color: var(--chat-input-border, #4b5563);
    min-width: 120px;
}

.chat-input-send-button {
    color: var(--chat-input-send-icon, #3b82f6);
    background-color: var(--chat-input-send-bg, transparent);
    transition: all 0.2s ease-in-out;
}

.chat-input-send-button:hover:not(:disabled) {
    color: var(--chat-input-send-icon-hover, #60a5fa);
    background-color: var(--chat-input-send-bg-hover, rgba(55, 65, 81, 0.5));
}

.chat-input-send-button:disabled {
    color: var(--chat-input-send-icon-disabled, #6b7280);
    cursor: not-allowed;
}

/* Light theme variables */
:root {
    --chat-input-bg: #ffffff;
    --chat-input-text: #1f2937;
    --chat-input-select-bg: #f3f4f6;
    --chat-input-send-icon: #3b82f6;
    --chat-input-send-icon-hover: #2563eb;
    --chat-input-send-icon-disabled: #9ca3af;
    --chat-input-send-bg: transparent;
    --chat-input-send-bg-hover: rgba(243, 244, 246, 0.5);
}

/* Dark theme variables */
.dark-mode {
    --chat-input-bg: #1f2937;
    --chat-input-text: #f3f4f6;
    --chat-input-select-bg: #374151;
    --chat-input-send-icon: #3b82f6;
    --chat-input-send-icon-hover: #60a5fa;
    --chat-input-send-icon-disabled: #6b7280;
    --chat-input-send-bg: transparent;
    --chat-input-send-bg-hover: rgba(55, 65, 81, 0.5);
}
</style>
