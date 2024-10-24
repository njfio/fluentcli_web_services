<template>
    <div class="h-full flex flex-col bg-white dark:bg-[#1e2329]">
        <!-- Layout Controls for Arena View -->
        <div v-if="isArenaView"
            class="flex items-center justify-end space-x-4 p-4 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
            <div class="flex items-center space-x-4">
                <div class="flex items-center space-x-2">
                    <label class="text-sm text-gray-600 dark:text-gray-300">Layout:</label>
                    <select v-model="localGridLayout"
                        class="px-3 py-1 rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200"
                        @change="updateLayout">
                        <option value="standard">Standard Grid</option>
                        <option value="brickwork">Brickwork</option>
                    </select>
                </div>

                <template v-if="localGridLayout === 'standard'">
                    <div class="flex items-center space-x-2">
                        <label class="text-sm text-gray-600 dark:text-gray-300">Columns:</label>
                        <select v-model="localGridColumns"
                            class="px-3 py-1 rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200"
                            @change="updateLayout">
                            <option v-for="n in 4" :key="n" :value="n">{{ n }}</option>
                        </select>
                    </div>

                    <button @click="toggleExpandAll"
                        class="px-3 py-1 rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors flex items-center space-x-1">
                        <span>{{ isAllExpanded ? 'Collapse All' : 'Expand All' }}</span>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transition-transform"
                            :class="{ 'rotate-180': isAllExpanded }" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                        </svg>
                    </button>
                </template>
            </div>
        </div>

        <div class="flex-1 overflow-y-auto" ref="chatMessages">
            <div class="max-w-7xl mx-auto px-4">
                <div v-if="currentConversation && displayMessages.length > 0">
                    <template v-for="(message, index) in displayMessages" :key="message.id">
                        <!-- User messages -->
                        <div v-if="message.role === 'user'" class="message-container animate-fade-in py-2 w-full"
                            :class="{ 'mt-8': index > 0 && displayMessages[index - 1]?.role !== message.role }">
                            <div
                                class="message rounded-xl shadow-sm max-w-3xl transition-all duration-200 hover:shadow-md bg-blue-600 text-white ml-auto">
                                <div class="message-content text-sm markdown-body p-4">
                                    <div v-html="message.renderedContent || message.content"></div>
                                </div>
                            </div>
                        </div>

                        <!-- Assistant messages -->
                        <div v-else-if="shouldStartNewAssistantGroup(index)" :class="[
                            'w-full mt-8',
                            isArenaView && localGridLayout === 'standard' ? `grid-container grid-cols-${localGridColumns}` : '',
                            isArenaView && localGridLayout === 'brickwork' ? 'masonry-layout' : ''
                        ]">
                            <template v-for="(groupMessage, groupIndex) in getAssistantMessageGroup(index)"
                                :key="groupMessage.id">
                                <div class="message-container animate-fade-in" :class="[
                                    isArenaView && localGridLayout === 'standard' ? 'grid-item' : '',
                                    isArenaView && localGridLayout === 'brickwork' ? 'masonry-item' : 'w-full max-w-3xl',
                                ]" :style="{ 'animation-delay': `${groupIndex * 100}ms` }">
                                    <div
                                        class="message rounded-xl shadow-sm transition-all duration-200 hover:shadow-md bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-600">
                                        <ResponseTopToolbar :providerModel="groupMessage.provider_model || ''"
                                            class="border-b border-gray-200 dark:border-gray-600" />
                                        <div class="message-content text-sm markdown-body p-4" :class="{
                                            'max-h-96 overflow-y-auto':
                                                (isArenaView && localGridLayout === 'standard' && !expandedMessages.has(groupMessage.id)) ||
                                                (!isArenaView && !expandedMessages.has(groupMessage.id))
                                        }">
                                            <template v-if="groupMessage.attachment_id">
                                                <div class="rounded-lg overflow-hidden shadow-lg">
                                                    <ImageRenderer :attachmentId="groupMessage.attachment_id"
                                                        :altText="'Generated image'" />
                                                </div>
                                            </template>
                                            <template v-else>
                                                <div v-html="groupMessage.renderedContent || groupMessage.content">
                                                </div>
                                            </template>
                                        </div>
                                        <div
                                            class="flex items-center justify-between border-t border-gray-200 dark:border-gray-600 p-2">
                                            <button v-if="!isArenaView || localGridLayout !== 'brickwork'"
                                                @click="toggleExpand(groupMessage.id)"
                                                class="text-sm text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300">
                                                {{ expandedMessages.has(groupMessage.id) ? 'Collapse' : 'Expand' }}
                                            </button>
                                            <ResponseToolbar :messageId="groupMessage.id"
                                                @deleteMessage="handleDeleteMessage" />
                                        </div>
                                    </div>
                                </div>
                            </template>
                        </div>
                    </template>
                </div>
                <div v-else-if="currentConversation" class="h-full flex items-center justify-center">
                    <div class="text-center space-y-4">
                        <div class="text-4xl text-gray-400 dark:text-gray-600">💭</div>
                        <p class="text-gray-500 dark:text-gray-400">No messages yet. Start a conversation!</p>
                    </div>
                </div>
                <div v-else class="h-full flex items-center justify-center">
                    <div class="text-center space-y-4">
                        <div class="text-4xl text-gray-400 dark:text-gray-600">👋</div>
                        <p class="text-gray-500 dark:text-gray-400">Select or create a conversation to start chatting.
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <!-- AI Thinking Indicator -->
        <div v-if="isLoading"
            class="absolute bottom-0 left-0 right-0 bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm p-3 text-sm text-gray-600 dark:text-gray-400 flex items-center justify-center border-t border-gray-200 dark:border-gray-700 transition-all duration-300">
            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-blue-500" xmlns="http://www.w3.org/2000/svg" fill="none"
                viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                </path>
            </svg>
            AI is thinking...
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType, ref, watch, nextTick, onMounted, computed } from 'vue';
import { useStore } from 'vuex';
import ResponseToolbar from './ResponseToolbar.vue';
import ResponseTopToolbar from './ResponseTopToolbar.vue';
import ImageRenderer from './ImageRenderer.vue';
import { useChatLogic } from './ChatLogic';

