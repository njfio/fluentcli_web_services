<template>
    <div class="api-key-manager">
        <h2 class="text-2xl font-bold mb-4">API Key Management</h2>
        <div v-if="loading" class="text-center">
            <p>Loading...</p>
        </div>
        <div v-else>
            <div class="api-key-form mb-6">
                <input v-model="newApiKey.key_value" placeholder="API Key Value"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <input v-model="newApiKey.description" placeholder="Description"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <button @click="createOrUpdateApiKey"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded dark:bg-blue-600 dark:hover:bg-blue-800">
                    {{ editingApiKey ? 'Update' : 'Add' }} API Key
                </button>
            </div>
            <ul v-if="apiKeys.length" class="api-key-list">
                <li v-for="apiKey in apiKeys" :key="apiKey.id"
                    class="api-key-item mb-2 p-2 border rounded flex justify-between items-center dark:bg-gray-800 dark:text-white">
                    <div class="flex-grow">
                        <p>{{ apiKey.description }}</p>
                    </div>
                    <div class="flex">
                        <button @click="editApiKey(apiKey)"
                            class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded mr-2 dark:bg-green-600 dark:hover:bg-green-800">
                            Edit
                        </button>
                        <button @click="confirmDelete(apiKey.id)"
                            class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded dark:bg-red-600 dark:hover:bg-red-800">
                            Delete
                        </button>
                    </div>
                </li>
            </ul>
            <p v-else class="dark:text-white">No API keys found.</p>
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

interface ApiKey {
    id: string;
    key_value: string;
    description: string;
}

const apiKeys = ref<ApiKey[]>([]);
const newApiKey = ref<{ key_value: string; description: string }>({ key_value: '', description: '' });
const loading = ref(false);
const error = ref('');
const editingApiKey = ref<ApiKey | null>(null);

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
    max-width: 600px;
    margin: 0 auto;
}
</style>
