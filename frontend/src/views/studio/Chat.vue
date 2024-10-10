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
                    class="mt-4 px-4 py-2 bg-primary-600 text-white rounded-md hover:bg-primary-700">
                    New Conversation
                </button>
            </div>
            <div class="chat-content w-3/4">
                <div v-if="currentConversation" class="chat-messages" ref="chatMessages">
                    <div v-for="(message, index) in currentMessages" :key="index" :class="['message', message.role]">
                        <div class="message-content" v-html="renderMarkdown(message.content)"></div>
                    </div>
                </div>
                <div v-if="currentConversation" class="chat-input">
                    <div class="mb-2">
                        <label for="provider-select" class="block text-sm font-medium text-gray-700">Select LLM
                            Provider:</label>
                        <select id="provider-select" v-model="selectedProviderId"
                            class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm rounded-md">
                            <option v-for="provider in llmProviders" :key="provider.id" :value="provider.id">
                                {{ provider.name }}
                            </option>
                        </select>
                    </div>
                    <textarea v-model="userInput" @keydown.enter.exact.prevent="sendMessage()"
                        @keydown.enter.shift.exact="newline"
                        placeholder="Type your message here... (Shift+Enter for new line)" rows="3"
                        class="w-full p-2 border rounded-md resize-none" :disabled="isLoading"></textarea>
                    <div class="flex justify-between items-center mt-2">
                        <span v-if="isLoading" class="text-gray-600">
                            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-primary-500 inline-block"
                                xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor"
                                    stroke-width="4">
                                </circle>
                                <path class="opacity-75" fill="currentColor"
                                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                                </path>
                            </svg>
                            AI is thinking...
                        </span>
                        <button @click="sendMessage()"
                            :disabled="isLoading || userInput.trim() === '' || !selectedProviderId"
                            class="px-4 py-2 bg-primary-600 text-white rounded-md hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed">
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
import { defineComponent, ref, computed, onMounted, nextTick, onUnmounted, watch } from 'vue';
import { useStore } from 'vuex';
import LLMService, { LLMProvider, LLMMessage } from '../../services/LLMService';
import { Message } from '../../store/modules/chat';

