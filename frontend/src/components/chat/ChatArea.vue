<template>
    <div class="flex-1 flex flex-col relative transition-all duration-300 ease-in-out"
        :class="[isSidebarOpen ? 'ml-64' : 'ml-16']" :style="{
            maxWidth: isSidebarOpen ? 'calc(100% - 20rem)' : 'calc(100% - 10rem)',
            width: '100%'
        }">
        <!-- Chat Messages -->
        <div class="flex-1 overflow-y-auto p-4" :style="{
            paddingBottom: isExpanded ? 'calc(66vh + 1rem)' : '8rem',
            maxWidth: '100%'
        }" ref="chatMessages">
            <div v-if="currentConversation && messages.length > 0">
                <div v-for="(message, index) in messages" :key="index" :class="['message mb-3 p-3 rounded-lg max-w-3xl',
                    message.role === 'user' ? 'bg-blue-600 text-white ml-auto' : 'bg-gray-700 text-white mr-auto']">
                    <div class="message-content text-sm markdown-body"
                        v-html="message.renderedContent || message.content">
                    </div>
                </div>
            </div>
            <div v-else-if="currentConversation" class="flex items-center justify-center h-full">
                <p class="text-gray-500 dark:text-gray-400">No messages yet. Start a conversation!</p>
            </div>
            <div v-else class="flex items-center justify-center h-full">
                <p class="text-gray-500 dark:text-gray-400">Select or create a conversation to start chatting.</p>
            </div>
        </div>

        <!-- AI Thinking Indicator -->
        <div v-if="isLoading"
            class="absolute top-0 left-0 right-0 -translate-y-full bg-gray-800 p-2 text-xs text-gray-400 flex items-center justify-center border-t border-gray-700">
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
import { defineComponent, PropType, ref, watch, nextTick, onMounted } from 'vue';

interface Message {
    role: string;
    content: string;
    renderedContent?: string;
}

interface Conversation {
    id: string;
    title: string;
}

export default defineComponent({
    name: 'ChatArea',
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
        const chatMessages = ref<HTMLElement | null>(null);

        const scrollToBottom = () => {
            if (chatMessages.value) {
                chatMessages.value.scrollTop = chatMessages.value.scrollHeight;
            }
        };

        watch(() => props.messages, () => {
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

        return {
            chatMessages,
        };
    },
});
</script>

<style scoped>
.markdown-body {
    @apply text-sm;
}

.markdown-body :deep(h1) {
    @apply text-2xl font-bold mb-2;
}

.markdown-body :deep(h2) {
    @apply text-xl font-bold mb-2;
}

.markdown-body :deep(h3) {
    @apply text-lg font-bold mb-1;
}

.markdown-body :deep(ul) {
    @apply list-disc list-inside mb-2;
}

.markdown-body :deep(ol) {
    @apply list-decimal list-inside mb-2;
}

.markdown-body :deep(pre) {
    @apply bg-gray-100 dark:bg-gray-700 p-2 rounded mb-2 overflow-x-auto;
}

.markdown-body :deep(code) {
    @apply font-mono text-sm bg-gray-100 dark:bg-gray-700 p-1 rounded;
}

.markdown-body :deep(p) {
    @apply mb-2;
}

.markdown-body :deep(a) {
    @apply text-blue-600 dark:text-blue-400 underline;
}

.markdown-body :deep(blockquote) {
    @apply border-l-4 border-gray-300 dark:border-gray-600 pl-2 italic my-2;
}

.markdown-body :deep(table) {
    @apply border-collapse border border-gray-300 dark:border-gray-600 my-2;
}

.markdown-body :deep(th),
.markdown-body :deep(td) {
    @apply border border-gray-300 dark:border-gray-600 p-1;
}

.markdown-body :deep(img) {
    @apply max-w-full h-auto my-2;
}
</style>
