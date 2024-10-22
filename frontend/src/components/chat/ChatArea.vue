<template>
    <div class="flex-1 flex flex-col h-full overflow-hidden">
        <!-- Chat Messages -->
        <div class="flex-grow overflow-y-auto p-4" ref="chatMessages">
            <div class="max-w-4xl mx-auto">
                <div v-if="currentConversation && displayMessages.length > 0">
                    <div v-for="(message, index) in displayMessages" :key="index"
                        :class="['message mb-3 rounded-lg max-w-3xl',
                            message.role === 'user' ? 'bg-blue-600 text-white ml-auto' : 'bg-gray-700 text-white mr-auto']">
                        <ResponseTopToolbar v-if="message.role === 'assistant'"
                            :providerModel="message.provider_model || ''" />
                        <div class="message-content text-sm markdown-body p-3">
                            <template v-if="message.attachment_id">
                                <ImageRenderer :messageId="message.id" :attachmentId="message.attachment_id"
                                    :altText="'Generated image'" />
                            </template>
                            <template v-else>
                                <div v-html="message.renderedContent || message.content"></div>
                            </template>
                        </div>
                        <ResponseToolbar :messageId="message.id" @deleteMessage="handleDeleteMessage" />
                    </div>
                </div>
                <div v-else-if="currentConversation" class="flex items-center justify-center h-full">
                    <p class="text-gray-500 dark:text-gray-400">No messages yet. Start a conversation!</p>
                </div>
                <div v-else class="flex items-center justify-center h-full">
                    <p class="text-gray-500 dark:text-gray-400">Select or create a conversation to start chatting.</p>
                </div>
            </div>
        </div>

        <!-- AI Thinking Indicator -->
        <div v-if="isLoading"
            class="absolute bottom-0 left-0 right-0 bg-gray-800 p-2 text-xs text-gray-400 flex items-center justify-center border-t border-gray-700">
            <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-blue-500" xmlns="http://www.w3.org/2000/svg" fill="none"
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
.markdown-body {
    font-size: 0.875rem;
    line-height: 1.25rem;
}

.markdown-body h1 {
    font-size: 1.5rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
}

.markdown-body h2 {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
}

.markdown-body h3 {
    font-size: 1.125rem;
    font-weight: bold;
    margin-bottom: 0.25rem;
}

.markdown-body ul {
    list-style-type: disc;
    list-style-position: inside;
    margin-bottom: 0.5rem;
}

.markdown-body ol {
    list-style-type: decimal;
    list-style-position: inside;
    margin-bottom: 0.5rem;
}

.markdown-body pre {
    background-color: rgba(255, 255, 255, 0.1);
    padding: 0.5rem;
    border-radius: 0.25rem;
    margin-bottom: 0.5rem;
    overflow-x: auto;
}

.markdown-body code {
    font-family: monospace;
    font-size: 0.875rem;
    background-color: rgba(255, 255, 255, 0.1);
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
}

.markdown-body p {
    margin-bottom: 0.5rem;
}

.markdown-body a {
    color: #3b82f6;
    text-decoration: underline;
}

.markdown-body blockquote {
    border-left-width: 4px;
    border-color: #4b5563;
    padding-left: 0.5rem;
    font-style: italic;
    margin: 0.5rem 0;
}

.markdown-body table {
    border-collapse: collapse;
    border: 1px solid #4b5563;
    margin: 0.5rem 0;
}

.markdown-body th,
.markdown-body td {
    border: 1px solid #4b5563;
    padding: 0.25rem;
}

.markdown-body img {
    max-width: 100%;
    height: auto;
    margin: 0.5rem 0;
}
</style>
