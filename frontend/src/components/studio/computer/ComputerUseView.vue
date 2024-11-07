<template>
    <div class="flex flex-col h-[calc(100vh-4rem)] overflow-hidden"> <!-- 4rem for header -->
        <!-- Main content area -->
        <div class="flex flex-col h-full">
            <!-- VNC Viewer container -->
            <div class="flex-1 bg-gray-900 flex items-end">
                <div class="w-full h-[62vh]">
                    <VncViewer :url="vncUrl" />
                </div>
            </div>

            <!-- Spacing between VNC and bottom section -->
            <div class="h-2"></div>

            <!-- Bottom section with File Browser and Chat side by side -->
            <div class="flex space-x-2 h-[calc(100vh-62vh-6rem)]"> <!-- Adjusted for VNC height -->
                <!-- File Browser with overflow handling -->
                <div class="w-1/2 bg-gray-900 border-t border-gray-700 overflow-auto">
                    <FileBrowser />
                </div>

                <!-- Chat section -->
                <div class="w-1/2 bg-gray-800 overflow-auto">
                    <ComputerUseChat />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import VncViewer from './VncViewer.vue'
import FileBrowser from './FileBrowser.vue'
import ComputerUseChat from './ComputerUseChat.vue'

// Use direct VNC port
const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
const wsHost = window.location.hostname
const wsPort = '5901'
const vncUrl = `${wsProtocol}//${wsHost}:${wsPort}`
</script>
