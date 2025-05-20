<template>
  <div class="unified-llm-config p-6">
    <div class="flex justify-between items-center mb-6">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">LLM Configuration</h2>
      <button @click="startNewConfig" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
        transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
        flex items-center space-x-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        <span>New Configuration</span>
      </button>
    </div>

    <!-- Configuration Wizard -->
    <div v-if="showWizard" class="mb-6 p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-md">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ editingConfig ? 'Edit Configuration' : 'New Configuration' }}
        </h3>
        <button @click="cancelWizard" class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Step Indicator -->
      <div class="flex mb-6">
        <div v-for="(step, index) in steps" :key="index"
          class="flex-1 text-center relative">
          <div :class="`
            w-8 h-8 mx-auto rounded-full flex items-center justify-center text-sm font-medium
            ${currentStep > index ? 'bg-green-500 text-white' : currentStep === index ? 'bg-blue-600 text-white' : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'}
          `">
            {{ index + 1 }}
          </div>
          <div class="mt-2 text-xs font-medium" :class="currentStep === index ? 'text-blue-600 dark:text-blue-400' : 'text-gray-500 dark:text-gray-400'">
            {{ step }}
          </div>
          <div v-if="index < steps.length - 1" class="absolute top-4 left-1/2 w-full h-0.5"
            :class="currentStep > index ? 'bg-green-500' : 'bg-gray-200 dark:bg-gray-700'"></div>
        </div>
      </div>

      <!-- Step 1: Select Template -->
      <div v-if="currentStep === 0" class="space-y-4">
        <h4 class="text-lg font-medium text-gray-900 dark:text-white mb-4">Select a Template</h4>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div v-for="template in templates" :key="template.id"
            @click="selectTemplate(template)"
            class="p-4 border rounded-lg cursor-pointer transition-all duration-200 hover:shadow-md"
            :class="selectedTemplate?.id === template.id ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-200 dark:border-gray-700'">
            <div class="flex items-center mb-2">
              <img :src="template.logo" :alt="template.name" class="w-8 h-8 mr-2">
              <h5 class="font-medium text-gray-900 dark:text-white">{{ template.name }}</h5>
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ template.description }}</p>
          </div>

          <div @click="selectTemplate(null)"
            class="p-4 border rounded-lg cursor-pointer transition-all duration-200 hover:shadow-md"
            :class="selectedTemplate === null ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-200 dark:border-gray-700'">
            <div class="flex items-center mb-2">
              <div class="w-8 h-8 mr-2 bg-gray-200 dark:bg-gray-700 rounded-full flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-500 dark:text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
              </div>
              <h5 class="font-medium text-gray-900 dark:text-white">Custom Configuration</h5>
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-400">Create a custom configuration from scratch</p>
          </div>
        </div>

        <div class="flex justify-end mt-6">
          <button @click="nextStep"
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm transition duration-200"
            :disabled="currentStep === 0 && selectedTemplate === undefined">
            Next
          </button>
        </div>
      </div>

      <!-- Step 2: API Key -->
      <div v-if="currentStep === 1" class="space-y-4">
        <h4 class="text-lg font-medium text-gray-900 dark:text-white mb-4">API Key</h4>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">API Key Name</label>
            <input v-model="configData.apiKeyName" placeholder="e.g., My OpenAI Key"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">API Key</label>
            <input v-model="configData.apiKey" type="password" placeholder="Enter your API key"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
          </div>

          <div v-if="selectedTemplate">
            <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <div class="flex items-start">
                <svg class="h-5 w-5 text-blue-400 mt-0.5 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                </svg>
                <div>
                  <p class="text-sm text-blue-800 dark:text-blue-200">
                    <span class="font-medium">How to get a {{ selectedTemplate.name }} API key:</span>
                  </p>
                  <ul class="mt-1.5 ml-4 text-sm text-blue-700 dark:text-blue-300 list-disc">
                    <li v-for="(step, index) in selectedTemplate.keyInstructions" :key="index">
                      {{ step }}
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="flex justify-between mt-6">
          <button @click="prevStep" class="px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600
            text-gray-700 dark:text-gray-200 font-medium rounded-lg text-sm transition duration-200">
            Back
          </button>
          <button @click="nextStep"
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm transition duration-200"
            :disabled="!configData.apiKeyName || !configData.apiKey">
            Next
          </button>
        </div>
      </div>

      <!-- Step 3: Provider Configuration -->
      <div v-if="currentStep === 2" class="space-y-4">
        <h4 class="text-lg font-medium text-gray-900 dark:text-white mb-4">Provider Configuration</h4>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Configuration Name</label>
            <input v-model="configData.name" placeholder="e.g., My GPT-4 Config"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Provider Type</label>
            <select v-model="configData.providerType" @change="updateEndpointAndConfig"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm appearance-none">
              <option value="">Select Provider Type</option>
              <option v-for="type in providerTypes" :key="type.value" :value="type.value">
                {{ type.label }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">API Endpoint</label>
            <input v-model="configData.apiEndpoint" placeholder="Enter API endpoint"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Supported Modalities</label>
            <input v-model="configData.supportedModalities" placeholder="e.g., text, image"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm" />
          </div>

          <div class="md:col-span-2">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Configuration</label>
            <textarea v-model="configData.configuration" placeholder="Enter JSON configuration" rows="4"
              class="w-full p-2.5 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded-lg
              focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white text-sm font-mono"></textarea>
          </div>
        </div>

        <div class="flex justify-between mt-6">
          <button @click="prevStep" class="px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600
            text-gray-700 dark:text-gray-200 font-medium rounded-lg text-sm transition duration-200">
            Back
          </button>
          <button @click="saveConfiguration"
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm transition duration-200"
            :disabled="!isConfigValid">
            {{ editingConfig ? 'Update' : 'Save' }} Configuration
          </button>
        </div>
      </div>
    </div>

    <!-- Loading Indicator -->
    <div v-if="loading" class="flex justify-center items-center py-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
    </div>

    <!-- Configuration Cards -->
    <div v-else-if="userConfigs.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="config in userConfigs" :key="config.id"
        class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="p-5">
          <div class="flex justify-between items-start">
            <div class="flex items-center">
              <img :src="getProviderLogo(config.provider_type)" alt="Provider Logo" class="w-8 h-8 mr-3">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ config.description || 'Unnamed Config' }}</h3>
            </div>
            <div class="flex space-x-2">
              <button @click="editConfig(config)" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              <button @click="deleteConfig(config.id)" class="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>

          <div class="mt-4 space-y-2">
            <div class="flex justify-between text-sm">
              <span class="text-gray-500 dark:text-gray-400">Provider:</span>
              <span class="text-gray-900 dark:text-white font-medium">{{ config.provider_type }}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-500 dark:text-gray-400">API Key:</span>
              <span class="text-gray-900 dark:text-white font-medium">{{ getApiKeyName(config.api_key_id) }}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-gray-500 dark:text-gray-400">Modalities:</span>
              <span class="text-gray-900 dark:text-white font-medium">{{ config.supported_modalities?.join(', ') }}</span>
            </div>
          </div>

          <button @click="testConfig(config)" class="mt-4 w-full px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600
            text-gray-700 dark:text-gray-200 font-medium rounded-lg text-sm transition duration-200 flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            Test Configuration
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-12 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
      </svg>
      <h3 class="text-xl font-medium text-gray-900 dark:text-white mb-2">No LLM Configurations</h3>
      <p class="text-gray-500 dark:text-gray-400 mb-6">Get started by creating your first LLM configuration</p>
      <button @click="startNewConfig" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm
        transition duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
        Create Configuration
      </button>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="mt-6">
      <div class="p-4 bg-red-50 dark:bg-red-900/50 border border-red-200 dark:border-red-800 rounded-lg">
        <div class="flex">
          <svg class="h-5 w-5 text-red-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
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
import { ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import apiClient from '../services/apiClient';

// Store
const store = useStore();
const userId = computed(() => {
  const id = store.state.auth.user?.id || store.getters.userId;
  console.log('User ID for LLM config:', id);
  return id;
});

// Define types
interface Template {
  id: string;
  name: string;
  logo: string;
  description: string;
  providerType: string;
  apiEndpoint: string;
  supportedModalities: string;
  configuration: string;
  keyInstructions: string[];
}

interface UserConfig {
  id: string;
  user_id: string;
  provider_id: string;
  api_key_id: string;
  description: string;
  provider_type: string;
  supported_modalities: string[];
}

interface ApiKey {
  id: string;
  description: string;
  key_value: string;
}

// Component state
const loading = ref(false);
const error = ref('');
const showWizard = ref(false);
const currentStep = ref(0);
const steps = ['Select Template', 'API Key', 'Provider Configuration'];
const selectedTemplate = ref<Template | null>(null);
const editingConfig = ref<UserConfig | null>(null);
const userConfigs = ref<UserConfig[]>([]);
const apiKeys = ref<ApiKey[]>([]);

// Form data
const configData = ref({
  name: '',
  apiKeyName: '',
  apiKey: '',
  providerType: '',
  apiEndpoint: '',
  supportedModalities: '',
  configuration: '',
});

// Provider types
const providerTypes = [
  { value: 'gpt', label: 'OpenAI GPT' },
  { value: 'claude', label: 'Anthropic Claude' },
  { value: 'gemini', label: 'Google Gemini' },
  { value: 'perplexity', label: 'Perplexity' },
  { value: 'grok', label: 'Grok' },
  { value: 'dalle', label: 'DALL-E' },
  { value: 'leonardo', label: 'Leonardo AI' },
  { value: 'stability', label: 'Stability AI' },
  { value: 'custom', label: 'Custom Provider' },
];

// Templates
const templates: Template[] = [
  {
    id: 'openai-gpt4',
    name: 'OpenAI GPT-4',
    logo: '/assets/openai-logo.svg',
    description: 'OpenAI\'s most advanced model for text generation and understanding',
    providerType: 'gpt',
    apiEndpoint: 'https://api.openai.com/v1/chat/completions',
    supportedModalities: 'text',
    configuration: JSON.stringify({
      model: 'gpt-4',
      temperature: 0.7,
      max_tokens: 1024,
      top_p: 1,
      frequency_penalty: 0,
      presence_penalty: 0
    }, null, 2),
    keyInstructions: [
      'Go to https://platform.openai.com/account/api-keys',
      'Sign in or create an account',
      'Click "Create new secret key"',
      'Copy the key (you won\'t be able to see it again)'
    ]
  },
  {
    id: 'anthropic-claude',
    name: 'Anthropic Claude',
    logo: '/assets/anthropic-logo.svg',
    description: 'Claude is a family of AI assistants created by Anthropic',
    providerType: 'claude',
    apiEndpoint: 'https://api.anthropic.com/v1/messages',
    supportedModalities: 'text',
    configuration: JSON.stringify({
      model: 'claude-3-opus-20240229',
      temperature: 0.7,
      max_tokens: 1024,
      top_p: 0.95,
      top_k: 50
    }, null, 2),
    keyInstructions: [
      'Go to https://console.anthropic.com/account/keys',
      'Sign in or create an account',
      'Click "Create Key"',
      'Copy the key (you won\'t be able to see it again)'
    ]
  },
  {
    id: 'google-gemini',
    name: 'Google Gemini',
    logo: '/assets/gemini-logo.svg',
    description: 'Google\'s most capable and general model, with improved instruction following',
    providerType: 'gemini',
    apiEndpoint: 'https://generativelanguage.googleapis.com/v1beta/models',
    supportedModalities: 'text,image',
    configuration: JSON.stringify({
      model: 'gemini-1.5-pro',
      temperature: 0.7,
      top_k: 40,
      top_p: 0.95,
      max_tokens: 1024
    }, null, 2),
    keyInstructions: [
      'Go to https://ai.google.dev/',
      'Sign in with your Google account',
      'Navigate to "Get API key"',
      'Create a new project or select an existing one',
      'Copy your API key'
    ]
  },
  {
    id: 'perplexity',
    name: 'Perplexity',
    logo: '/assets/perplexity-logo.svg',
    description: 'Perplexity AI offers online LLMs with real-time information access',
    providerType: 'perplexity',
    apiEndpoint: 'https://api.perplexity.ai/chat/completions',
    supportedModalities: 'text',
    configuration: JSON.stringify({
      model: 'llama-3.1-sonar-huge-128k-online',
      max_tokens: 1024,
      temperature: 0.7
    }, null, 2),
    keyInstructions: [
      'Go to https://www.perplexity.ai/settings/api',
      'Sign in or create an account',
      'Generate a new API key',
      'Copy the key'
    ]
  }
];

// Computed properties
const isConfigValid = computed(() => {
  return configData.value.name &&
         configData.value.providerType &&
         configData.value.apiEndpoint &&
         configData.value.supportedModalities &&
         isValidJSON(configData.value.configuration);
});

// Methods
const isValidJSON = (str: string): boolean => {
  try {
    JSON.parse(str);
    return true;
  } catch {
    return false;
  }
};

const startNewConfig = (): void => {
  showWizard.value = true;
  currentStep.value = 0;
  selectedTemplate.value = null;
  editingConfig.value = null;
  resetConfigData();
};

const resetConfigData = (): void => {
  configData.value = {
    name: '',
    apiKeyName: '',
    apiKey: '',
    providerType: '',
    apiEndpoint: '',
    supportedModalities: '',
    configuration: '',
  };
};

const cancelWizard = (): void => {
  if (confirm('Are you sure you want to cancel? All unsaved changes will be lost.')) {
    showWizard.value = false;
    resetConfigData();
  }
};

const nextStep = (): void => {
  if (currentStep.value < steps.length - 1) {
    currentStep.value++;
  }
};

const prevStep = (): void => {
  if (currentStep.value > 0) {
    currentStep.value--;
  }
};

const selectTemplate = (template: Template | null): void => {
  selectedTemplate.value = template;

  if (template) {
    configData.value.name = `My ${template.name} Config`;
    configData.value.providerType = template.providerType;
    configData.value.apiEndpoint = template.apiEndpoint;
    configData.value.supportedModalities = template.supportedModalities;
    configData.value.configuration = template.configuration;
  } else {
    configData.value.name = '';
    configData.value.providerType = '';
    configData.value.apiEndpoint = '';
    configData.value.supportedModalities = '';
    configData.value.configuration = '';
  }
};

const updateEndpointAndConfig = (): void => {
  const selectedType = configData.value.providerType;
  const template = templates.find(t => t.providerType === selectedType);

  if (template) {
    configData.value.apiEndpoint = template.apiEndpoint;
    configData.value.supportedModalities = template.supportedModalities;
    configData.value.configuration = template.configuration;
  }
};

const saveConfiguration = async (): Promise<void> => {
  if (!isConfigValid.value) return;

  loading.value = true;
  error.value = '';

  try {
    // Get user ID from store
    const currentUserId = userId.value;
    console.log('Current user ID:', currentUserId);

    if (!currentUserId) {
      throw new Error('User ID not found. Please refresh the page and try again.');
    }

    // 1. Create or get API key
    let apiKeyId: string;

    if (editingConfig.value && editingConfig.value.api_key_id) {
      // Use existing API key if editing
      apiKeyId = editingConfig.value.api_key_id;
    } else {
      // Create new API key
      console.log('Creating API key with name:', configData.value.apiKeyName);
      const apiKeyResponse = await apiClient.createApiKey(
        configData.value.apiKey,
        configData.value.apiKeyName
      );
      apiKeyId = apiKeyResponse.data.id;
      console.log('Created API key with ID:', apiKeyId);
    }

    // 2. Create or update LLM provider
    const providerData = {
      name: configData.value.name,
      provider_type: configData.value.providerType,
      api_endpoint: configData.value.apiEndpoint,
      supported_modalities: configData.value.supportedModalities.split(',').map(s => s.trim()),
      configuration: JSON.parse(configData.value.configuration),
      user_id: currentUserId
    };

    console.log('Provider data:', providerData);
    let providerId: string;

    if (editingConfig.value && editingConfig.value.provider_id) {
      // Update existing provider
      console.log('Updating provider:', editingConfig.value.provider_id);
      await apiClient.updateLLMProvider(editingConfig.value.provider_id, providerData);
      providerId = editingConfig.value.provider_id;
    } else {
      // Create new provider
      console.log('Creating new provider');
      const providerResponse = await apiClient.createLLMProvider(providerData);
      providerId = providerResponse.data.id;
      console.log('Created provider with ID:', providerId);
    }

    // 3. Create or update user LLM config
    const configDataToSave = {
      user_id: currentUserId,
      provider_id: providerId,
      api_key_id: apiKeyId,
      description: configData.value.name,
    };

    console.log('Config data to save:', configDataToSave);

    if (editingConfig.value) {
      console.log('Updating config:', editingConfig.value.id);
      await apiClient.updateUserLLMConfig(editingConfig.value.id, configDataToSave);
    } else {
      console.log('Creating new config');
      await apiClient.createUserLLMConfig(configDataToSave);
    }

    // Reset and fetch updated data
    showWizard.value = false;
    resetConfigData();
    await fetchUserConfigs();

  } catch (err) {
    console.error('Error saving configuration:', err);
    error.value = `Failed to ${editingConfig.value ? 'update' : 'create'} configuration. Please try again.`;
  } finally {
    loading.value = false;
  }
};

const fetchUserConfigs = async (): Promise<void> => {
  loading.value = true;
  error.value = '';

  try {
    // Check if user ID is available
    const currentUserId = userId.value;
    console.log('Fetching configs for user ID:', currentUserId);

    if (!currentUserId) {
      console.warn('No user ID available when fetching configs');
      // Don't throw error here, just log warning and continue
    }

    // Fetch user LLM configs with provider and API key details
    console.log('Fetching user LLM configs');
    const configsResponse = await apiClient.listUserLLMConfigs();
    userConfigs.value = configsResponse.data;
    console.log('Fetched user configs:', userConfigs.value.length);

    // Fetch API keys for reference
    console.log('Fetching API keys');
    const apiKeysResponse = await apiClient.listApiKeys();
    apiKeys.value = apiKeysResponse.data;
    console.log('Fetched API keys:', apiKeys.value.length);

  } catch (err) {
    console.error('Error fetching configurations:', err);
    if (err instanceof Error) {
      error.value = `Failed to fetch configurations: ${err.message}`;
    } else {
      error.value = 'Failed to fetch configurations. Please try again.';
    }
  } finally {
    loading.value = false;
  }
};

const editConfig = async (config: UserConfig): Promise<void> => {
  editingConfig.value = config;

  try {
    // Fetch provider details
    const providerResponse = await apiClient.getLLMProvider(config.provider_id);
    const provider = providerResponse.data;

    // Set up form data
    configData.value = {
      name: config.description || '',
      apiKeyName: getApiKeyName(config.api_key_id),
      apiKey: '********', // We don't get the actual key back for security
      providerType: provider.provider_type,
      apiEndpoint: provider.api_endpoint,
      supportedModalities: provider.supported_modalities?.join(', ') || '',
      configuration: JSON.stringify(provider.configuration, null, 2),
    };

    // Show wizard at provider configuration step
    showWizard.value = true;
    currentStep.value = 2;

  } catch (err) {
    console.error('Error setting up edit form:', err);
    error.value = 'Failed to load configuration details. Please try again.';
  }
};

const deleteConfig = async (id: string): Promise<void> => {
  if (!confirm('Are you sure you want to delete this configuration?')) return;

  loading.value = true;
  error.value = '';

  try {
    await apiClient.deleteUserLLMConfig(id);
    await fetchUserConfigs();
  } catch (err) {
    console.error('Error deleting configuration:', err);
    error.value = 'Failed to delete configuration. Please try again.';
  } finally {
    loading.value = false;
  }
};

const testConfig = async (_config: UserConfig): Promise<void> => {
  // Implement test functionality
  alert('Test functionality will be implemented in a future update.');
};

const getApiKeyName = (apiKeyId: string): string => {
  const apiKey = apiKeys.value.find(key => key.id === apiKeyId);
  return apiKey ? apiKey.description : 'Unknown API Key';
};

const getProviderLogo = (providerType: string): string => {
  // Use local assets to avoid 404 errors
  const logoMap: Record<string, string> = {
    'gpt': '/assets/openai-logo.svg',
    'claude': '/assets/anthropic-logo.svg',
    'gemini': '/assets/gemini-logo.svg',
    'perplexity': '/assets/perplexity-logo.svg',
    'grok': '/assets/grok-logo.svg',
    'dalle': '/assets/dalle-logo.svg',
    'leonardo': '/assets/leonardo-logo.svg',
    'stability': '/assets/stability-logo.svg',
    'custom': '/assets/custom-logo.svg'
  };

  return logoMap[providerType] || '/assets/default-logo.svg';
};

// Initialize
onMounted(() => {
  fetchUserConfigs();
});
</script>

<style scoped>
.unified-llm-config {
  max-width: 1200px;
  margin: 0 auto;
}
</style>
