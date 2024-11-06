<template>
    <div class="flex flex-col max-h-[calc(100vh-4rem)]"> <!-- 4rem for header -->
        <div class="flex flex-1 min-h-0"> <!-- min-h-0 prevents flex child overflow -->
            <!-- Left section - VNC Viewer and File Browser -->
            <div class="w-2/3 h-full flex flex-col">
                <!-- VNC Viewer - 60% height -->
                <div class="h-3/5 bg-gray-900">
                    <VncViewer :url="vncUrl" />
                </div>

                <!-- File Browser - 40% height -->
                <div class="h-2/5 bg-gray-900 border-t border-gray-700">
                    <FileBrowser />
                </div>
            </div>

            <!-- Chat section - 33% width -->
            <div class="w-1/3 h-full bg-gray-800">
                <ComputerUseChat />
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
