<template>
    <div class="fixed bottom-0 right-5 bg-gray-800 border-t border-gray-700 transition-all duration-300 ease-in-out rounded-tl-lg shadow-lg"
        :class="[
            isExpanded ? 'h-[66vh]' : 'h-40',
        ]" :style="{
            width: isSidebarOpen ? 'calc(75% - 16rem)' : 'calc(75% - 4rem)',
            right: '1rem',
            left: 'auto'
        }">
        <div class="p-3 flex flex-col h-full">
            <div class="mb-2 flex justify-between items-center">
                <label for="config-select" class="block text-xs font-medium text-gray-300">Select User LLM
                    Config:</label>
                <button @click="toggleExpand" class="text-blue-400 text-sm">
                    {{ isExpanded ? 'Collapse' : 'Expand' }}
                </button>
            </div>
            <select id="config-select" v-model="selectedConfigIdComputed"
                class="w-full p-1 text-sm border border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-gray-700 text-gray-100 mb-2">
                <option v-for="config in userLLMConfigs" :key="config.id" :value="config.id">
                    {{ config.name || `Config ${config.id}` }}
                </option>
            </select>
            <div class="flex-grow relative" :style="{ height: isExpanded ? '40rem' : '2rem' }">
                <MonacoEditor v-model="userInput" :options="editorOptions" language="markdown" theme="vs-dark"
                    @send-message="sendMessage" />
            </div>
            <div class="mt-2 flex justify-end">
                <button @click="sendMessage"
                    :disabled="isLoading || userInput.trim() === '' || !selectedConfigIdComputed || !currentConversation"
                    class="px-4 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 disabled:opacity-50 disabled:cursor-not-allowed transition duration-150 ease-in-out">
                    Send
                </button>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, PropType } from 'vue';
import MonacoEditor from '../studio/editors/MonacoEditor.vue';

interface UserLLMConfig {
    id: string;
    name?: string;
    userId: string;
    providerId: string;
    apiKeyId: string;
}

interface Conversation {
    id: string;
    title: string;
}

export default defineComponent({
    name: 'ChatInput',
    components: {
        MonacoEditor,
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
    },
    emits: {
        'toggle-expand': () => true,
        'update:selectedConfigId': (configId: string) => typeof configId === 'string',
        'send-message': (message: string) => typeof message === 'string',
    },
    setup(props, { emit }) {
        const userInput = ref('');

        const selectedConfigIdComputed = computed({
            get: () => props.selectedConfigId,
            set: (value: string) => emit('update:selectedConfigId', value)
        });

        const toggleExpand = () => {
            emit('toggle-expand');
        };

        const sendMessage = () => {
            if (userInput.value.trim() !== '' && props.currentConversation) {
                emit('send-message', userInput.value);
                userInput.value = '';
            }
        };

        const editorOptions = computed(() => ({
            minimap: { enabled: false },
            lineNumbers: 'off',
            glyphMargin: false,
            folding: false,
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
            contextmenu: false,
            quickSuggestions: false,
            suggestOnTriggerCharacters: false,
            acceptSuggestionOnEnter: 'off',
            tabCompletion: 'off',
            wordBasedSuggestions: false,
            parameterHints: { enabled: false },
            links: false,
            renderWhitespace: 'none',
            overviewRulerLanes: 0,
            hideCursorInOverviewRuler: true,
            scrollbar: {
                vertical: 'hidden',
                horizontal: 'hidden'
            },
            renderLineHighlight: 'none',
            fixedOverflowWidgets: true
        }));

        return {
            userInput,
            selectedConfigIdComputed,
            toggleExpand,
            sendMessage,
            editorOptions,
        };
    },
});
</script>

<style scoped>
/* Add these styles for the Monaco editor */
:deep(.monaco-editor) {
    width: 100%;
    height: 100%;
}

:deep(.monaco-editor .overflow-guard) {
    border-radius: 0.375rem;
}
</style>
