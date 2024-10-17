<template>
    <div class="llm-provider-manager">
        <h2 class="text-2xl font-bold mb-4">LLM Provider Management</h2>
        <div v-if="loading" class="text-center">
            <p>Loading...</p>
        </div>
        <div v-else>
            <div class="llm-provider-form mb-6">
                <input v-model="newProvider.name" placeholder="Provider Name"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <input v-model="newProvider.apiEndpoint" placeholder="API Endpoint"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <select v-model="newProvider.apiKeyId" class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white">
                    <option value="">Select API Key</option>
                    <option v-for="apiKey in apiKeys" :key="apiKey.id" :value="apiKey.id">
                        {{ apiKey.description }}
                    </option>
                </select>
                <button @click="createLLMProvider"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded dark:bg-blue-600 dark:hover:bg-blue-800">
                    Add LLM Provider
                </button>
            </div>
            <ul v-if="llmProviders.length" class="llm-provider-list">
                <li v-for="provider in llmProviders" :key="provider.id"
                    class="llm-provider-item mb-2 p-2 border rounded flex justify-between items-center dark:bg-gray-800 dark:text-white">
                    <div>
                        <p><strong>{{ provider.name }}</strong></p>
                        <p class="text-sm">{{ provider.apiEndpoint }}</p>
                        <p class="text-sm">API Key: {{ getApiKeyDescription(provider.apiKeyId) }}</p>
                    </div>
                    <div>
                        <button @click="deleteLLMProvider(provider.id)"
                            class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded dark:bg-red-600 dark:hover:bg-red-800">
                            Delete
                        </button>
                    </div>
                </li>
            </ul>
            <p v-else class="dark:text-white">No LLM providers found.</p>
        </div>
        <div v-if="error"
            class="error-message mt-4 p-2 bg-red-100 text-red-700 rounded dark:bg-red-900 dark:text-red-300">
            {{ error }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import apiClient from '../services/apiClient';

interface LLMProvider {
    id: string;
    name: string;
    apiEndpoint: string;
    apiKeyId: string;
}

interface ApiKey {
    id: string;
    description: string;
}

const llmProviders = ref<LLMProvider[]>([]);
const apiKeys = ref<ApiKey[]>([]);
const newProvider = ref<{ name: string; apiEndpoint: string; apiKeyId: string }>({ name: '', apiEndpoint: '', apiKeyId: '' });
const loading = ref(false);
const error = ref('');

const fetchLLMProviders = async () => {
    loading.value = true;
    error.value = '';
    try {
        const response = await apiClient.listLLMProviders();
        llmProviders.value = response.data;
    } catch (err) {
        console.error('Error fetching LLM providers:', err);
        error.value = 'Failed to fetch LLM providers. Please try again.';
    } finally {
        loading.value = false;
    }
};

const fetchApiKeys = async () => {
    try {
        const response = await apiClient.listApiKeys();
        apiKeys.value = response.data;
    } catch (err) {
        console.error('Error fetching API keys:', err);
        error.value = 'Failed to fetch API keys. Please try again.';
    }
};

const createLLMProvider = async () => {
    loading.value = true;
    error.value = '';
    try {
        await apiClient.createLLMProvider(newProvider.value.name, newProvider.value.apiEndpoint);
        // Create user LLM config with the selected API key
        if (newProvider.value.apiKeyId) {
            await apiClient.createUserLLMConfig(newProvider.value.name, newProvider.value.apiKeyId);
        }
        newProvider.value = { name: '', apiEndpoint: '', apiKeyId: '' };
        await fetchLLMProviders();
    } catch (err) {
        console.error('Error creating LLM provider:', err);
        error.value = 'Failed to create LLM provider. Please try again.';
    } finally {
        loading.value = false;
    }
};

const deleteLLMProvider = async (id: string) => {
    if (!confirm('Are you sure you want to delete this LLM provider?')) return;

    loading.value = true;
    error.value = '';
    try {
        await apiClient.deleteLLMProvider(id);
        await fetchLLMProviders();
    } catch (err) {
        console.error('Error deleting LLM provider:', err);
        error.value = 'Failed to delete LLM provider. Please try again.';
    } finally {
        loading.value = false;
    }
};

const getApiKeyDescription = (apiKeyId: string) => {
    const apiKey = apiKeys.value.find(key => key.id === apiKeyId);
    return apiKey ? apiKey.description : 'Not found';
};

onMounted(() => {
    fetchLLMProviders();
    fetchApiKeys();
});
</script>

<style scoped>
.llm-provider-manager {
    max-width: 800px;
    margin: 0 auto;
}
</style>
