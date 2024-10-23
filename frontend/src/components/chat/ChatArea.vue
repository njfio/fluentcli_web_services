<template>
    <div class="h-full flex flex-col bg-white dark:bg-[#1e2329]">
        <div class="flex-1 overflow-y-auto" ref="chatMessages">
            <div class="max-w-5xl mx-auto px-4">
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
                            isArenaView ? 'grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3 place-items-center' : ''
                        ]">
                            <template v-for="(groupMessage, groupIndex) in getAssistantMessageGroup(index)"
                                :key="groupMessage.id">
                                <div class="message-container animate-fade-in" :class="[
                                    isArenaView ? 'w-full max-w-md' : 'w-full max-w-3xl',
                                ]" :style="{ 'animation-delay': `${groupIndex * 100}ms` }">
                                    <div class="message rounded-xl shadow-sm transition-all duration-200 hover:shadow-md bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-600"
                                        :class="{ 'expanded': expandedMessages.has(groupMessage.id) }">
                                        <ResponseTopToolbar :providerModel="groupMessage.provider_model || ''"
                                            class="border-b border-gray-200 dark:border-gray-600" />
                                        <div class="message-content text-sm markdown-body p-4"
                                            :class="{ 'max-h-96 overflow-y-auto': !expandedMessages.has(groupMessage.id) }">
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
                                            <button @click="toggleExpand(groupMessage.id)"
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
        const { deleteMessage } = useChatLogic();
        const chatMessages = ref<HTMLElement | null>(null);
        const deletedMessageIds = ref<Set<string>>(new Set());
        const expandedMessages = ref<Set<string>>(new Set());
        const processedGroups = ref<Set<string>>(new Set());

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

            // Start a new group if this is the first message or previous message is from user
            const prevMessage = index > 0 ? displayMessages.value[index - 1] : null;
            return !prevMessage || prevMessage.role !== 'assistant';
        };

        const getAssistantMessageGroup = (startIndex: number) => {
            const messages = [];
            let i = startIndex;

            // Get all consecutive assistant messages
            while (i < displayMessages.value.length &&
                displayMessages.value[i].role === 'assistant') {
                messages.push(displayMessages.value[i]);
                i++;
            }

            // Sort messages by creation date
            messages.sort((a, b) => new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime());

            // Mark this group as processed
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

        onMounted(() => {
            scrollToBottom();
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

.markdown-body {
    font-size: 0.875rem;
    line-height: 1.5;
    color: inherit;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
    color: inherit;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}

.markdown-body h1 {
    font-size: 1.5rem;
}

.markdown-body h2 {
    font-size: 1.25rem;
}

.markdown-body h3 {
    font-size: 1.125rem;
}

.markdown-body ul,
.markdown-body ol {
    padding-left: 1.5em;
    margin: 0.5em 0;
}

.markdown-body ul {
    list-style-type: disc;
}

.markdown-body ol {
    list-style-type: decimal;
}

.markdown-body pre {
    background-color: rgba(0, 0, 0, 0.1);
    padding: 1rem;
    border-radius: 0.5rem;
    margin: 0.75rem 0;
    overflow-x: auto;
}

.dark .markdown-body pre {
    background-color: rgba(0, 0, 0, 0.2);
}

.markdown-body code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 0.875rem;
    background-color: rgba(0, 0, 0, 0.1);
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
}

.dark .markdown-body code {
    background-color: rgba(0, 0, 0, 0.2);
}

.markdown-body p {
    margin: 0.75rem 0;
}

.markdown-body a {
    color: #3b82f6;
    text-decoration: underline;
    transition: color 0.2s;
}

.dark .markdown-body a {
    color: #60a5fa;
}

.markdown-body a:hover {
    color: #2563eb;
}

.dark .markdown-body a:hover {
    color: #93c5fd;
}

.markdown-body blockquote {
    border-left: 4px solid #e5e7eb;
    padding: 0.5rem 1rem;
    font-style: italic;
    margin: 0.75rem 0;
    background-color: rgba(0, 0, 0, 0.05);
    border-radius: 0.25rem;
}

.dark .markdown-body blockquote {
    border-color: #4b5563;
    background-color: rgba(0, 0, 0, 0.2);
}

.markdown-body img {
    max-width: 100%;
    height: auto;
    margin: 0.75rem 0;
    border-radius: 0.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.dark .markdown-body img {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3), 0 2px 4px -1px rgba(0, 0, 0, 0.18);
}

.message.expanded .message-content {
    max-height: none !important;
}
</style>
