<template>
    <div class="user-llm-config-manager">
        <h2 class="text-2xl font-bold mb-4">User LLM Configuration Management</h2>
        <div v-if="loading" class="text-center">
            <p>Loading...</p>
        </div>
        <div v-else>
            <div class="user-llm-config-form mb-6">
                <select v-model="newConfig.providerId" class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white">
                    <option value="">Select LLM Provider</option>
                    <option v-for="provider in llmProviders" :key="provider.id" :value="provider.id">
                        {{ provider.name }}
                    </option>
                </select>
                <select v-model="newConfig.apiKeyId" class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white">
                    <option value="">Select API Key</option>
                    <option v-for="apiKey in apiKeys" :key="apiKey.id" :value="apiKey.id">
                        {{ apiKey.description }}
                    </option>
                </select>
                <input v-model="newConfig.description" placeholder="Description"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <button @click="createOrUpdateUserLLMConfig"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded dark:bg-blue-600 dark:hover:bg-blue-800">
                    {{ editingConfig ? 'Update' : 'Add' }} User LLM Config
                </button>
                <button v-if="editingConfig" @click="cancelEdit"
                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded dark:bg-gray-600 dark:hover:bg-gray-800 ml-2">
                    Cancel
                </button>
            </div>
            <ul v-if="userLLMConfigs.length" class="user-llm-config-list">
                <li v-for="config in userLLMConfigs" :key="config.id"
                    class="user-llm-config-item mb-2 p-2 border rounded flex justify-between items-center dark:bg-gray-800 dark:text-white">
                    <div>
                        <p class="font-bold">{{ config.description || 'No description' }}</p>
                        <p class="text-sm">Provider: {{ getProviderName(config.provider_id) }}</p>
                        <p class="text-sm">API Key: {{ getApiKeyDescription(config.api_key_id) }}</p>
                    </div>
                    <div>
                        <button @click="editUserLLMConfig(config)"
                            class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded dark:bg-green-600 dark:hover:bg-green-800 mr-2">
                            Edit
                        </button>
                        <button @click="deleteUserLLMConfig(config.id)"
                            class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded dark:bg-red-600 dark:hover:bg-red-800">
                            Delete
                        </button>
                    </div>
                </li>
            </ul>
            <p v-else class="dark:text-white">No User LLM Configurations found.</p>
        </div>
        <div v-if="error"
            class="error-message mt-4 p-2 bg-red-100 text-red-700 rounded dark:bg-red-900 dark:text-red-300">
            {{ error }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useStore } from 'vuex';
import apiClient from '../services/apiClient';

interface UserLLMConfig {
    id: string;
    user_id: string;
    provider_id: string;
    api_key_id: string;
    description?: string;
}

interface LLMProvider {
    id: string;
    name: string;
}

interface ApiKey {
    id: string;
    description: string;
}

const store = useStore();
const userId = computed(() => store.state.auth.user?.user_id);

const userLLMConfigs = ref<UserLLMConfig[]>([]);
const llmProviders = ref<LLMProvider[]>([]);
const apiKeys = ref<ApiKey[]>([]);
const newConfig = ref<{ providerId: string; apiKeyId: string; description: string }>({ providerId: '', apiKeyId: '', description: '' });
const loading = ref(false);
const error = ref('');
const editingConfig = ref<UserLLMConfig | null>(null);

const fetchUserLLMConfigs = async () => {
    loading.value = true;
    error.value = '';
    try {
        const response = await apiClient.listUserLLMConfigs();
        userLLMConfigs.value = response.data;
    } catch (err) {
        console.error('Error fetching User LLM Configs:', err);
        error.value = 'Failed to fetch User LLM Configs. Please try again.';
    } finally {
        loading.value = false;
    }
};

const fetchLLMProviders = async () => {
    try {
        const response = await apiClient.listLLMProviders();
        llmProviders.value = response.data;
    } catch (err) {
        console.error('Error fetching LLM Providers:', err);
        error.value = 'Failed to fetch LLM Providers. Please try again.';
    }
};

const fetchApiKeys = async () => {
    try {
        const response = await apiClient.listApiKeys();
        apiKeys.value = response.data;
    } catch (err) {
        console.error('Error fetching API Keys:', err);
        error.value = 'Failed to fetch API Keys. Please try again.';
    }
};

const createOrUpdateUserLLMConfig = async () => {
    loading.value = true;
    error.value = '';
    try {
        if (!userId.value) {
            throw new Error('User ID not found');
        }
        const configData = {
            user_id: userId.value,
            provider_id: newConfig.value.providerId,
            api_key_id: newConfig.value.apiKeyId,
            description: newConfig.value.description,
        };
        if (editingConfig.value) {
            await apiClient.updateUserLLMConfig(editingConfig.value.id, configData);
        } else {
            await apiClient.createUserLLMConfig(configData);
        }
        newConfig.value = { providerId: '', apiKeyId: '', description: '' };
        editingConfig.value = null;
        await fetchUserLLMConfigs();
    } catch (err) {
        console.error('Error creating/updating User LLM Config:', err);
        error.value = `Failed to ${editingConfig.value ? 'update' : 'create'} User LLM Config. Please try again.`;
    } finally {
        loading.value = false;
    }
};

const editUserLLMConfig = (config: UserLLMConfig) => {
    editingConfig.value = config;
    newConfig.value = {
        providerId: config.provider_id,
        apiKeyId: config.api_key_id,
        description: config.description || '',
    };
};

const cancelEdit = () => {
    editingConfig.value = null;
    newConfig.value = { providerId: '', apiKeyId: '', description: '' };
};

const deleteUserLLMConfig = async (id: string) => {
    if (!confirm('Are you sure you want to delete this User LLM Configuration?')) return;

    loading.value = true;
    error.value = '';
    try {
        await apiClient.deleteUserLLMConfig(id);
        await fetchUserLLMConfigs();
    } catch (err) {
        console.error('Error deleting User LLM Config:', err);
        error.value = 'Failed to delete User LLM Config. Please try again.';
    } finally {
        loading.value = false;
    }
};

const getProviderName = (providerId: string) => {
    const provider = llmProviders.value.find(p => p.id === providerId);
    return provider ? provider.name : 'Unknown Provider';
};

const getApiKeyDescription = (apiKeyId: string) => {
    const apiKey = apiKeys.value.find(key => key.id === apiKeyId);
    return apiKey ? apiKey.description : 'Unknown API Key';
};

onMounted(() => {
    fetchUserLLMConfigs();
    fetchLLMProviders();
    fetchApiKeys();
});
</script>

<style scoped>
.user-llm-config-manager {
    max-width: 800px;
    margin: 0 auto;
}
</style>
