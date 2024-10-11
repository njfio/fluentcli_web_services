<template>
    <div class="chat-container flex h-full bg-gray-100 dark:bg-gray-900">
        <!-- Sidebar -->
        <div class="w-1/4 bg-white dark:bg-gray-800 shadow-md p-4 overflow-y-auto">
            <h2 class="text-xl font-bold mb-4 text-gray-800 dark:text-gray-200">Conversations</h2>
            <ul class="space-y-2">
                <li v-for="conversation in conversations" :key="conversation.id"
                    @click="selectConversation(conversation.id)"
                    :class="{ 'bg-blue-100 dark:bg-blue-900': currentConversation && conversation.id === currentConversation.id }"
                    class="cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 p-2 rounded-lg transition duration-150 ease-in-out">
                    <div class="flex items-center">
                        <svg class="w-4 h-4 mr-2 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor"
                            viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z">
                            </path>
                        </svg>
                        <span class="text-sm text-gray-700 dark:text-gray-300">{{ conversation.title }}</span>
                    </div>
                </li>
            </ul>
            <button @click="createNewConversation"
                class="mt-4 w-full px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 transition duration-150 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50">
                New Conversation
            </button>
        </div>

        <!-- Chat Area -->
        <div class="flex-1 flex flex-col relative">
            <!-- Chat Messages -->
            <div class="flex-1 overflow-y-auto p-4 pb-32" ref="chatMessages">
                <div v-if="currentConversation && currentMessages.length > 0">
                    <div v-for="(message, index) in currentMessages" :key="index"
                        :class="['message mb-3 p-3 rounded-lg max-w-3xl',
                            message.role === 'user' ? 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 ml-auto' : 'bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-200 mr-auto shadow-md']">
                        <div class="message-content text-sm" v-html="message.renderedContent || message.content">
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

            <!-- Floating Input Area -->
            <div class="fixed bottom-0 left-1/4 right-0 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 transition-all duration-300 ease-in-out"
                :class="{ 'h-44': isExpanded, 'h-auto': !isExpanded }">
                <!-- AI Thinking Indicator -->
                <div v-if="isLoading"
                    class="absolute top-0 left-0 right-0 -translate-y-full bg-white dark:bg-gray-800 p-2 text-xs text-gray-600 dark:text-gray-400 flex items-center justify-center border-t border-gray-200 dark:border-gray-700">
                    <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-blue-500" xmlns="http://www.w3.org/2000/svg"
                        fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                        </circle>
                        <path class="opacity-75" fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                        </path>
                    </svg>
                    AI is thinking...
                </div>

                <div class="p-3">
                    <div class="mb-2 flex justify-between items-center">
                        <label for="provider-select"
                            class="block text-xs font-medium text-gray-700 dark:text-gray-300">Select LLM
                            Provider:</label>
                        <button @click="toggleExpand" class="text-blue-600 dark:text-blue-400 text-sm">
                            {{ isExpanded ? 'Collapse' : 'Expand' }}
                        </button>
                    </div>
                    <select id="provider-select" v-model="selectedProviderId"
                        class="w-full p-1 text-sm border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 mb-2">
                        <option v-for="provider in llmProviders" :key="provider.id" :value="provider.id">
                            {{ provider.name }}
                        </option>
                    </select>
                    <div class="flex items-start">
                        <textarea v-model="userInput" @keydown.enter.exact.prevent="sendMessage"
                            @keydown.enter.shift.exact="newline"
                            placeholder="Type your message here... (Shift+Enter for new line)"
                            :rows="isExpanded ? 4 : 1"
                            class="flex-1 p-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
                            :disabled="isLoading || !currentConversation"></textarea>
                        <button @click="sendMessage"
                            :disabled="isLoading || userInput.trim() === '' || !selectedProviderId || !currentConversation"
                            class="ml-3 px-4 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 disabled:opacity-50 disabled:cursor-not-allowed transition duration-150 ease-in-out">
                            Send
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, nextTick, ref } from 'vue';
import { useChatLogic } from '../../components/chat/ChatLogic';
import LLMService from '../../services/LLMService';

export default defineComponent({
    name: 'Chat',
    setup() {
        const {
            userInput,
            isLoading,
            error,
            conversations,
            currentConversation,
            currentMessages,
            llmProviders,
            selectedProviderId,
            loadMessages,
            selectConversation,
            createNewConversation,
            sendMessage,
            retryLastMessage,
            newline,
            scrollToBottom,
        } = useChatLogic();

        const chatMessagesRef = ref<HTMLElement | null>(null);
        const isExpanded = ref(false);

        const toggleExpand = () => {
            isExpanded.value = !isExpanded.value;
        };

        onMounted(async () => {
            try {
                llmProviders.value = await LLMService.getProviders();
                if (llmProviders.value.length > 0) {
                    selectedProviderId.value = llmProviders.value[0].id;
                } else {
                    error.value = 'No LLM providers available. Please contact the administrator.';
                }
            } catch (err) {
                console.error('Error fetching LLM providers:', err);
                error.value = 'Failed to fetch LLM providers. Please try again later.';
            }
        });

        watch(currentConversation, async (newConversation) => {
            if (newConversation) {
                await loadMessages(newConversation.id);
                nextTick(() => {
                    scrollToBottom();
                });
            }
        });

        watch(currentMessages, () => {
            nextTick(() => {
                scrollToBottom();
            });
        });

        return {
            userInput,
            chatMessages: chatMessagesRef,
            isLoading,
            error,
            conversations,
            currentConversation,
            currentMessages,
            llmProviders,
            selectedProviderId,
            selectConversation,
            createNewConversation,
            sendMessage,
            retryLastMessage,
            newline,
            scrollToBottom,
            isExpanded,
            toggleExpand,
        };
    },
});
</script>

<style scoped>
.chat-container {
    height: calc(100vh - 64px);
    /* Adjust this value based on your header height */
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

.message-content :deep(h1) {
    @apply text-lg font-bold mb-1;
}

.message-content :deep(h2) {
    @apply text-base font-bold mb-1;
}

.message-content :deep(h3) {
    @apply text-sm font-bold mb-1;
}

.message-content :deep(ul) {
    @apply list-disc list-inside mb-1;
}

.message-content :deep(ol) {
    @apply list-decimal list-inside mb-1;
}

.message-content :deep(pre) {
    @apply bg-gray-100 dark:bg-gray-700 p-2 rounded mb-1 overflow-x-auto;
}

.message-content :deep(code) {
    @apply font-mono text-xs bg-gray-100 dark:bg-gray-700 p-1 rounded;
}

.message-content :deep(p) {
    @apply mb-1;
}

.message-content :deep(a) {
    @apply text-blue-600 dark:text-blue-400 underline;
}

.message-content :deep(blockquote) {
    @apply border-l-4 border-gray-300 dark:border-gray-600 pl-2 italic my-1;
}

.message-content :deep(table) {
    @apply border-collapse border border-gray-300 dark:border-gray-600 my-1;
}

.message-content :deep(th),
.message-content :deep(td) {
    @apply border border-gray-300 dark:border-gray-600 p-1;
}

.message-content :deep(img) {
    @apply max-w-full h-auto my-1;
}
</style>
