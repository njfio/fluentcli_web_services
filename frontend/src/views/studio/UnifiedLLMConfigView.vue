<template>
  <div class="p-6">
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">LLM Configuration</h1>
      <p class="mt-2 text-gray-600 dark:text-gray-400">
        Configure your LLM providers and API keys in one place.
      </p>
    </div>

    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
      <UnifiedLLMConfigManager />
    </div>
  </div>
</template>

<script setup lang="ts">
import UnifiedLLMConfigManager from '@/components/UnifiedLLMConfigManager.vue';
</script>
