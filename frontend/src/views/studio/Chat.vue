<template>
    <div
        class="chat-container flex h-full bg-gray-100 dark:bg-gray-900 overflow-y-auto flex-grow max-h-[calc(100vh-200px)]">
        <!-- Sidebar -->
        <div :class="['bg-gray-800 flex flex-col transition-all duration-300 ease-in-out max-h-screen',
            isSidebarOpen ? 'w-64' : 'w-16']">
            <!-- Toggle button -->
            <button @click="toggleSidebar" class="p-4 text-gray-300 hover:text-white focus:outline-none"
                :class="{ 'self-end': isSidebarOpen }">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                </svg>
            </button>

            <h2 class="text-xl font-bold mb-4 text-gray-200 px-4 " v-if="isSidebarOpen">Conversations</h2>
            <button @click="createNewConversation"
                class="mb-4 mx-4 px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 transition duration-150 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50">
                {{ isSidebarOpen ? 'New Conversation' : '+' }}
            </button>
            <div class="overflow-y-auto flex-grow max-h-[calc(100vh-200px)]">
                <ul class="space-y-2 px-2">
                    <li v-for="conversation in conversations" :key="conversation.id"
                        @click="selectConversation(conversation.id)"
                        :class="{ 'bg-gray-700': currentConversation && conversation.id === currentConversation.id }"
                        class="cursor-pointer hover:bg-gray-700 p-2 rounded-lg transition duration-150 ease-in-out flex items-center">
                        <svg class="w-4 h-4 mr-2 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z">
                            </path>
                        </svg>
                        <span v-if="isSidebarOpen" class="text-sm text-gray-300 truncate">{{ conversation.title
                            }}</span>
                    </li>
                </ul>
            </div>
        </div>

        <!-- Chat Area -->
        <div
            :class="['flex-1 flex flex-col relative transition-all duration-300 ease-in-out', isSidebarOpen ? 'ml-64' : 'ml-16']">
            <!-- Chat Messages -->
            <div class="flex-1 overflow-y-auto p-4 pb-32" ref="chatMessages">
                <div v-if="currentConversation && currentMessages.length > 0">
                    <div v-for="(message, index) in currentMessages" :key="index"
                        :class="['message mb-3 p-3 rounded-lg max-w-3xl',
                            message.role === 'user' ? 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 ml-auto' : 'bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-200 mr-auto shadow-md']">
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

            <!-- Floating Input Area -->
            <div class="fixed bottom-0 right-0 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 transition-all duration-300 ease-in-out"
                :class="{ 'h-44': isExpanded, 'h-auto': !isExpanded, 'left-64': isSidebarOpen, 'left-16': !isSidebarOpen }">
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
import { useStore } from 'vuex';

export default defineComponent({
    name: 'Chat',
    setup() {
        const store = useStore();
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
        } = useChatLogic();

        const chatMessagesRef = ref<HTMLElement | null>(null);
        const isExpanded = ref(false);
        const isSidebarOpen = ref(true);

        const toggleExpand = () => {
            isExpanded.value = !isExpanded.value;
        };

        const toggleSidebar = () => {
            isSidebarOpen.value = !isSidebarOpen.value;
        };

        const scrollToBottom = () => {
            if (chatMessagesRef.value) {
                chatMessagesRef.value.scrollTop = chatMessagesRef.value.scrollHeight;
            }
        };

        onMounted(async () => {
            try {
                console.log('Fetching conversations...');
                await store.dispatch('chat/getConversations');
                console.log('Conversations fetched:', conversations.value);

                llmProviders.value = await LLMService.getProviders();
                if (llmProviders.value.length > 0) {
                    selectedProviderId.value = llmProviders.value[0].id;
                } else {
                    error.value = 'No LLM providers available. Please contact the administrator.';
                }
            } catch (err) {
                console.error('Error in onMounted:', err);
                error.value = 'Failed to fetch conversations or LLM providers. Please try again later.';
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
        }, { deep: true });

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
            isSidebarOpen,
            toggleSidebar,
        };
    },
});</script>

<style scoped>
.chat-container {
    height: calc(100vh - 64px);
    /* Adjust this value based on your header height */
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

/* Add this new style for smoother sidebar transition */
.chat-container>div {
    transition: all 0.3s ease-in-out;
}

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
