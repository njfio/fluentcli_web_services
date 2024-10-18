<template>
    <div class="user-llm-configs-view">
        <h1 class="text-2xl font-bold mb-4">User LLM Configurations</h1>
        <UserLLMConfigManager />
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import UserLLMConfigManager from '@/components/UserLLMConfigManager.vue';

export default defineComponent({
    name: 'UserLLMConfigsView',
    components: {
        UserLLMConfigManager,
    },
});
</script>

<style scoped>
.user-llm-configs-view {
    padding: 20px;
}
</style>
