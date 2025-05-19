<template>
  <div class="settings">
    <h1 class="text-2xl font-bold mb-4">Settings</h1>
    <p class="mb-6">Manage your Studio settings here.</p>

    <div class="settings-section">
      <h2 class="text-xl font-semibold mb-2">API Keys</h2>
      <p class="mb-4">Manage your API keys for various integrations.</p>
      <router-link to="/studio/apikeys" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Manage API Keys
      </router-link>
    </div>

    <!-- Add other settings sections here -->
  </div>
</template>

<script setup lang="ts">
// Add any necessary imports and logic here
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
