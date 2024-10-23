<template>
    <div
        class="flex items-center justify-end px-4 py-2 text-xs bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400">
        <div class="flex items-center space-x-3">
            <button class="toolbar-button group" @click="copyContent" title="Copy message">
                <svg class="w-4 h-4 transition-transform group-hover:scale-110" viewBox="0 0 24 24" fill="none"
                    xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M8 5H6C4.89543 5 4 5.89543 4 7V19C4 20.1046 4.89543 21 6 21H16C17.1046 21 18 20.1046 18 19V17M8 5C8 6.10457 8.89543 7 10 7H12C13.1046 7 14 6.10457 14 5M8 5C8 3.89543 8.89543 3 10 3H12C13.1046 3 14 3.89543 14 5M14 5H16C17.1046 5 18 5.89543 18 7V10"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                <span class="tooltip">Copy</span>
            </button>
            <button class="toolbar-button group" @click="regenerateResponse" title="Regenerate response">
                <svg class="w-4 h-4 transition-transform group-hover:rotate-180" viewBox="0 0 24 24" fill="none"
                    xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M4 4V9H4.58152M19.9381 11C19.446 7.05369 16.0796 4 12 4C8.64262 4 5.76829 6.06817 4.58152 9M4.58152 9H9M20 20V15H19.4185M19.4185 15C18.2317 17.9318 15.3574 20 12 20C7.92038 20 4.55399 16.9463 4.06189 13M19.4185 15H15"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                <span class="tooltip">Regenerate</span>
            </button>
            <button class="toolbar-button group" @click="$emit('deleteMessage', messageId)" title="Delete message">
                <svg class="w-4 h-4 transition-colors group-hover:text-red-500" viewBox="0 0 24 24" fill="none"
                    xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M19 7L18.1327 19.1425C18.0579 20.1891 17.187 21 16.1378 21H7.86224C6.81296 21 5.94208 20.1891 5.86732 19.1425L5 7M10 11V17M14 11V17M15 7V4C15 3.44772 14.5523 3 14 3H10C9.44772 3 9 3.44772 9 4V7M4 7H20"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                <span class="tooltip">Delete</span>
            </button>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'ResponseToolbar',
    props: {
        messageId: {
            type: String,
            required: true,
        },
    },
    emits: ['deleteMessage'],
    setup(props) {
        const copyContent = async () => {
            try {
                const messageElement = document.querySelector(`[data-message-id="${props.messageId}"] .message-content`);
                if (messageElement) {
                    const textContent = messageElement.textContent || '';
                    await navigator.clipboard.writeText(textContent);
                }
            } catch (err) {
                console.error('Failed to copy message:', err);
            }
        };

        const regenerateResponse = () => {
            // TODO: Implement regenerate functionality
            console.log('Regenerate response clicked');
        };

        return {
            copyContent,
            regenerateResponse,
        };
    },
});
</script>

<style scoped>
.toolbar-button {
    @apply p-1.5 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors duration-200 relative;
    @apply focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800;
}

.tooltip {
    @apply invisible absolute -top-8 left-1/2 -translate-x-1/2 px-2 py-1 rounded bg-gray-800 text-white text-xs;
    @apply group-hover:visible opacity-0 group-hover:opacity-100 transition-opacity duration-200;
}

.tooltip::after {
    content: '';
    @apply absolute -bottom-1 left-1/2 -translate-x-1/2 border-4 border-transparent border-t-gray-800;
}
</style>
