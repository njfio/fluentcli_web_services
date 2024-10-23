<template>
    <div
        class="flex-1 flex flex-col h-full overflow-hidden bg-gradient-to-b from-gray-50 to-white dark:from-gray-900 dark:to-gray-800">
        <!-- Chat Messages -->
        <div class="flex-grow overflow-y-auto p-4 space-y-6" ref="chatMessages">
            <div class="max-w-5xl mx-auto">
                <div v-if="currentConversation && displayMessages.length > 0">
                    <div v-for="(message, index) in displayMessages" :key="index"
                        class="message-container animate-fade-in"
                        :class="{ 'mt-8': index > 0 && displayMessages[index - 1]?.role !== message.role }">
                        <div
                            :class="['message rounded-xl shadow-sm max-w-3xl transition-all duration-200 hover:shadow-md',
                                message.role === 'user'
                                    ? 'bg-blue-600 text-white ml-auto'
                                    : 'bg-white dark:bg-gray-700 text-gray-900 dark:text-white mr-auto border border-gray-100 dark:border-gray-600']">
                            <ResponseTopToolbar v-if="message.role === 'assistant'"
                                :providerModel="message.provider_model || ''"
                                class="border-b border-gray-100 dark:border-gray-600" />
                            <div class="message-content text-sm markdown-body p-4">
                                <template v-if="message.attachment_id">
                                    <div class="rounded-lg overflow-hidden shadow-lg">
                                        <ImageRenderer :attachmentId="message.attachment_id"
                                            :altText="'Generated image'" />
                                    </div>
                                </template>
                                <template v-else>
                                    <div v-html="message.renderedContent || message.content"></div>
                                </template>
                            </div>
                            <ResponseToolbar :messageId="message.id" @deleteMessage="handleDeleteMessage"
                                class="border-t border-gray-100 dark:border-gray-600" />
                        </div>
                    </div>
                </div>
                <div v-else-if="currentConversation" class="flex items-center justify-center h-full">
                    <div class="text-center space-y-4">
                        <div class="text-4xl text-gray-300 dark:text-gray-600">💭</div>
                        <p class="text-gray-500 dark:text-gray-400">No messages yet. Start a conversation!</p>
                    </div>
                </div>
                <div v-else class="flex items-center justify-center h-full">
                    <div class="text-center space-y-4">
                        <div class="text-4xl text-gray-300 dark:text-gray-600">👋</div>
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
    },
    setup(props) {
        const { deleteMessage } = useChatLogic();
        const chatMessages = ref<HTMLElement | null>(null);
        const deletedMessageIds = ref<Set<string>>(new Set());

        const displayMessages = computed(() => {
            return props.messages.filter(message => !deletedMessageIds.value.has(message.id));
        });

        const scrollToBottom = () => {
            if (chatMessages.value) {
                chatMessages.value.scrollTop = chatMessages.value.scrollHeight;
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
            });
        });

        onMounted(() => {
            scrollToBottom();
        });

        const handleDeleteMessage = async (messageId: string) => {
            await deleteMessage(messageId);
            deletedMessageIds.value.add(messageId);
        };

        return {
            chatMessages,
            handleDeleteMessage,
            displayMessages,
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
}

.markdown-body h1 {
    font-size: 1.5rem;
    font-weight: bold;
    margin-bottom: 1rem;
    color: inherit;
}

.markdown-body h2 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.75rem;
    color: inherit;
}

.markdown-body h3 {
    font-size: 1.125rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
    color: inherit;
}

.markdown-body ul {
    list-style-type: disc;
    list-style-position: inside;
    margin: 0.75rem 0;
    padding-left: 1rem;
}

.markdown-body ol {
    list-style-type: decimal;
    list-style-position: inside;
    margin: 0.75rem 0;
    padding-left: 1rem;
}

.markdown-body pre {
    background-color: rgba(0, 0, 0, 0.05);
    padding: 1rem;
    border-radius: 0.5rem;
    margin: 0.75rem 0;
    overflow-x: auto;
}

.dark .markdown-body pre {
    background-color: rgba(255, 255, 255, 0.1);
}

.markdown-body code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 0.875rem;
    background-color: rgba(0, 0, 0, 0.05);
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
}

.dark .markdown-body code {
    background-color: rgba(255, 255, 255, 0.1);
}

.markdown-body p {
    margin: 0.75rem 0;
}

.markdown-body a {
    color: #3b82f6;
    text-decoration: underline;
    transition: color 0.2s;
}

.markdown-body a:hover {
    color: #2563eb;
}

.markdown-body blockquote {
    border-left: 4px solid #e5e7eb;
    padding: 0.5rem 1rem;
    font-style: italic;
    margin: 0.75rem 0;
    background-color: rgba(0, 0, 0, 0.02);
    border-radius: 0.25rem;
}

.dark .markdown-body blockquote {
    border-color: #4b5563;
    background-color: rgba(255, 255, 255, 0.05);
}

.markdown-body table {
    border-collapse: collapse;
    width: 100%;
    margin: 0.75rem 0;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    overflow: hidden;
}

.dark .markdown-body table {
    border-color: #4b5563;
}

.markdown-body th,
.markdown-body td {
    border: 1px solid #e5e7eb;
    padding: 0.5rem;
}

.dark .markdown-body th,
.dark .markdown-body td {
    border-color: #4b5563;
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
</style>
