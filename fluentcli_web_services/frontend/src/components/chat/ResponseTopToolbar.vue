<template>
    <div
        class="flex items-center justify-between px-4 py-2 text-xs text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600">
        <div class="flex items-center space-x-2">
            <span class="inline-flex items-center">
                <svg class="w-4 h-4 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M9 6H20M9 12H20M9 18H20M5 6V6.01M5 12V12.01M5 18V18.01" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                Model:
            </span>
            <span class="font-medium text-blue-600 dark:text-blue-400">{{ providerModel }}</span>
        </div>
        <div class="flex items-center space-x-2">
            <button
                class="text-gray-900 hover:text-gray-700 dark:text-white dark:hover:text-gray-300 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 rounded p-1"
                :title="copyButtonTitle" @click="copyMessageContent"
                :class="{ 'text-green-600 dark:text-green-500': copied }">
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M8 5H6C4.89543 5 4 5.89543 4 7V19C4 20.1046 4.89543 21 6 21H16C17.1046 21 18 20.1046 18 19V17M8 5C8 6.10457 8.89543 7 10 7H12C13.1046 7 14 6.10457 14 5M8 5C8 3.89543 8.89543 3 10 3H12C13.1046 3 14 3.89543 14 5M14 5H16C17.1046 5 18 5.89543 18 7V10"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
    providerModel: string
}>()

const copyButtonTitle = ref('Copy message content')
const copied = ref(false)

const copyMessageContent = async () => {
    try {
        // Find the closest message container and get its content
        const messageElement = document.activeElement?.closest('.message')
        if (!messageElement) {
            throw new Error('Message content not found')
        }

        const contentElement = messageElement.querySelector('.message-content')
        if (!contentElement) {
            throw new Error('Message content not found')
        }

        // Get the text content, removing HTML tags
        const textContent = contentElement.textContent || ''
        await navigator.clipboard.writeText(textContent)

        // Show success feedback
        copied.value = true
        copyButtonTitle.value = 'Copied!'
        setTimeout(() => {
            copyButtonTitle.value = 'Copy message content'
            copied.value = false
        }, 2000)
    } catch (err) {
        console.error('Failed to copy message content:', err)
        copyButtonTitle.value = 'Failed to copy'
        setTimeout(() => {
            copyButtonTitle.value = 'Copy message content'
        }, 2000)
    }
}
</script>

<style scoped>
.space-x-2> :not([hidden])~ :not([hidden]) {
    --tw-space-x-reverse: 0;
    margin-right: calc(0.5rem * var(--tw-space-x-reverse));
    margin-left: calc(0.5rem * calc(1 - var(--tw-space-x-reverse)));
}

/* Smooth transitions */
.transition-colors {
    transition-property: background-color, border-color, color, fill, stroke;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 200ms;
}

/* Focus styles */
.focus\:outline-none:focus {
    outline: 2px solid transparent;
    outline-offset: 2px;
}

.focus\:ring-2:focus {
    --tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);
    --tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);
    box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000);
}
</style>
