<template>
    <div class="llm-provider-manager p-6">
        <div class="flex justify-between items-center mb-6">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white">LLM Provider Management</h2>
            <button @click="showAddForm = true" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
                transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                <span>Add New Provider</span>
            </button>
        </div>

        <!-- Add/Edit Form -->
        <div v-if="showAddForm"
            class="mb-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Provider Name</label>
                    <input v-model="newProvider.name" placeholder="Enter provider name" class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Provider Type</label>
                    <select v-model="newProvider.providerType" @change="updateProviderDefaults"
                        class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm appearance-none">
                        <option value="">Select Provider Type</option>
                        <option value="gpt">GPT</option>
                        <option value="claude">Claude</option>
                        <option value="claude-computer">Claude Computer</option>
                        <option value="command">Command</option>
                        <option value="dalle">DALL-E</option>
                        <option value="perplexity">Perplexity</option>
                        <option value="gemini">Gemini</option>
                        <option value="grok">Grok</option>
                        <option value="leonardo">Leonardo AI</option>
                        <option value="stability">Stability AI</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">API Endpoint</label>
                    <input v-model="newProvider.apiEndpoint" placeholder="Enter API endpoint" class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Supported
                        Modalities</label>
                    <input v-model="newProvider.supportedModalities" placeholder="e.g., text, image" class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
                </div>
                <div class="md:col-span-2">
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Configuration</label>
                    <textarea v-model="newProvider.configuration" placeholder="Enter JSON configuration" rows="4"
                        class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg 
                        focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm font-mono"></textarea>
                </div>
            </div>
            <div class="flex justify-end space-x-2">
                <button @click="cancelEdit" class="px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-600 dark:hover:bg-gray-500 
                    text-gray-700 dark:text-gray-200 font-medium rounded-lg text-sm transition duration-200">
                    Cancel
                </button>
                <button @click="createOrUpdateLLMProvider" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
                    transition duration-200 disabled:opacity-50 disabled:cursor-not-allowed" :disabled="!isFormValid">
                    {{ editingProvider ? 'Update' : 'Add' }} Provider
                </button>
            </div>
        </div>

        <div v-if="loading" class="flex justify-center items-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        </div>

        <div v-else-if="llmProviders.length">
            <table class="w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead class="bg-gray-50 dark:bg-gray-700">
                    <tr>
                        <th scope="col"
                            class="w-1/4 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Name
                        </th>
                        <th scope="col"
                            class="w-1/6 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Type
                        </th>
                        <th scope="col"
                            class="w-1/4 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Endpoint
                        </th>
                        <th scope="col"
                            class="w-1/4 px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Modalities
                        </th>
                        <th scope="col"
                            class="w-1/6 px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                            Actions
                        </th>
                    </tr>
                </thead>
                <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                    <tr v-for="provider in llmProviders" :key="provider.id"
                        class="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-150">
                        <td class="px-4 py-4">
                            <div class="text-sm font-medium text-gray-900 dark:text-white truncate">{{ provider.name }}
                            </div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400">{{ provider.provider_type }}</div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400 truncate">{{ provider.api_endpoint }}
                            </div>
                        </td>
                        <td class="px-4 py-4">
                            <div class="text-sm text-gray-500 dark:text-gray-400">
                                {{ provider.supported_modalities?.join(', ') }}
                            </div>
                        </td>
                        <td class="px-4 py-4 text-right">
                            <div class="flex justify-end space-x-3">
                                <button @click="editLLMProvider(provider)"
                                    class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300 text-sm">
                                    Edit
                                </button>
                                <button @click="deleteLLMProvider(provider.id)"
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
                    d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <p class="text-lg font-medium">No LLM providers found</p>
            <p class="mt-1">Add your first provider using the button above</p>
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
const showAddForm = ref(false);

const isFormValid = computed(() => {
    return newProvider.value.name &&
        newProvider.value.providerType &&
        newProvider.value.apiEndpoint &&
        newProvider.value.supportedModalities &&
        isValidJSON(newProvider.value.configuration);
});

const isValidJSON = (str: string) => {
    try {
        JSON.parse(str);
        return true;
    } catch {
        return false;
    }
};

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