interface Message {
    id: string;
    role: string;
    content: string;
    renderedContent?: string;
    provider_model?: string;
    attachment_id?: string;
    createdAt: string;
}

interface Conversation {
    id: string;
    title: string;
}

export default defineComponent({
    name: 'ChatArea',
    components: {
        ResponseToolbar,
        ResponseTopToolbar,
        ImageRenderer,
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
        currentConversation: {
            type: Object as PropType<Conversation | null>,
            required: true,
        },
        messages: {
            type: Array as PropType<Message[]>,
            required: true,
        },
        isLoading: {
            type: Boolean,
            required: true,
        },
        isArenaView: {
            type: Boolean,
            default: false,
        },
    },
    setup(props) {
        const store = useStore();
        const { deleteMessage } = useChatLogic();
        const chatMessages = ref<HTMLElement | null>(null);
        const deletedMessageIds = ref<Set<string>>(new Set());
        const expandedMessages = ref<Set<string>>(new Set());
        const processedGroups = ref<Set<string>>(new Set());

        // Local state with store sync
        const localGridLayout = ref<'standard' | 'brickwork'>('standard');
        const localGridColumns = ref(3);

        // Computed property for expand all state
        const isAllExpanded = computed(() => {
            const assistantMessages = displayMessages.value.filter(m => m.role === 'assistant');
            return assistantMessages.length > 0 && assistantMessages.every(m => expandedMessages.value.has(m.id));
        });

        const displayMessages = computed(() => {
            return props.messages
                .filter(message => !deletedMessageIds.value.has(message.id))
                .sort((a, b) => new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime());
        });

        const scrollToBottom = () => {
            if (chatMessages.value) {
                chatMessages.value.scrollTop = chatMessages.value.scrollHeight;
            }
        };

        const shouldStartNewAssistantGroup = (index: number) => {
            const message = displayMessages.value[index];
            if (message.role !== 'assistant') return false;

            const prevMessage = index > 0 ? displayMessages.value[index - 1] : null;
            return !prevMessage || prevMessage.role !== 'assistant';
        };

        const getAssistantMessageGroup = (startIndex: number) => {
            const messages = [];
            let i = startIndex;

            while (i < displayMessages.value.length &&
                displayMessages.value[i].role === 'assistant') {
                messages.push(displayMessages.value[i]);
                i++;
            }

            messages.sort((a, b) => new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime());

            const groupId = messages.map(m => m.id).join('-');
            if (!processedGroups.value.has(groupId)) {
                processedGroups.value.add(groupId);
            }

            return messages;
        };

        const toggleExpand = (messageId: string) => {
            if (expandedMessages.value.has(messageId)) {
                expandedMessages.value.delete(messageId);
            } else {
                expandedMessages.value.add(messageId);
            }
        };

        const toggleExpandAll = () => {
            const assistantMessages = displayMessages.value.filter(m => m.role === 'assistant');
            if (isAllExpanded.value) {
                // Collapse all
                assistantMessages.forEach(m => expandedMessages.value.delete(m.id));
            } else {
                // Expand all
                assistantMessages.forEach(m => expandedMessages.value.add(m.id));
            }
        };

        const updateLayout = () => {
            try {
                store.dispatch('chatUI/setGridLayout', localGridLayout.value);
                store.dispatch('chatUI/setGridColumns', localGridColumns.value);
            } catch (error) {
                console.warn('Store not ready for layout updates');
            }
        };

        // Initialize from store if available
        onMounted(() => {
            try {
                if (store.state.chatUI) {
                    localGridLayout.value = store.state.chatUI.gridLayout || 'standard';
                    localGridColumns.value = store.state.chatUI.gridColumns || 3;
                }
            } catch (error) {
                console.warn('Store not initialized for chatUI');
            }
            scrollToBottom();
        });

        // Watch store changes
        watch(() => store.state.chatUI?.gridLayout, (newLayout) => {
            if (newLayout && newLayout !== localGridLayout.value) {
                localGridLayout.value = newLayout;
            }
        });

        watch(() => store.state.chatUI?.gridColumns, (newColumns) => {
            if (newColumns && newColumns !== localGridColumns.value) {
                localGridColumns.value = newColumns;
            }
        });

        watch(displayMessages, () => {
            nextTick(() => {
                scrollToBottom();
            });
        }, { deep: true });

        watch(() => props.currentConversation, () => {
            nextTick(() => {
                scrollToBottom();
                expandedMessages.value.clear();
                processedGroups.value.clear();
            });
        });

        const handleDeleteMessage = async (messageId: string) => {
            await deleteMessage(messageId);
            deletedMessageIds.value.add(messageId);
            expandedMessages.value.delete(messageId);
        };

        return {
            chatMessages,
            handleDeleteMessage,
            displayMessages,
            expandedMessages,
            toggleExpand,
            shouldStartNewAssistantGroup,
            getAssistantMessageGroup,
            localGridLayout,
            localGridColumns,
            updateLayout,
            isAllExpanded,
            toggleExpandAll
        };
    },
});
</script>

