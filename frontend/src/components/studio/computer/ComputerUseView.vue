<template>
    <div class="flex flex-col max-h-[calc(100vh-4rem)]"> <!-- 4rem for header -->
        <div class="flex flex-1 min-h-0"> <!-- min-h-0 prevents flex child overflow -->
            <!-- VNC Viewer section - 66% width -->
            <div class="w-2/3 h-full bg-gray-900">
                <VncViewer :url="vncUrl" />
            </div>

            <!-- Chat section - 33% width -->
            <div class="w-1/3 h-full bg-gray-800 flex flex-col">
                <div class="flex-1 p-4 overflow-y-auto">
                    <div class="bg-gray-700 rounded p-4 mb-4">
                        <p class="text-gray-300">Welcome to Computer Use. I can help you interact with the computer
                            system.</p>
                    </div>
                </div>
                <div class="p-4 border-t border-gray-700">
                    <div class="flex gap-2">
                        <input type="text" v-model="message" @keyup.enter="sendMessage" placeholder="Enter a message..."
                            class="flex-1 bg-gray-700 text-gray-200 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500">
                        <button @click="sendMessage"
                            class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500">
                            Send
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import VncViewer from './VncViewer.vue'

const message = ref('')
// Use direct VNC port
const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
const wsHost = window.location.hostname
const wsPort = '5901'
const vncUrl = `${wsProtocol}//${wsHost}:${wsPort}`

const sendMessage = () => {
    if (!message.value.trim()) return
    // Handle message sending
    message.value = ''
}
</script>
