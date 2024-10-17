<template>
    <div class="api-keys-view">
        <h1 class="text-2xl font-bold mb-4">API Keys</h1>
        <ApiKeyManager />
    </div>
</template>

<script setup lang="ts">
import ApiKeyManager from '@/components/ApiKeyManager.vue';
</script>

<style scoped>
.api-keys-view {
    padding: 1rem;
}
</style>