<style scoped>
.message-container {
    opacity: 0;
    animation: fadeIn 0.3s ease-in-out forwards;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(10px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.grid-container {
    display: grid;
    gap: 1rem;
    align-items: start;
}

.grid-item {
    width: 100%;
    height: fit-content;
    display: flex;
    flex-direction: column;
}

.grid-cols-1 {
    grid-template-columns: repeat(1, minmax(0, 1fr));
}

.grid-cols-2 {
    grid-template-columns: repeat(2, minmax(0, 1fr));
}

.grid-cols-3 {
    grid-template-columns: repeat(3, minmax(0, 1fr));
}

.grid-cols-4 {
    grid-template-columns: repeat(4, minmax(0, 1fr));
}

.masonry-layout {
    column-count: 3;
    column-gap: 1rem;
    margin: 1rem 0;
    width: 100%;
}

@media (max-width: 1200px) {
    .masonry-layout {
        column-count: 2;
    }
}

@media (max-width: 768px) {
    .masonry-layout {
        column-count: 1;
    }
}

.masonry-item {
    break-inside: avoid;
    margin-bottom: 1rem;
    display: inline-block;
    width: 100%;
}

.masonry-item .message {
    overflow-x: auto;
    max-width: 100%;
}

.masonry-item pre {
    max-width: none;
    overflow-x: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
}

.masonry-item .markdown-body {
    overflow-wrap: break-word;
    word-wrap: break-word;
    word-break: break-word;
}

.masonry-item table {
    display: block;
    overflow-x: auto;
    white-space: nowrap;
}

.markdown-body {
    font-size: 1.125rem;
    line-height: 1.75;
    color: inherit;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
    color: inherit;
    margin-top: 1.5em;
    margin-bottom: 0.75em;
    font-weight: 600;
    line-height: 1.3;
}

.markdown-body h1 {
    font-size: 1.875rem;
    border-bottom: 2px solid rgba(0, 0, 0, 0.1);
    padding-bottom: 0.5rem;
}

.dark .markdown-body h1 {
    border-bottom-color: rgba(255, 255, 255, 0.1);
}

.markdown-body h2 {
    font-size: 1.5rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    padding-bottom: 0.3rem;
}

.dark .markdown-body h2 {
    border-bottom-color: rgba(255, 255, 255, 0.1);
}

.markdown-body h3 {
    font-size: 1.25rem;
}

.markdown-body p {
    margin: 1rem 0;
    line-height: 1.75;
}

.markdown-body ul,
.markdown-body ol {
    padding-left: 1.75em;
    margin: 0.75em 0;
}

.markdown-body ul {
    list-style-type: disc;
}

.markdown-body ol {
    list-style-type: decimal;
}

.markdown-body li {
    margin: 0.5em 0;
    line-height: 1.75;
}

.markdown-body pre {
    background-color: rgba(0, 0, 0, 0.05);
    padding: 1.25rem;
    border-radius: 0.75rem;
    margin: 1rem 0;
    overflow-x: auto;
    font-size: 1rem;
    line-height: 1.6;
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.dark .markdown-body pre {
    background-color: rgba(0, 0, 0, 0.2);
    border-color: rgba(255, 255, 255, 0.1);
}

.markdown-body code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 1rem;
    background-color: rgba(0, 0, 0, 0.05);
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
}

.dark .markdown-body code {
    background-color: rgba(0, 0, 0, 0.2);
}

.markdown-body a {
    color: #3b82f6;
    text-decoration: underline;
    transition: all 0.2s;
    font-weight: 500;
}

.dark .markdown-body a {
    color: #60a5fa;
}

.markdown-body a:hover {
    color: #2563eb;
    background-color: rgba(59, 130, 246, 0.1);
}

.dark .markdown-body a:hover {
    color: #93c5fd;
    background-color: rgba(96, 165, 250, 0.1);
}

.markdown-body blockquote {
    border-left: 4px solid #3b82f6;
    padding: 1rem 1.5rem;
    margin: 1.5rem 0;
    background-color: rgba(59, 130, 246, 0.1);
    border-radius: 0.5rem;
}

.dark .markdown-body blockquote {
    border-color: #60a5fa;
    background-color: rgba(96, 165, 250, 0.1);
}

.markdown-body img {
    max-width: 100%;
    height: auto;
    margin: 1.5rem 0;
    border-radius: 0.75rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.dark .markdown-body img {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3), 0 2px 4px -1px rgba(0, 0, 0, 0.18);
}

.message.expanded .message-content {
    max-height: none !important;
}

/* Table styles */
.markdown-body table {
    width: 100%;
    margin: 1.5rem 0;
    border-collapse: separate;
    border-spacing: 0;
    border-radius: 0.5rem;
    overflow: hidden;
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.dark .markdown-body table {
    border-color: rgba(255, 255, 255, 0.1);
}

.markdown-body th,
.markdown-body td {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    border-right: 1px solid rgba(0, 0, 0, 0.1);
    text-align: left;
}

.dark .markdown-body th,
.dark .markdown-body td {
    border-color: rgba(255, 255, 255, 0.1);
}

.markdown-body th {
    background-color: rgba(0, 0, 0, 0.05);
    font-weight: 600;
}

.dark .markdown-body th {
    background-color: rgba(255, 255, 255, 0.05);
}

.markdown-body tr:last-child td {
    border-bottom: none;
}

.markdown-body td:last-child,
.markdown-body th:last-child {
    border-right: none;
}
</style>
