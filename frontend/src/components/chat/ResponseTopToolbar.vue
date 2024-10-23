<template>
    <div class="flex items-center justify-between px-4 py-2 text-xs text-gray-500 dark:text-gray-400">
        <div class="flex items-center space-x-2">
            <span class="inline-flex items-center">
                <svg class="w-4 h-4 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M9 6H20M9 12H20M9 18H20M5 6V6.01M5 12V12.01M5 18V18.01" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
                Model:
            </span>
            <span class="font-medium text-blue-500 dark:text-blue-400">{{ providerModel }}</span>
        </div>
        <div class="flex items-center space-x-2">
            <button class="hover:text-gray-700 dark:hover:text-gray-300 transition-colors duration-200"
                title="Copy model name" @click="copyToClipboard">
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M8 5H6C4.89543 5 4 5.89543 4 7V19C4 20.1046 4.89543 21 6 21H16C17.1046 21 18 20.1046 18 19V17M8 5C8 6.10457 8.89543 7 10 7H12C13.1046 7 14 6.10457 14 5M8 5C8 3.89543 8.89543 3 10 3H12C13.1046 3 14 3.89543 14 5M14 5H16C17.1046 5 18 5.89543 18 7V10"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'ResponseTopToolbar',
    props: {
        providerModel: {
            type: String,
            required: true,
        },
    },
    setup(props) {
        const copyToClipboard = async () => {
            try {
                await navigator.clipboard.writeText(props.providerModel);
            } catch (err) {
                console.error('Failed to copy to clipboard:', err);
            }
        };

        return {
            copyToClipboard,
        };
    },
});
</script>

<style scoped>
button {
    @apply focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 rounded;
}
</style>
