<template>
    <div ref="container" class="vnc-container">
        <div id="vnc-display" ref="vncDisplay"></div>
        <div v-if="!connected" class="status-overlay">
            <p class="text-gray-400">{{ error || 'Connecting to virtual display...' }}</p>
            <button v-if="error" @click="reconnect"
                class="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
                Try Again
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import RFB from '@novnc/novnc/lib/rfb'

const props = defineProps<{
    url: string
}>()

const container = ref<HTMLElement | null>(null)
const vncDisplay = ref<HTMLElement | null>(null)
const connected = ref(false)
const error = ref('')
let rfb: InstanceType<typeof RFB> | null = null

const initVNC = async () => {
    if (!vncDisplay.value) {
        console.error('VNC display element not found')
        error.value = 'Display initialization failed'
        return
    }

    try {
        console.log('Initializing VNC connection to:', props.url)

        // Clean up existing connection if any
        if (rfb) {
            console.log('Cleaning up existing VNC connection')
            rfb.disconnect()
            rfb = null
        }

        rfb = new RFB(vncDisplay.value, props.url, {
            wsProtocols: ['binary'],
            credentials: { password: '' },
            scaleViewport: true,
            clipViewport: true
        })

        rfb.addEventListener('connect', () => {
            console.log('VNC connected successfully')
            connected.value = true
            error.value = ''
            // Focus the VNC display for input
            vncDisplay.value?.focus()
        })

        rfb.addEventListener('disconnect', (e: any) => {
            console.log('VNC disconnected:', e)
            connected.value = false
            error.value = e ? `Disconnected: ${e}` : 'Disconnected from display'
        })

        rfb.addEventListener('credentialsrequired', () => {
            console.log('VNC credentials required')
            error.value = 'VNC credentials required'
        })

        rfb.addEventListener('securityfailure', (e: any) => {
            console.error('VNC security failure:', e)
            error.value = 'Security failure'
        })

        rfb.addEventListener('error', (e: any) => {
            console.error('VNC error:', e)
            error.value = e ? `Error: ${e}` : 'Connection error'
        })

    } catch (err) {
        console.error('Failed to initialize VNC:', err)
        error.value = err instanceof Error ? err.message : 'Failed to connect to display'
    }
}

const reconnect = () => {
    console.log('Attempting to reconnect VNC')
    error.value = ''
    connected.value = false
    initVNC()
}

onMounted(() => {
    console.log('VNC viewer component mounted')
    if (props.url) {
        initVNC()
    } else {
        console.error('No VNC URL provided')
        error.value = 'No VNC URL provided'
    }
})

onUnmounted(() => {
    console.log('VNC viewer component unmounting')
    if (rfb) {
        console.log('Disconnecting VNC')
        rfb.disconnect()
        rfb = null
    }
})
</script>

<style scoped>
.vnc-container {
    background: #1a1a1a;
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

#vnc-display {
    width: 100%;
    height: 100%;
    outline: none;
}

#vnc-display:focus {
    outline: none;
}

.status-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.75);
}
</style>
