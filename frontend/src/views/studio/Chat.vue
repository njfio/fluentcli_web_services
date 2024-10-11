<template>
    <div class="chat-container">
        <h1 class="text-2xl font-bold mb-4">AI Chat</h1>
        <div class="flex">
            <div class="conversation-list w-1/4 pr-4">
                <h2 class="text-xl font-bold mb-2">Conversations</h2>
                <ul>
                    <li v-for="conversation in conversations" :key="conversation.id"
                        @click="selectConversation(conversation.id)"
                        :class="{ 'font-bold': currentConversation && conversation.id === currentConversation.id }"
                        class="cursor-pointer hover:bg-gray-100 p-2 rounded">
                        {{ conversation.title }}
                    </li>
                </ul>
                <button @click="createNewConversation"
                    class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
                    New Conversation
                </button>
            </div>
            <div class="chat-content w-3/4">
                <div v-if="currentConversation" class="chat-messages" ref="chatMessages">
                    <div v-for="(message, index) in currentMessages" :key="index" :class="['message', message.role]">
                        <div class="message-content" v-html="message.renderedContent || message.content"></div>
                    </div>
                </div>
                <div v-if="currentConversation" class="chat-input">
                    <div class="mb-2">
                        <label for="provider-select" class="block text-sm font-medium text-gray-700">Select LLM
                            Provider:</label>
                        <select id="provider-select" v-model="selectedProviderId"
                            class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md">
                            <option v-for="provider in llmProviders" :key="provider.id" :value="provider.id">
                                {{ provider.name }}
                            </option>
                        </select>
                    </div>
                    <textarea v-model="userInput" @keydown.enter.exact.prevent="sendMessage"
                        @keydown.enter.shift.exact="newline"
                        placeholder="Type your message here... (Shift+Enter for new line)" rows="3"
                        class="w-full p-2 border rounded-md resize-none" :disabled="isLoading"></textarea>
                    <div class="flex justify-between items-center mt-2">
                        <span v-if="isLoading" class="text-gray-600">
                            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-blue-500 inline-block"
                                xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor"
                                    stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor"
                                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                                </path>
                            </svg>
                            AI is thinking...
                        </span>
                        <button @click="sendMessage"
                            :disabled="isLoading || userInput.trim() === '' || !selectedProviderId"
                            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed">
                            Send
                        </button>
                    </div>
                </div>
                <div v-if="error" class="mt-4 p-4 bg-red-100 text-red-700 rounded-md">
                    {{ error }}
                    <button @click="retryLastMessage" class="ml-2 underline">Retry</button>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, nextTick } from 'vue';
import { useChatLogic } from '../../components/chat/ChatLogic';
import LLMService from '../../services/LLMService';

export default defineComponent({
    name: 'Chat',
    setup() {
        const {
            userInput,
            chatMessages,
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
            chatMessages,
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
        };
    },
});
</script>

<style scoped>
.chat-container {
    @apply max-w-6xl mx-auto p-4;
}

.chat-messages {
    @apply h-[calc(100vh-300px)] overflow-y-auto mb-4 p-4 border rounded-md;
}

.message {
    @apply mb-4 p-3 rounded-lg;
}

.message.user {
    @apply bg-blue-100 text-blue-800;
}

.message.assistant {
    @apply bg-gray-100 text-gray-800;
}

.message-content {
    @apply break-words;
}

.chat-input {
    @apply mt-4;
}

.message-content :deep(h1) {
    @apply text-2xl font-bold mb-2;
}

.message-content :deep(h2) {
    @apply text-xl font-bold mb-2;
}

.message-content :deep(h3) {
    @apply text-lg font-bold mb-2;
}

.message-content :deep(ul) {
    @apply list-disc list-inside mb-2;
}

.message-content :deep(pre) {
    @apply bg-gray-100 p-2 rounded mb-2 overflow-x-auto;
}

.message-content :deep(code) {
    @apply font-mono text-sm;
}
</style>
