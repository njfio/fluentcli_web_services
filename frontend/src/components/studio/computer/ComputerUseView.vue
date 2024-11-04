<template>
    <div class="flex h-full">
        <!-- Left side: VNC viewer -->
        <div class="w-1/2 h-full bg-gray-900 border-r border-gray-700">
            <Suspense>
                <VncViewer :url="vncUrl" />
                <template #fallback>
                    <div class="flex items-center justify-center h-full">
                        <p class="text-gray-400">Loading VNC viewer...</p>
                    </div>
                </template>
            </Suspense>
        </div>

        <!-- Right side: Chat interface -->
        <div class="w-1/2 h-full flex flex-col">
            <ComputerUseChat />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ComputerUseChat from './ComputerUseChat.vue'
import VncViewer from './VncViewer.vue'

const vncUrl = ref('ws://localhost:5901')
</script>
