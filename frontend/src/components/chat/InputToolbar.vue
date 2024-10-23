<template>
    <div class="input-toolbar flex space-x-2">
        <button
            class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors duration-150"
            @click="$emit('attach-file')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd"
                    d="M8 4a3 3 0 00-3 3v4a5 5 0 0010 0V7a1 1 0 112 0v4a7 7 0 11-14 0V7a5 5 0 0110 0v4a3 3 0 11-6 0V7a1 1 0 012 0v4a1 1 0 102 0V7a3 3 0 00-3-3z"
                    clip-rule="evenodd" />
            </svg>
        </button>
        <button
            class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors duration-150"
            @click="$emit('insert-image')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd"
                    d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z"
                    clip-rule="evenodd" />
            </svg>
        </button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'InputToolbar',
    emits: ['attach-file', 'insert-image'],
});
</script>

<style scoped>
.input-toolbar button {
    padding: 0.375rem;
    border-radius: 0.375rem;
}

.input-toolbar button:hover {
    background-color: rgba(0, 0, 0, 0.05);
}

.dark .input-toolbar button:hover {
    background-color: rgba(255, 255, 255, 0.05);
}

.input-toolbar button:focus {
    outline: 2px solid transparent;
    outline-offset: 2px;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5);
}
</style>
