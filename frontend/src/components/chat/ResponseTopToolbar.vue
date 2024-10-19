<template>
    <div class="response-top-toolbar">
        <div class="provider-info">
            {{ providerModel }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

const props = defineProps<{
    providerModel: string;
}>();
</script>

<style scoped>
.response-top-toolbar {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 0.25rem 0.5rem;
    background-color: rgba(0, 0, 0, 0.1);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
}

.provider-info {
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 500;
}
</style>
