<template>
    <div class="llm-provider-manager bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
        <h2 class="text-2xl font-bold mb-6 text-gray-800 dark:text-white">LLM Provider Management</h2>
        <div v-if="loading" class="text-center text-gray-600 dark:text-gray-400">
            <p>Loading...</p>
        </div>
        <div v-else>
            <div class="llm-provider-form mb-8 space-y-4">
                <input v-model="newProvider.name" placeholder="Provider Name"
                    class="w-full p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <select v-model="newProvider.providerType" @change="updateProviderDefaults"
                    class="w-full p-2 border rounded dark:bg-gray-700 dark:text-white">
                    <option value="">Select Provider Type</option>
                    <option value="gpt">GPT</option>
                    <option value="claude">Claude</option>
                    <option value="command">Command</option>
                    <option value="dalle">DALL-E</option>
                    <option value="perplexity">Perplexity</option>
                    <option value="gemini">Gemini</option>
                </select>
                <input v-model="newProvider.apiEndpoint" placeholder="API Endpoint"
                    class="w-full p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <input v-model="newProvider.supportedModalities" placeholder="Supported Modalities (comma-separated)"
                    class="w-full p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <textarea v-model="newProvider.configuration" placeholder="Configuration (JSON)" rows="4"
                    class="w-full p-2 border rounded dark:bg-gray-700 dark:text-white"></textarea>
                <div class="flex space-x-2">
                    <button @click="createOrUpdateLLMProvider"
                        class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition duration-300">
                        {{ editingProvider ? 'Update' : 'Add' }} LLM Provider
                    </button>
                    <button v-if="editingProvider" @click="cancelEdit"
                        class="bg-gray-500 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded transition duration-300">
                        Cancel
                    </button>
                </div>
            </div>
            <ul v-if="llmProviders.length" class="space-y-4">
                <li v-for="provider in llmProviders" :key="provider.id"
                    class="bg-gray-100 dark:bg-gray-700 p-4 rounded-lg shadow">
                    <div class="flex justify-between items-start">
                        <div>
                            <h3 class="text-lg font-semibold text-gray-800 dark:text-white">{{ provider.name }}</h3>
                            <p class="text-sm text-gray-600 dark:text-gray-300">Type: {{ provider.provider_type }}</p>
                            <p class="text-sm text-gray-600 dark:text-gray-300">Endpoint: {{ provider.api_endpoint }}
                            </p>
                            <p class="text-sm text-gray-600 dark:text-gray-300">Modalities: {{
                                provider.supported_modalities ?
                                    provider.supported_modalities.join(', ') : '' }}</p>
                            <details class="mt-2">
                                <summary class="text-sm text-gray-600 dark:text-gray-300 cursor-pointer">Configuration
                                </summary>
                                <pre
                                    class="text-xs bg-gray-200 dark:bg-gray-600 p-2 rounded mt-1 overflow-x-auto">{{ JSON.stringify(provider.configuration, null, 2) }}</pre>
                            </details>
                        </div>
                        <div class="space-x-2">
                            <button @click="editLLMProvider(provider)"
                                class="bg-green-500 hover:bg-green-600 text-white font-bold py-1 px-2 rounded transition duration-300">
                                Edit
                            </button>
                            <button @click="deleteLLMProvider(provider.id)"
                                class="bg-red-500 hover:bg-red-600 text-white font-bold py-1 px-2 rounded transition duration-300">
                                Delete
                            </button>
                        </div>
                    </div>
                </li>
            </ul>
            <p v-else class="text-gray-600 dark:text-gray-400">No LLM providers found.</p>
        </div>
        <div v-if="error" class="mt-4 p-2 bg-red-100 text-red-700 rounded dark:bg-red-900 dark:text-red-300">
            {{ error }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useStore } from 'vuex';
import apiClient from '../services/apiClient';

