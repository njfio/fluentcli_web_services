<template>
    <div class="flex flex-col h-full">
        <!-- Chat messages -->
        <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="chatContainer">
            <div v-for="(message, index) in visibleMessages" :key="index" class="message">
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
                    <div v-if="getToolError(message.content)" class="mt-2 text-red-400">
                        {{ getToolError(message.content) }}
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
                <input v-model="currentMessage" @keyup.enter="handleAction" type="text" placeholder="Enter a message..."
                    class="flex-1 bg-gray-800 text-white px-4 py-2 rounded-lg border border-gray-600 focus:outline-none focus:border-blue-500"
                    :disabled="isProcessing" />
                <button @click="handleAction"
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none"
                    :class="{ 'bg-red-600 hover:bg-red-700': isProcessing }">
                    {{ isProcessing ? 'Cancel' : 'Send' }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onBeforeUnmount, computed } from 'vue'
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
const error = ref<string | null>(null)
const isProcessing = ref(false)
const currentAssistantMessage = ref<Message | null>(null)

// Only show user and assistant messages
const visibleMessages = computed(() =>
    messages.value.filter(msg => msg.role === 'user' || msg.role === 'assistant')
)

const getTextContent = (content: string): string => {
    if (!content) return '';

    // Remove <img> tags and their content
    content = content.replace(/<img>.*?<\/img>/g, '');
    // Remove <continue> tags and their content
    content = content.replace(/<continue>.*?<\/continue>/g, '');

    return content.trim();
}

const getImageData = (content: string): string | undefined => {
    const match = content.match(/<img>(.*?)<\/img>/);
    if (match) {
        return match[1];
    }
    return undefined;
}

const getToolError = (content: string): string | undefined => {
    const toolResult = ComputerUseService.parseToolResult(content);
    return ComputerUseService.getToolError(toolResult);
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

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const processStream = async (stream: ReadableStream<Uint8Array>): Promise<boolean> => {
    const reader = stream.getReader();
    let buffer = '';
    let shouldContinue = false;

    try {
        while (true) {
            const { done, value } = await reader.read();
            if (done) break;

            const text = decoder.decode(value);
            buffer += text;

            // Check if we have a complete message
            if (ComputerUseService.isToolResult(buffer)) {
                // Add the tool result to the current assistant message
                if (currentAssistantMessage.value) {
                    currentAssistantMessage.value.content += '\n\n' + buffer;
                    currentAssistantMessage.value.renderedContent = renderMarkdown(currentAssistantMessage.value.content);
                }
                shouldContinue = ComputerUseService.shouldContinue(buffer);
                break;
            } else if (currentAssistantMessage.value) {
                // Update the current assistant message
                currentAssistantMessage.value.content = buffer;
                currentAssistantMessage.value.renderedContent = renderMarkdown(buffer);
            } else {
                // Start a new assistant message
                currentAssistantMessage.value = {
                    role: 'assistant',
                    content: buffer,
                    renderedContent: renderMarkdown(buffer)
                };
                messages.value.push(currentAssistantMessage.value);
            }

            await scrollToBottom();
        }
    } catch (err: unknown) {
        const error = err as Error;
        if (error.name !== 'AbortError') {
            console.error('Error processing stream:', error);
            throw error;
        }
    } finally {
        reader.releaseLock();
    }

    return shouldContinue;
}

const startConversationLoop = async (userMsg: Message) => {
    if (isProcessing.value) return;
    isProcessing.value = true;
    currentAssistantMessage.value = null;

    try {
        let continueLoop = true;
        while (continueLoop && !error.value && isProcessing.value) {
            // Get messages up to the current point
            const apiMessages = messages.value.slice();
            if (currentAssistantMessage.value) {
                apiMessages.push(currentAssistantMessage.value);
            } else {
                apiMessages.push(userMsg);
            }

            const stream = await ComputerUseService.chat(apiMessages);
            continueLoop = await processStream(stream);

            if (continueLoop) {
                // Add continue message
                apiMessages.push({
                    role: 'user',
                    content: 'continue',
                    renderedContent: 'continue'
                });
                await delay(100);
            }
        }
    } catch (error: any) {
        console.error('Error in conversation loop:', error);
        throw error;
    } finally {
        isProcessing.value = false;
        currentAssistantMessage.value = null;
    }
}

const handleAction = async () => {
    if (isProcessing.value) {
        // Cancel the current conversation
        ComputerUseService.cancelRequest();
        isProcessing.value = false;
        currentAssistantMessage.value = null;
        return;
    }

    if (!currentMessage.value.trim()) return;

    clearError();

    try {
        // Create user message
        const userMsg: Message = {
            role: 'user',
            content: currentMessage.value,
            renderedContent: renderMarkdown(currentMessage.value)
        };

        // Add to display messages
        messages.value.push(userMsg);
        currentMessage.value = '';
        await scrollToBottom();

        // Start conversation loop with this user message
        await startConversationLoop(userMsg);
    } catch (error: any) {
        console.error('Error in computer use chat:', error);
        const errorMessage = error?.response?.data?.message || error?.message || 'An error occurred while processing your message. Please try again.';
        error.value = errorMessage;

        // Remove the last message if it was empty
        if (messages.value[messages.value.length - 1].content === '') {
            messages.value.pop();
        }
    }
}

onBeforeUnmount(() => {
    ComputerUseService.cancelRequest();
});

const decoder = new TextDecoder();
</script>
