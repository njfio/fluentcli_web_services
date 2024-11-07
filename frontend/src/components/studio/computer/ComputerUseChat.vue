<template>
    <div class="flex flex-col h-full">
        <!-- Chat messages -->
        <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="chatContainer">
            <div v-for="(message, index) in messages" :key="index" class="message">
                <div :class="[
                    'p-3 rounded-lg max-w-3xl',
                    message.role === 'user' ? 'bg-blue-600 ml-auto text-white' : 'bg-gray-700 text-white'
                ]">
                    <div class="whitespace-pre-wrap break-words">
                        {{ getTextContent(message.content) }}
                    </div>
                    <div v-if="getImageData(message.content)" class="mt-2">
                        <img :src="getImageData(message.content)" alt="Screenshot" class="max-w-full rounded-lg" />
                    </div>
                    <div v-if="getErrorContent(message.content)" class="mt-2 text-red-400">
                        {{ getErrorContent(message.content) }}
                    </div>
                </div>
            </div>
            <!-- Error message -->
            <div v-if="error" class="p-3 rounded-lg bg-red-600 text-white">
                {{ error }}
            </div>
        </div>

        <!-- Message input -->
        <div class="border-t border-gray-700 p-4">
            <div class="flex space-x-2">
                <input v-model="currentMessage" @keyup.enter="sendMessage" type="text" placeholder="Enter a message..."
                    class="flex-1 bg-gray-800 text-white px-4 py-2 rounded-lg border border-gray-600 focus:outline-none focus:border-blue-500"
                    :disabled="loading" />
                <button @click="sendMessage"
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none disabled:opacity-50"
                    :disabled="loading">
                    {{ loading ? 'Sending...' : 'Send' }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import ComputerUseService, { ComputerUseMessage } from '../../../services/ComputerUseService'
import { marked } from 'marked'
import DOMPurify from 'dompurify'

interface Message extends ComputerUseMessage {
    renderedContent: string;
}

const messages = ref<Message[]>([{
    role: 'assistant',
    content: 'Welcome to Computer Use. I can help you interact with the computer system.',
    renderedContent: 'Welcome to Computer Use. I can help you interact with the computer system.'
}])
const currentMessage = ref('')
const chatContainer = ref<HTMLElement | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

const getTextContent = (content: string): string => {
    if (!content) return '';

    // Find the last occurrence of a JSON object
    const jsonStart = content.lastIndexOf('{');
    if (jsonStart === -1) return content;

    // Return everything before the JSON
    return content.substring(0, jsonStart).trim();
}

const getImageData = (content: string): string | undefined => {
    try {
        // Find the last occurrence of a JSON object
        const jsonStart = content.lastIndexOf('{');
        if (jsonStart === -1) return undefined;

        // Parse the JSON portion
        const jsonStr = content.substring(jsonStart);
        const data = JSON.parse(jsonStr);

        if (data.image && typeof data.image === 'string') {
            return data.image;
        }
        return undefined;
    } catch (e) {
        return undefined;
    }
}

const getErrorContent = (content: string): string | undefined => {
    try {
        if (!content.includes('Error:')) return undefined;

        const errorStart = content.indexOf('Error:');
        let errorContent = content.substring(errorStart);

        // Try to parse as JSON if it contains JSON
        const jsonStart = errorContent.indexOf('{');
        if (jsonStart !== -1) {
            try {
                const jsonStr = errorContent.substring(jsonStart);
                const data = JSON.parse(jsonStr);
                if (data.error) {
                    return `Error: ${data.error}`;
                }
            } catch (e) {
                // If JSON parsing fails, return the raw error
            }
        }

        return errorContent;
    } catch (e) {
        return undefined;
    }
}

const hasImage = (message: Message): boolean => {
    return getImageData(message.content) !== undefined;
}

const getRecentMessages = (allMessages: Message[]): Message[] => {
    const result: Message[] = [];
    let imageCount = 0;

    // Start from the most recent messages
    for (let i = allMessages.length - 1; i >= 0; i--) {
        const message = allMessages[i];

        // If message has image, increment counter
        if (hasImage(message)) {
            imageCount++;
            // Only include up to 1 message with image to prevent token limit
            if (imageCount > 1) {
                continue;
            }
        }

        // Add message to result
        result.unshift(message);

        // Keep only last 6 messages without images
        if (result.length >= 6 && !hasImage(message)) {
            break;
        }
    }

    return result;
}

const scrollToBottom = async () => {
    await nextTick()
    if (chatContainer.value) {
        chatContainer.value.scrollTop = chatContainer.value.scrollHeight
    }
}

const renderMarkdown = (text: string): string => {
    const rawMarkup = marked(text)
    return DOMPurify.sanitize(rawMarkup)
}

const clearError = () => {
    error.value = null;
}

const sendMessage = async () => {
    if (!currentMessage.value.trim() || loading.value) return;

    const message = currentMessage.value;
    loading.value = true;
    clearError();

    // Add user message
    messages.value.push({
        role: 'user',
        content: message,
        renderedContent: renderMarkdown(message)
    });
    currentMessage.value = '';
    await scrollToBottom();

    try {
        // Get recent messages to prevent token limit
        const recentMessages = getRecentMessages(messages.value);

        let assistantMessage: Message = {
            role: 'assistant',
            content: '',
            renderedContent: ''
        };
        messages.value.push(assistantMessage);

        const response = await ComputerUseService.chat(recentMessages);

        // Handle the response text directly
        assistantMessage.content = response;
        assistantMessage.renderedContent = renderMarkdown(assistantMessage.content);
        await scrollToBottom();

        // If we detect an error in the response, handle it
        if (getErrorContent(assistantMessage.content)) {
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    } catch (error: any) {
        console.error('Error in computer use chat:', error);
        const errorMessage = error?.response?.data?.message || error?.message || 'An error occurred while processing your message. Please try again.';
        error.value = errorMessage;

        // Remove the last message if it was empty
        if (messages.value[messages.value.length - 1].content === '') {
            messages.value.pop();
        }
    } finally {
        loading.value = false;
        await scrollToBottom();
    }
}
</script>