interface LLMProvider {
    id: string;
    name: string;
    provider_type: string;
    api_endpoint: string;
    supported_modalities?: string[];
    configuration: any;
}

const store = useStore();
const userId = computed(() => store.state.auth.user?.user_id);

const llmProviders = ref<LLMProvider[]>([]);
const newProvider = ref<{
    name: string;
    providerType: string;
    apiEndpoint: string;
    supportedModalities: string;
    configuration: string;
}>({
    name: '',
    providerType: '',
    apiEndpoint: '',
    supportedModalities: '',
    configuration: '',
});
const loading = ref(false);
const error = ref('');
const editingProvider = ref<LLMProvider | null>(null);

const fetchLLMProviders = async () => {
    loading.value = true;
    error.value = '';
    try {
        const response = await apiClient.listLLMProviders();
        llmProviders.value = response.data;
        console.log('Fetched LLM Providers:', llmProviders.value);
    } catch (err) {
        console.error('Error fetching LLM providers:', err);
        error.value = 'Failed to fetch LLM providers. Please try again.';
    } finally {
        loading.value = false;
    }
};

const createOrUpdateLLMProvider = async () => {
    loading.value = true;
    error.value = '';
    try {
        if (!userId.value) {
            throw new Error('User ID not found');
        }
        const providerData = {
            name: newProvider.value.name,
            provider_type: newProvider.value.providerType,
            api_endpoint: newProvider.value.apiEndpoint,
            supported_modalities: newProvider.value.supportedModalities.split(',').map(s => s.trim()),
            configuration: JSON.parse(newProvider.value.configuration),
            user_id: userId.value
        };
        if (editingProvider.value) {
            await apiClient.updateLLMProvider(editingProvider.value.id, providerData);
        } else {
            await apiClient.createLLMProvider(providerData);
        }
        newProvider.value = { name: '', providerType: '', apiEndpoint: '', supportedModalities: '', configuration: '' };
        editingProvider.value = null;
        await fetchLLMProviders();
    } catch (err) {
        console.error('Error creating/updating LLM provider:', err);
        error.value = `Failed to ${editingProvider.value ? 'update' : 'create'} LLM provider. Please try again.`;
    } finally {
        loading.value = false;
    }
};

const editLLMProvider = (provider: LLMProvider) => {
    console.log('Editing provider:', provider);
    editingProvider.value = provider;
    newProvider.value = {
        name: provider.name,
        providerType: provider.provider_type,
        apiEndpoint: provider.api_endpoint,
        supportedModalities: provider.supported_modalities ? provider.supported_modalities.join(', ') : '',
        configuration: JSON.stringify(provider.configuration, null, 2),
    };
    console.log('New provider values:', newProvider.value);
};

const cancelEdit = () => {
    editingProvider.value = null;
    newProvider.value = { name: '', providerType: '', apiEndpoint: '', supportedModalities: '', configuration: '' };
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

const updateProviderDefaults = () => {
    if (newProvider.value.providerType === 'perplexity') {
        newProvider.value.apiEndpoint = 'https://api.perplexity.ai/chat/completions';
        newProvider.value.supportedModalities = 'text';
        newProvider.value.configuration = JSON.stringify({
            model: 'mixtral-8x7b-instruct',
            max_tokens: 1024,
            temperature: 0.7
        }, null, 2);
    } else if (newProvider.value.providerType === 'gemini') {
        newProvider.value.apiEndpoint = 'https://generativelanguage.googleapis.com/v1beta/models';
        newProvider.value.supportedModalities = 'text';
        newProvider.value.configuration = JSON.stringify({
            model: 'gemini-pro',
            temperature: 0.7,
            top_k: 40,
            top_p: 0.95,
            max_tokens: 1024
        }, null, 2);
    }
};

onMounted(() => {
    fetchLLMProviders();
});
</script>

<style scoped>
.llm-provider-manager {
    max-width: 800px;
    margin: 0 auto;
}
</style>
