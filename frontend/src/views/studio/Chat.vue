<template>
    <div class="chat-container">
        <h1 class="text-2xl font-bold mb-4">AI Chat</h1>
        <div class="chat-messages" ref="chatMessages">
            <div v-for="(message, index) in messages" :key="index" :class="['message', message.role]">
                <div class="message-content" v-html="renderMarkdown(message.content)"></div>
            </div>
        </div>
        <div class="chat-input">
            <textarea v-model="userInput" @keydown.enter.exact.prevent="sendMessage()"
                @keydown.enter.shift.exact="newline" placeholder="Type your message here... (Shift+Enter for new line)"
                rows="3" class="w-full p-2 border rounded-md resize-none" :disabled="isLoading"></textarea>
            <div class="flex justify-between items-center mt-2">
                <span v-if="isLoading" class="text-gray-600">
                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-primary-500 inline-block"
                        xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                        </circle>
                        <path class="opacity-75" fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                        </path>
                    </svg>
                    AI is thinking...
                </span>
                <button @click="sendMessage()" :disabled="isLoading || userInput.trim() === ''"
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
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, nextTick, onUnmounted } from 'vue';
import AuthService from '@/services/AuthService';

interface ChatMessage {
    role: 'user' | 'assistant';
    content: string;
}

export default defineComponent({
    name: 'Chat',
    setup() {
        const messages = ref<ChatMessage[]>([]);
        const userInput = ref('');
        const chatMessages = ref<HTMLElement | null>(null);
        const isLoading = ref(false);
        const error = ref('');
        let abortController: AbortController | null = null;
        let retryCount = 0;
        const maxRetries = 3;

        const sendMessage = async () => {
            if (userInput.value.trim() === '' || isLoading.value) return;
            await processMessage(userInput.value);
        };

        const processMessage = async (message: string, retry = false) => {
            if (!retry) {
                const userMessage: ChatMessage = {
                    role: 'user',
                    content: message,
                };
                messages.value.push(userMessage);
                userInput.value = '';
            }

            error.value = '';
            isLoading.value = true;

            scrollToBottom();

            try {
                // Abort previous request if it exists
                if (abortController) {
                    abortController.abort();
                }

                abortController = new AbortController();

                const token = AuthService.getToken();
                if (!token) {
                    throw new Error('No authentication token found');
                }

                const response = await fetch(`/api/chat/stream?content=${encodeURIComponent(message)}`, {
                    headers: {
                        'Authorization': `Bearer ${token}`,
                    },
                    signal: abortController.signal,
                });

                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }

                const reader = response.body?.getReader();
                const decoder = new TextDecoder();

                let assistantMessage: ChatMessage = {
                    role: 'assistant',
                    content: '',
                };

                while (true) {
                    const { done, value } = await reader!.read();
                    if (done) break;

                    const chunk = decoder.decode(value);
                    const lines = chunk.split('\n');
                    for (const line of lines) {
                        if (line.startsWith('data: ')) {
                            const data = JSON.parse(line.slice(6));
                            if (assistantMessage.content === '') {
                                messages.value.push(assistantMessage);
                            }
                            assistantMessage.content += data.message;
                            scrollToBottom();
                        }
                    }
                }

                isLoading.value = false;
                retryCount = 0;

            } catch (err: any) {
                console.error('Error sending message:', err);
                handleError('Failed to send message. Please try again.');
            }
        };

        const handleError = (errorMessage: string) => {
            if (retryCount < maxRetries) {
                retryCount++;
                setTimeout(() => processMessage(messages.value[messages.value.length - 1].content, true), 1000 * retryCount);
            } else {
                error.value = errorMessage;
                isLoading.value = false;
                retryCount = 0;
            }
        };

        const retryLastMessage = () => {
            error.value = '';
            processMessage(messages.value[messages.value.length - 1].content, true);
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
            // This is a very basic markdown renderer. You might want to use a more robust solution in production.
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

        onMounted(() => {
            scrollToBottom();
        });

        onUnmounted(() => {
            if (abortController) {
                abortController.abort();
            }
        });

        return {
            messages,
            userInput,
            sendMessage,
            chatMessages,
            isLoading,
            error,
            newline,
            renderMarkdown,
            retryLastMessage,
        };
    },
});
</script>

<style scoped>
.chat-container {
    @apply max-w-4xl mx-auto p-4;
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

/* Add some basic styling for rendered markdown */
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