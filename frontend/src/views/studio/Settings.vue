<template>
  <div class="settings">
    <h1 class="text-2xl font-bold mb-4">Settings</h1>
    <p class="mb-6">Manage your Studio settings here.</p>

    <div class="settings-section mb-8">
      <h2 class="text-xl font-semibold mb-4">LLM Configurations</h2>
      <p class="mb-4">Set up and manage your AI model configurations in one place.</p>
      <UnifiedLLMConfigManager />
    </div>

    <div class="settings-section mb-8">
      <h2 class="text-xl font-semibold mb-4">Legacy Settings</h2>
      <p class="mb-4 text-gray-600">These settings are maintained for backward compatibility.</p>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <h3 class="text-lg font-medium mb-2">API Keys</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">Manage your API keys for various integrations.</p>
          <router-link to="/studio/apikeys" class="inline-block px-4 py-2 bg-blue-500 hover:bg-blue-700 text-white font-bold rounded transition duration-200">
            Manage API Keys
          </router-link>
        </div>

        <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <h3 class="text-lg font-medium mb-2">LLM Providers</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">Configure LLM provider settings.</p>
          <router-link to="/studio/llmproviders" class="inline-block px-4 py-2 bg-blue-500 hover:bg-blue-700 text-white font-bold rounded transition duration-200">
            Manage LLM Providers
          </router-link>
        </div>

        <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <h3 class="text-lg font-medium mb-2">User LLM Configs</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">Manage your user-specific LLM configurations.</p>
          <router-link to="/studio/userllmconfigs" class="inline-block px-4 py-2 bg-blue-500 hover:bg-blue-700 text-white font-bold rounded transition duration-200">
            Manage User LLM Configs
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import UnifiedLLMConfigManager from '@/components/UnifiedLLMConfigManager.vue';
</script>

<style scoped>
.settings {
  padding: 20px;
}

.settings-section {
  margin-bottom: 20px;
  padding: 20px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
}
</style>