export default defineComponent({
    name: 'Chat',
    setup() {
        const store = useStore();
        const userInput = ref('');
        const chatMessages = ref<HTMLElement | null>(null);
        const isLoading = ref(false);
        const error = ref('');
        let abortController: AbortController | null = null;
        let retryCount = 0;
        const maxRetries = 3;

        const conversations = computed(() => store.state.chat.conversations);
        const currentConversation = computed(() => store.state.chat.currentConversation);
        const messages = computed(() => store.state.chat.messages);
        const currentMessages = ref<Message[]>([]);

        const llmProviders = ref<LLMProvider[]>([]);
        const selectedProviderId = ref<string>('');

        onMounted(async () => {
            await store.dispatch('chat/getConversations');
            scrollToBottom();
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

        const selectConversation = async (conversationId: string) => {
            await store.dispatch('chat/getConversation', conversationId);
            await store.dispatch('chat/getMessages', conversationId);
            currentMessages.value = messages.value.filter((m: Message | null): m is Message => m !== null);
        };

        const createNewConversation = async () => {
            const title = prompt('Enter conversation title:');
            if (title) {
                const newConversation = await store.dispatch('chat/createConversation', title);
                await selectConversation(newConversation.id);
            }
        };

        const sendMessage = async () => {
            if (userInput.value.trim() === '' || isLoading.value || !currentConversation.value || !selectedProviderId.value) return;
            await processMessage(userInput.value);
        };

        const processMessage = async (message: string, retry = false) => {
            if (!selectedProviderId.value) {
                error.value = 'Please select an LLM provider before sending a message.';
                return;
            }

            let userMessage: Message | null = null;
            if (!retry && currentConversation.value) {
                userMessage = await store.dispatch('chat/createMessage', {
                    conversationId: currentConversation.value.id,
                    role: 'user',
                    content: message,
                });
                if (userMessage) {
                    currentMessages.value.push(userMessage);
                }
                userInput.value = '';
            }

            error.value = '';
            isLoading.value = true;

            scrollToBottom();

            try {
                if (abortController) {
                    abortController.abort();
                }

                abortController = new AbortController();

                const llmMessages: LLMMessage[] = [
                    ...currentMessages.value.map((m: Message) => ({
                        role: m.role as 'system' | 'user' | 'assistant',
                        content: m.content,
                    })),
                    {
                        role: 'user',
                        content: message,
                    },
                ];

                console.log('Current Messages:', JSON.stringify(currentMessages.value, null, 2));
                console.log('LLM Messages:', JSON.stringify(llmMessages, null, 2));
                console.log('Selected Provider ID:', selectedProviderId.value);

                const stream = await LLMService.streamChat(selectedProviderId.value, llmMessages);
                const reader = stream.getReader();
                const decoder = new TextDecoder();

                let assistantMessage: Message | null = null;

                while (true) {
                    const { done, value } = await reader.read();
                    if (done) break;

                    const chunk = decoder.decode(value);
                    const lines = chunk.split('\n');

                    for (const line of lines) {
                        if (line.startsWith('data: ')) {
                            const jsonStr = line.slice(6).trim();
                            if (jsonStr === '[DONE]') {
                                console.log('Stream completed');
                                continue;
                            }
                            try {
                                const parsedChunk = JSON.parse(jsonStr);
                                console.log('Parsed chunk:', parsedChunk);
                                if (parsedChunk.choices && parsedChunk.choices[0].delta.content) {
                                    if (!assistantMessage) {
                                        assistantMessage = await store.dispatch('chat/createMessage', {
                                            conversationId: currentConversation.value!.id,
                                            role: 'assistant',
                                            content: '',
                                        });
                                        if (assistantMessage) {
                                            currentMessages.value.push(assistantMessage);
                                        }
                                    }
                                    if (assistantMessage) {
                                        assistantMessage.content += parsedChunk.choices[0].delta.content;
                                        await store.dispatch('chat/updateMessage', assistantMessage);
                                        currentMessages.value[currentMessages.value.length - 1] = { ...assistantMessage };
                                        scrollToBottom();
                                    }
                                }
                            } catch (e) {
                                console.error('Error parsing chunk:', e, 'Raw data:', jsonStr);
                            }
                        }
                    }
                }

                isLoading.value = false;
                retryCount = 0;

            } catch (err: any) {
                console.error('Error sending message:', err);
                handleError(err);
            }
        };

        const handleError = (err: any) => {
            if (retryCount < maxRetries) {
                retryCount++;
                setTimeout(() => processMessage(currentMessages.value[currentMessages.value.length - 1].content, true), 1000 * retryCount);
            } else {
                if (err.message === 'Failed to fetch') {
                    error.value = 'Network error. Please check your internet connection and try again.';
                } else if (err.response && err.response.status === 401) {
                    error.value = 'Authentication error. Please log in again.';
                } else {
                    error.value = `Error: ${err.message || 'Unknown error occurred'}. Please try again.`;
                }
                isLoading.value = false;
                retryCount = 0;
            }
        };

        const retryLastMessage = () => {
            error.value = '';
            const lastUserMessage = currentMessages.value.filter((m: Message) => m.role === 'user').pop();
            if (lastUserMessage) {
                processMessage(lastUserMessage.content, true);
            }
        };

        const scrollToBottom = () => {
            nextTick(() => {
                if (chatMessages.value) {
                    chatMessages.value.scrollTop = chatMessages.value.scrollHeight;
                }
            });
        };

        const newline = () => {
            userInput.value += '\n';
        };

        const renderMarkdown = (text: string): string => {
            if (!text) return '';
            return text
                .replace(/^# (.*$)/gim, '<h1>$1</h1>')
                .replace(/^## (.*$)/gim, '<h2>$1</h2>')
                .replace(/^### (.*$)/gim, '<h3>$1</h3>')
                .replace(/\*\*(.*)\*\*/gim, '<strong>$1</strong>')
                .replace(/\*(.*)\*/gim, '<em>$1</em>')
                .replace(/```([\s\S]*?)```/g, '<pre><code>$1</code></pre>')
                .replace(/- (.*)/gim, '<li>$1</li>')
                .replace(/\n/gim, '<br>');
        };

        watch(currentConversation, async (newConversation) => {
            if (newConversation) {
                await store.dispatch('chat/getMessages', newConversation.id);
                currentMessages.value = messages.value.filter((m: Message | null): m is Message => m !== null);
            }
        });

        onUnmounted(() => {
            if (abortController) {
                abortController.abort();
            }
        });

        return {
            conversations,
            currentConversation,
            currentMessages,
            userInput,
            sendMessage,
            chatMessages,
            isLoading,
            error,
            newline,
            renderMarkdown,
            retryLastMessage,
            selectConversation,
            createNewConversation,
            llmProviders,
            selectedProviderId,
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
    @apply bg-primary-100 text-primary-800;
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
