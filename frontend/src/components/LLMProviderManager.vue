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
                <select v-model="newProvider.providerType" @change="updateProviderDefaults"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white">
                    <option value="">Select Provider Type</option>
                    <option value="gpt">GPT</option>
                    <option value="claude">Claude</option>
                    <option value="command">Command</option>
                    <option value="dalle">DALL-E</option>
                    <option value="perplexity">Perplexity</option>
                </select>
                <input v-model="newProvider.apiEndpoint" placeholder="API Endpoint"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <input v-model="newProvider.supportedModalities" placeholder="Supported Modalities (comma-separated)"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white" />
                <textarea v-model="newProvider.configuration" placeholder="Configuration (JSON)"
                    class="mr-2 p-2 border rounded dark:bg-gray-700 dark:text-white"></textarea>
                <button @click="createOrUpdateLLMProvider"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded dark:bg-blue-600 dark:hover:bg-blue-800">
                    {{ editingProvider ? 'Update' : 'Add' }} LLM Provider
                </button>
                <button v-if="editingProvider" @click="cancelEdit"
                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded dark:bg-gray-600 dark:hover:bg-gray-800 ml-2">
                    Cancel
                </button>
            </div>
            <ul v-if="llmProviders.length" class="llm-provider-list">
                <li v-for="provider in llmProviders" :key="provider.id"
                    class="llm-provider-item mb-2 p-2 border rounded flex justify-between items-center dark:bg-gray-800 dark:text-white">
                    <div>
                        <p><strong>{{ provider.name }}</strong></p>
                        <p class="text-sm">Type: {{ provider.provider_type }}</p>
                        <p class="text-sm">Endpoint: {{ provider.api_endpoint }}</p>
                        <p class="text-sm">Modalities: {{ provider.supported_modalities ?
                            provider.supported_modalities.join(', ') : '' }}</p>
                        <p class="text-sm">Configuration: {{ JSON.stringify(provider.configuration) }}</p>
                    </div>
                    <div>
                        <button @click="editLLMProvider(provider)"
                            class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded dark:bg-green-600 dark:hover:bg-green-800 mr-2">
                            Edit
                        </button>
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
        configuration: JSON.stringify(provider.configuration),
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
