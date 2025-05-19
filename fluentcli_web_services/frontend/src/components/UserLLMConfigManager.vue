<template>
    <div class="user-llm-config-manager p-6">
        <div class="flex justify-between items-center mb-6">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white">User LLM Configuration</h2>
            <button @click="showAddForm = true" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
                transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                <span>Add New Configuration</span>
            </button>
        </div>

        <!-- Add/Edit Form -->
        <div v-if="showAddForm"
            class="mb-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">LLM Provider</label>
                    <select v-model="newConfig.providerId"
                        class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm appearance-none">
                        <option value="">Select LLM Provider</option>
                        <option v-for="provider in llmProviders" :key="provider.id" :value="provider.id">
                            {{ provider.name }}
                        </option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">API Key</label>
                    <select v-model="newConfig.apiKeyId"
                        class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm appearance-none">
                        <option value="">Select API Key</option>
                        <option v-for="apiKey in apiKeys" :key="apiKey.id" :value="apiKey.id">
                            {{ apiKey.description }}
                        </option>
                    </select>
                </div>
                <div class="md:col-span-2">
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Description</label>
                    <input v-model="newConfig.description" placeholder="Enter a description" class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
                </div>
            </div>
            <div class="flex justify-end space-x-2">
                <button @click="cancelEdit" class="px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-600 dark:hover:bg-gray-500 
                    text-gray-700 dark:text-gray-200 font-medium rounded-lg text-sm transition duration-200">
                    Cancel
                </button>
                <button @click="createOrUpdateUserLLMConfig" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
                    transition duration-200 disabled:opacity-50 disabled:cursor-not-allowed" :disabled="!isFormValid">
                    {{ editingConfig ? 'Update' : 'Add' }} Configuration
                </button>
            </div>
        </div>

        <div v-if="loading" class="flex justify-center items-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>

        <div v-else-if="userLLMConfigs.length">
            <table class="w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead class="bg-gray-50 dark:bg-gray-700">
                    <tr>
                        <th scope="col"
                            class="w-1/3 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Description
                        </th>
                        <th scope="col"
                            class="w-1/4 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Provider
                        </th>
                        <th scope="col"
                            class="w-1/4 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            API Key
                        </th>
                        <th scope="col"
                            class="w-1/6 px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Actions
                        </th>
                    </tr>
                </thead>
                <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                    <tr v-for="config in userLLMConfigs" :key="config.id"
                        class="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-150">
                        <td class="px-4 py-4">
                            <div class="text-sm font-medium text-gray-900 dark:text-white truncate">
                                {{ config.description || 'No description' }}
                            </div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400">{{ getProviderName(config.provider_id)
                                }}</div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400">{{
                                getApiKeyDescription(config.api_key_id) }}</div>
                        </td>
                        <td class="px-4 py-4 text-right">
                            <div class="flex justify-end space-x-3">
                                <button @click="editUserLLMConfig(config)"
                                    class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300 text-sm">
                                    Edit
                                </button>
                                <button @click="deleteUserLLMConfig(config.id)"
                                    class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300 text-sm">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div v-else class="text-center py-8 text-gray-500 dark:text-gray-400">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto mb-4 text-gray-400" fill="none"
                viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <p class="text-lg font-medium">No configurations found</p>
            <p class="mt-1">Add your first configuration using the button above</p>
        </div>

        <div v-if="error" class="mt-6">
            <div class="p-4 bg-red-50 dark:bg-red-900/50 border border-red-200 dark:border-red-800 rounded-lg">
                <div class="flex">
                    <svg class="h-5 w-5 text-red-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                            clip-rule="evenodd" />
                    </svg>
                    <div class="ml-3">
                        <p class="text-sm text-red-800 dark:text-red-200">{{ error }}</p>
                    </div>
                </div>
            </div>
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
const showAddForm = ref(false);

const isFormValid = computed(() => {
    return newConfig.value.providerId && newConfig.value.apiKeyId && newConfig.value.description;
});

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
    if (!isFormValid.value) return;

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
        showAddForm.value = false;
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
    showAddForm.value = true;
};

const cancelEdit = () => {
    editingConfig.value = null;
    newConfig.value = { providerId: '', apiKeyId: '', description: '' };
    showAddForm.value = false;
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
    max-width: 1200px;
    margin: 0 auto;
}
</style>
