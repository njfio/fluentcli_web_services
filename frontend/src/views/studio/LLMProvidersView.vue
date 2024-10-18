<template>
    <div class="llm-providers-view">
        <h1 class="text-2xl font-bold mb-4">LLM Providers</h1>
        <LLMProviderManager />
    </div>
</template>

<script setup lang="ts">
import LLMProviderManager from '../../components/LLMProviderManager.vue';
</script>

<style scoped>
.llm-providers-view {
    padding: 20px;
}
</style>
