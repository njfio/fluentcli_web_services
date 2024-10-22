<template>
    <div class="response-top-toolbar">
        <div class="provider-info">
            {{ providerModel }}
        </div>
    </div>
</template>

<script setup lang="ts">
defineProps<{
    providerModel: string;
}>();
</script>

<style scoped>
.response-top-toolbar {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 0.25rem 0.5rem;
    background-color: rgba(0, 0, 0, 0.03);
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
    transition: background-color 0.2s, border-color 0.2s;
}

.dark .response-top-toolbar {
    background-color: rgba(255, 255, 255, 0.05);
    border-bottom-color: rgba(255, 255, 255, 0.1);
}

.provider-info {
    font-size: 0.75rem;
    color: rgba(0, 0, 0, 0.6);
    font-weight: 500;
    transition: color 0.2s;
}

.dark .provider-info {
    color: rgba(255, 255, 255, 0.7);
}

/* When inside a message */
:global(.message.bg-blue-600) .response-top-toolbar {
    background-color: rgba(255, 255, 255, 0.1);
    border-bottom-color: rgba(255, 255, 255, 0.2);
}

:global(.message.bg-blue-600) .provider-info {
    color: rgba(255, 255, 255, 0.9);
}
</style>