const createOrUpdateLLMProvider = async () => {
    if (!isFormValid.value) return;

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
        showAddForm.value = false;
        await fetchLLMProviders();
    } catch (err) {
        console.error('Error creating/updating LLM provider:', err);
        error.value = `Failed to ${editingProvider.value ? 'update' : 'create'} LLM provider. Please try again.`;
    } finally {
        loading.value = false;
    }
};

const editLLMProvider = (provider: LLMProvider) => {
    editingProvider.value = provider;
    newProvider.value = {
        name: provider.name,
        providerType: provider.provider_type,
        apiEndpoint: provider.api_endpoint,
        supportedModalities: provider.supported_modalities ? provider.supported_modalities.join(', ') : '',
        configuration: JSON.stringify(provider.configuration, null, 2),
    };
    showAddForm.value = true;
};

const cancelEdit = () => {
    editingProvider.value = null;
    newProvider.value = { name: '', providerType: '', apiEndpoint: '', supportedModalities: '', configuration: '' };
    showAddForm.value = false;
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
    if (newProvider.value.providerType === 'claude-computer') {
        newProvider.value.apiEndpoint = 'https://api.anthropic.com/v1/messages';
        newProvider.value.supportedModalities = 'text,computer';
        newProvider.value.configuration = JSON.stringify({
            model: 'claude-3-5-sonnet-20241022',
            max_tokens: 1024,
            temperature: 0.7,
            tools: [
                {
                    type: "computer_20241022",
                    name: "computer",
                    display_width_px: 1024,
                    display_height_px: 768,
                    display_number: 1
                },
                {
                    type: "text_editor_20241022",
                    name: "str_replace_editor"
                },
                {
                    type: "bash_20241022",
                    name: "bash"
                }
            ]
        }, null, 2);
    } else if (newProvider.value.providerType === 'perplexity') {
        newProvider.value.apiEndpoint = 'https://api.perplexity.ai/chat/completions';
        newProvider.value.supportedModalities = 'text';
        newProvider.value.configuration = JSON.stringify({
            model: 'llama-3.1-sonar-huge-128k-online',
            max_tokens: 1024,
            temperature: 0.7
        }, null, 2);
    } else if (newProvider.value.providerType === 'gemini') {
        newProvider.value.apiEndpoint = 'https://generativelanguage.googleapis.com/v1beta/models';
        newProvider.value.supportedModalities = 'text';
        newProvider.value.configuration = JSON.stringify({
            model: 'gemini-1.5-pro',
            temperature: 0.7,
            top_k: 40,
            top_p: 0.95,
            max_tokens: 1024
        }, null, 2);
    } else if (newProvider.value.providerType === 'grok') {
        newProvider.value.apiEndpoint = 'https://api.x.ai/v1/chat/completions';
        newProvider.value.supportedModalities = 'text';
        newProvider.value.configuration = JSON.stringify({
            model: 'grok-beta',
            temperature: 0.7,
            max_tokens: 1024,
            top_p: 0.95,
            stream: true
        }, null, 2);
    } else if (newProvider.value.providerType === 'leonardo') {
        newProvider.value.apiEndpoint = 'https://cloud.leonardo.ai/api/rest/v1/generations';
        newProvider.value.supportedModalities = 'image';
        newProvider.value.configuration = JSON.stringify({
            model: 'phoenix',
            width: 1024,
            height: 1024,
            num_images: 1,
            contrast: 3.5,
            alchemy: true,
            enhance_prompt: false
        }, null, 2);
    } else if (newProvider.value.providerType === 'stability') {
        newProvider.value.apiEndpoint = 'https://api.stability.ai/v2beta/stable-image/generate/ultra';
        newProvider.value.supportedModalities = 'image';
        newProvider.value.configuration = JSON.stringify({
            output_format: "png",
            cfg_scale: 7,
            height: 1024,
            width: 1024,
            steps: 30,
            seed: 0,
            style: "enhance"
        }, null, 2);
    }
};

onMounted(() => {
    fetchLLMProviders();
});
</script>

<style scoped>
.llm-provider-manager {
    max-width: 1200px;
    margin: 0 auto;
}
</style>
