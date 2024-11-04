<template>
    <div class="flex flex-col h-full">
        <!-- Chat messages -->
        <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="chatContainer">
            <div v-for="(message, index) in messages" :key="index" class="message">
                <div :class="[
                    'p-3 rounded-lg max-w-3xl',
                    message.role === 'user' ? 'bg-blue-600 ml-auto' : 'bg-gray-700'
                ]">
                    <pre class="whitespace-pre-wrap break-words text-white">{{ message.content }}</pre>
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

interface ChatMessage {
    role: 'user' | 'assistant'
    content: string
}

const messages = ref<ChatMessage[]>([{
    role: 'assistant',
    content: 'Welcome to Computer Use. I can help you interact with the computer system.'
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

const sendMessage = async () => {
    if (!currentMessage.value.trim() || loading.value) return

    const message = currentMessage.value
    loading.value = true

    messages.value.push({
        role: 'user',
        content: message
    })
    currentMessage.value = ''
    await scrollToBottom()

    // TODO: Send message to Anthropic computer provider
    // This will be integrated with the existing chat system

    loading.value = false
}
</script>

<style scoped>
.message pre {
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 0.9em;
}
</style>
