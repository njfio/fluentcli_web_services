<template>
    <div class="flex flex-col h-full">
        <!-- Chat messages -->
        <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="chatContainer">
            <div v-for="(message, index) in messages" :key="index" class="message">
                <div :class="[
                    'p-3 rounded-lg max-w-3xl',
                    message.role === 'user' ? 'bg-blue-600 ml-auto text-white' : 'bg-gray-700 text-white'
                ]">
                    <div class="whitespace-pre-wrap break-words" v-html="message.renderedContent"></div>
                </div>
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
import { ref, nextTick } from 'vue'
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

const sendMessage = async () => {
    if (!currentMessage.value.trim() || loading.value) return

    const message = currentMessage.value
    loading.value = true

    // Add user message
    messages.value.push({
        role: 'user',
        content: message,
        renderedContent: renderMarkdown(message)
    })
    currentMessage.value = ''
    await scrollToBottom()

    try {
        // Get response stream from computer use service
        const stream = await ComputerUseService.chat(messages.value)
        const reader = stream.getReader()
        const decoder = new TextDecoder()

        let assistantMessage: Message = {
            role: 'assistant',
            content: '',
            renderedContent: ''
        }
        messages.value.push(assistantMessage)

        while (true) {
            const { done, value } = await reader.read()
            if (done) break

            const chunk = decoder.decode(value)
            assistantMessage.content += chunk
            assistantMessage.renderedContent = renderMarkdown(assistantMessage.content)
            await scrollToBottom()
        }

        // Update the last message with complete response
        messages.value[messages.value.length - 1] = assistantMessage
    } catch (error) {
        console.error('Error in computer use chat:', error)
        messages.value.push({
            role: 'assistant',
            content: 'An error occurred while processing your message. Please try again.',
            renderedContent: 'An error occurred while processing your message. Please try again.'
        })
    } finally {
        loading.value = false
        await scrollToBottom()
    }
}
</script>
