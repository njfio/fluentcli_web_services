<template>
    <div class="api-key-manager p-6">
        <div class="flex justify-between items-center mb-6">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white">API Key Management</h2>
            <button @click="showAddForm = true" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
                transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                <span>Add New Key</span>
            </button>
        </div>

        <!-- Add/Edit Form -->
        <div v-if="showAddForm"
            class="mb-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">API Key</label>
                    <input v-model="newApiKey.key_value" placeholder="Enter your API key" class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Description</label>
                    <input v-model="newApiKey.description" placeholder="Enter a description" class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
                </div>
            </div>
            <div class="flex justify-end space-x-2">
                <button @click="cancelEdit" class="px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-600 dark:hover:bg-gray-500 
                    text-gray-700 dark:text-gray-200 font-medium rounded-lg text-sm transition duration-200">
                    Cancel
                </button>
                <button @click="createOrUpdateApiKey" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
                    transition duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                    :disabled="!newApiKey.key_value || !newApiKey.description">
                    {{ editingApiKey ? 'Update' : 'Add' }} Key
                </button>
            </div>
        </div>

        <div v-if="loading" class="flex justify-center items-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>

        <div v-else-if="apiKeys.length">
            <table class="w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead class="bg-gray-50 dark:bg-gray-700">
                    <tr>
                        <th scope="col"
                            class="w-2/5 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Description
                        </th>
                        <th scope="col"
                            class="w-1/5 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            API Key
                        </th>
                        <th scope="col"
                            class="w-1/5 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Created
                        </th>
                        <th scope="col"
                            class="w-1/5 px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Actions
                        </th>
                    </tr>
                </thead>
                <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                    <tr v-for="apiKey in apiKeys" :key="apiKey.id"
                        class="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-150">
                        <td class="px-4 py-4">
                            <div class="text-sm font-medium text-gray-900 dark:text-white truncate">{{
                                apiKey.description }}</div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400 font-mono">********</div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400">{{ formatDate(apiKey.created_at) }}
                            </div>
                        </td>
                        <td class="px-4 py-4 text-right">
                            <div class="flex justify-end space-x-3">
                                <button @click="editApiKey(apiKey)"
                                    class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300 text-sm">
                                    Edit
                                </button>
                                <button @click="confirmDelete(apiKey.id)"
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
                    d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
            </svg>
            <p class="text-lg font-medium">No API keys found</p>
            <p class="mt-1">Add your first API key using the button above</p>
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
import { ref, onMounted } from 'vue';
import apiClient from '../services/apiClient';

interface ApiKey {
    id: string;
    key_value: string;
    description: string;
    created_at?: string;
}

const apiKeys = ref<ApiKey[]>([]);
const newApiKey = ref<{ key_value: string; description: string }>({ key_value: '', description: '' });
const loading = ref(false);
const error = ref('');
const editingApiKey = ref<ApiKey | null>(null);
const showAddForm = ref(false);

const formatDate = (dateString?: string) => {
    if (!dateString) return '';
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
};

const fetchApiKeys = async () => {
    loading.value = true;
    error.value = '';
    try {
        const response = await apiClient.listApiKeys();
        apiKeys.value = response.data;
    } catch (err) {
        console.error('Error fetching API keys:', err);
        error.value = 'Failed to fetch API keys. Please try again.';
    } finally {
        loading.value = false;
    }
};

const createOrUpdateApiKey = async () => {
    if (!newApiKey.value.key_value || !newApiKey.value.description) return;

    loading.value = true;
    error.value = '';
    try {
        if (editingApiKey.value) {
            await apiClient.updateApiKey(
                editingApiKey.value.id,
                newApiKey.value.key_value,
                newApiKey.value.description
            );
            editingApiKey.value = null;
        } else {
            await apiClient.createApiKey(
                newApiKey.value.key_value,
                newApiKey.value.description
            );
        }
        newApiKey.value = { key_value: '', description: '' };
        showAddForm.value = false;
        await fetchApiKeys();
    } catch (err) {
        console.error('Error creating/updating API key:', err);
        error.value = `Failed to ${editingApiKey.value ? 'update' : 'create'} API key. Please try again.`;
    } finally {
        loading.value = false;
    }
};

const editApiKey = (apiKey: ApiKey) => {
    editingApiKey.value = { ...apiKey };
    newApiKey.value = { key_value: apiKey.key_value, description: apiKey.description };
    showAddForm.value = true;
};

const cancelEdit = () => {
    editingApiKey.value = null;
    newApiKey.value = { key_value: '', description: '' };
    showAddForm.value = false;
};

const confirmDelete = (id: string) => {
    if (confirm('Are you sure you want to delete this API key?')) {
        deleteApiKey(id);
    }
};

const deleteApiKey = async (id: string) => {
    loading.value = true;
    error.value = '';
    try {
        await apiClient.deleteApiKey(id);
        await fetchApiKeys();
    } catch (err) {
        console.error('Error deleting API key:', err);
        error.value = 'Failed to delete API key. Please try again.';
    } finally {
        loading.value = false;
    }
};

onMounted(fetchApiKeys);
</script>

<style scoped>
.api-key-manager {
    max-width: 1200px;
    margin: 0 auto;
}
</style>
