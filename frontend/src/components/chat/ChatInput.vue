<template>
    <div class="fixed bottom-0 right-5 border-t transition-all duration-300 ease-in-out rounded-tl-lg shadow-lg" :class="[
        isExpanded ? 'h-[66vh]' : 'h-48',
        'chat-input-container'
    ]" :style="{
        width: isSidebarOpen ? 'calc(75% - 16rem)' : 'calc(75% - 4rem)',
        right: '1rem',
        left: 'auto'
    }">
        <div class="p-3 flex flex-col h-full">
            <div class="mb-2 flex justify-between items-center">
                <div class="w-1/2">
                    <select id="config-select" v-model="selectedConfigIdComputed"
                        class="w-full p-1 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 chat-input-select">
                        <option v-for="config in userLLMConfigs" :key="config.id" :value="config.id">
                            {{ getConfigDescription(config) }}
                        </option>
                    </select>
                </div>
                <button @click="toggleExpand" class="ml-2 p-1 rounded-full chat-input-icon-button">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
                        <path v-if="isExpanded" fill-rule="evenodd"
                            d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z"
                            clip-rule="evenodd" />
                        <path v-else fill-rule="evenodd"
                            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                            clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
            <div class="flex-grow relative flex items-center" :style="{ height: isExpanded ? '40rem' : '3rem' }">
                <div class="flex-grow pr-10 h-full">
                    <MonacoEditor v-model="userInputComputed" :options="editorOptions" language="markdown"
                        :theme="isDarkMode ? 'vs-dark' : 'vs'" />
                </div>
                <button @click="handleSendMessage"
                    :disabled="isLoading || userInputComputed.trim() === '' || !selectedConfigIdComputed || !currentConversation"
                    class="absolute right-0 top-1/2 transform -translate-y-1/2 p-2 rounded-full chat-input-send-button">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M14 5l7 7m0 0l-7 7m7-7H3" />
                    </svg>
                </button>
            </div>
            <div class="mt-2">
                <InputToolbar @attach-file="handleAttachFile" @insert-image="handleInsertImage" />
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, computed, PropType, ref } from 'vue';
import MonacoEditor from '../studio/editors/MonacoEditor.vue';
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
        MonacoEditor,
        InputToolbar,
    },
    props: {
        isSidebarOpen: {
            type: Boolean,
            required: true,
        },
        isExpanded: {
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
        'toggle-expand': () => true,
        'update:selectedConfigId': (configId: string) => typeof configId === 'string',
        'update:userInput': (input: string) => typeof input === 'string',
        'send-message': () => true,
    },
    setup(props, { emit }) {
        const isDarkMode = ref(true); // You may want to make this reactive based on your app's theme

        const selectedConfigIdComputed = computed({
            get: () => props.selectedConfigId,
            set: (value: string) => emit('update:selectedConfigId', value)
        });

        const userInputComputed = computed({
            get: () => props.userInput,
            set: (value: string) => emit('update:userInput', value)
        });

        const toggleExpand = () => {
            emit('toggle-expand');
        };

        const handleSendMessage = () => {
            emit('send-message');
        };

        const getConfigDescription = (config: UserLLMConfig) => {
            return config.description || config.name || `Config ${config.id.slice(0, 8)}`;
        };

        const handleAttachFile = () => {
            // Implement file attachment logic
            console.log('Attach file clicked');
        };

        const handleInsertImage = () => {
            // Implement image insertion logic
            console.log('Insert image clicked');
        };

        const editorOptions = computed(() => ({
            minimap: { enabled: false },
            lineNumbers: 'off',
            glyphMargin: true,
            folding: true,
            lineDecorationsWidth: 0,
            lineNumbersMinChars: 0,
            wordWrap: 'on',
            wrappingStrategy: 'advanced',
            automaticLayout: true,
            scrollBeyondLastLine: false,
            fontSize: 14,
            fontFamily: 'Menlo, Monaco, monospace',
            cursorBlinking: 'smooth',
            cursorSmoothCaretAnimation: true,
            smoothScrolling: true,
            contextmenu: true,
            quickSuggestions: false,
            suggestOnTriggerCharacters: false,
            acceptSuggestionOnEnter: 'on',
            tabCompletion: 'on',
            wordBasedSuggestions: true,
            parameterHints: { enabled: false },
            links: true,
            renderWhitespace: 'none',
            overviewRulerLanes: 0,
            hideCursorInOverviewRuler: true,
            scrollbar: {
                vertical: 'hidden',
                horizontal: 'hidden'
            },
            renderLineHighlight: 'none',
            fixedOverflowWidgets: true,
            lineHeight: 20,
            padding: { top: 4, bottom: 4 }
        }));

        return {
            isDarkMode,
            selectedConfigIdComputed,
            userInputComputed,
            toggleExpand,
            handleSendMessage,
            editorOptions,
            getConfigDescription,
            handleAttachFile,
            handleInsertImage,
        };
    },
});
</script>

<style scoped>
/* Theme-aware styles */
.chat-input-container {
    background-color: var(--chat-input-bg, #1f2937);
    border-color: var(--chat-input-border, #374151);
}

.chat-input-select {
    background-color: var(--chat-input-select-bg, #374151);
    color: var(--chat-input-text, #f3f4f6);
    border-color: var(--chat-input-border, #4b5563);
}

.chat-input-icon-button {
    color: var(--chat-input-icon, #9ca3af);
    background-color: var(--chat-input-icon-bg, #374151);
}

.chat-input-icon-button:hover {
    color: var(--chat-input-icon-hover, #f3f4f6);
}

.chat-input-send-button {
    color: var(--chat-input-send-icon, #3b82f6);
    background-color: var(--chat-input-send-bg, transparent);
}

.chat-input-send-button:hover {
    color: var(--chat-input-send-icon-hover, #2563eb);
}

.chat-input-send-button:disabled {
    color: var(--chat-input-send-icon-disabled, #6b7280);
}

/* Add these styles for the Monaco editor */
:deep(.monaco-editor) {
    width: 100%;
    height: 100%;
}

:deep(.monaco-editor .overflow-guard) {
    border-radius: 0.375rem;
}

/* Light theme variables */
:root {
    --chat-input-bg: #ffffff;
    --chat-input-border: #e5e7eb;
    --chat-input-text: #1f2937;
    --chat-input-select-bg: #f3f4f6;
    --chat-input-icon: #6b7280;
    --chat-input-icon-bg: #f3f4f6;
    --chat-input-icon-hover: #374151;
    --chat-input-send-icon: #3b82f6;
    --chat-input-send-icon-hover: #2563eb;
    --chat-input-send-icon-disabled: #9ca3af;
}

/* Dark theme variables */
.dark-mode {
    --chat-input-bg: #1f2937;
    --chat-input-border: #374151;
    --chat-input-text: #f3f4f6;
    --chat-input-select-bg: #374151;
    --chat-input-icon: #9ca3af;
    --chat-input-icon-bg: #374151;
    --chat-input-icon-hover: #f3f4f6;
    --chat-input-send-icon: #3b82f6;
    --chat-input-send-icon-hover: #60a5fa;
    --chat-input-send-icon-disabled: #6b7280;
}
</style>
