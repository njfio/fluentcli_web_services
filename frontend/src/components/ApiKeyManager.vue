<template>
    <div class="api-key-manager">
        <h2 class="text-2xl font-bold mb-4">API Key Management</h2>
        <div v-if="loading" class="text-center">
            <p>Loading...</p>
        </div>
        <div v-else>
            <div class="api-key-form mb-6">
                <input v-model="newApiKey.name" placeholder="API Key Name" class="mr-2 p-2 border rounded" />
                <input v-model="newApiKey.key_value" placeholder="API Key Value" class="mr-2 p-2 border rounded" />
                <input v-model="newApiKey.description" placeholder="Description" class="mr-2 p-2 border rounded" />
                <button @click="createApiKey"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    Add API Key
                </button>
            </div>
            <ul v-if="apiKeys.length" class="api-key-list">
                <li v-for="apiKey in apiKeys" :key="apiKey.id"
                    class="api-key-item mb-2 p-2 border rounded flex justify-between items-center">
                    <div>
                        <span>{{ apiKey.name }}: {{ maskApiKey(apiKey.key_value) }}</span>
                        <p class="text-sm text-gray-600">{{ apiKey.description }}</p>
                    </div>
                    <button @click="confirmDelete(apiKey.id)"
                        class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded">
                        Delete
                    </button>
                </li>
            </ul>
            <p v-else>No API keys found.</p>
        </div>
        <div v-if="error" class="error-message mt-4 p-2 bg-red-100 text-red-700 rounded">
            {{ error }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import apiClient from '@/services/apiClient';

interface ApiKey {
    id: string;
    name: string;
    key_value: string;
    description: string;
}

const apiKeys = ref<ApiKey[]>([]);
const newApiKey = ref<{ name: string; key_value: string; description: string }>({ name: '', key_value: '', description: '' });
const loading = ref(false);
const error = ref('');

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

const createApiKey = async () => {
    loading.value = true;
    error.value = '';
    try {
        await apiClient.createApiKey(newApiKey.value.name, newApiKey.value.key_value, newApiKey.value.description);
        newApiKey.value = { name: '', key_value: '', description: '' };
        await fetchApiKeys();
    } catch (err) {
        console.error('Error creating API key:', err);
        error.value = 'Failed to create API key. Please try again.';
    } finally {
        loading.value = false;
    }
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

const maskApiKey = (key: string) => {
    if (!key) return 'N/A';
    if (key.length <= 8) return '*'.repeat(key.length);
    return key.substring(0, 4) + '*'.repeat(key.length - 8) + key.substring(key.length - 4);
};

onMounted(fetchApiKeys);
</script>

<style scoped>
.api-key-manager {
    max-width: 600px;
    margin: 0 auto;
}
</style>
