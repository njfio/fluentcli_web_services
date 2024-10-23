import { Module } from 'vuex';
import { RootState } from '../types';
import apiClient from '../../services/apiClient';
import { AxiosError } from 'axios';

export interface Conversation {
    id: string;
    title: string;
    mode: string;
    createdAt: string;
    updatedAt: string;
}

export interface Message {
    id: string;
    conversationId: string;
    role: string;
    content: string;
    createdAt: string;
    renderedContent?: string;
    providerModel?: string;
    attachmentId?: string;
    rawOutput?: string;
    usageStats?: any;
}

export interface Attachment {
    id: string;
    messageId: string;
    fileType: string;
    filePath: string;
    createdAt: string;
}

export interface LLMProvider {
    id: string;
    name: string;
    providerType: string;
    apiEndpoint: string;
    supportedModalities: string[];
    configuration: any;
}

export interface UserLLMConfig {
    id: string;
    userId: string;
    providerId: string;
    apiKeyId: string;
    description?: string;
}

export interface ChatState {
    conversations: Conversation[];
    currentConversation: Conversation | null;
    messages: Message[];
    attachments: { [messageId: string]: Attachment[] };
    llmProviders: LLMProvider[];
    userLLMConfigs: UserLLMConfig[];
}

const chatModule: Module<ChatState, RootState> = {
    namespaced: true,
    state: {
        conversations: [],
        currentConversation: null,
        messages: [],
        attachments: {},
        llmProviders: [],
        userLLMConfigs: [],
    },
    mutations: {
        setConversations(state, conversations: Conversation[]) {
            state.conversations = conversations;
        },
        setCurrentConversation(state, conversation: Conversation) {
            state.currentConversation = conversation;
        },
        addConversation(state, conversation: Conversation) {
            state.conversations.push(conversation);
        },
        setMessages(state, messages: Message[]) {
            state.messages = messages || [];
        },
        addMessage(state, message: Message) {
            state.messages.push(message);
        },
        updateMessage(state, updatedMessage: Message) {
            const index = state.messages.findIndex(message => message.id === updatedMessage.id);
            if (index !== -1) {
                state.messages[index] = updatedMessage;
            }
        },
        removeMessage(state, messageId: string) {
            state.messages = state.messages.filter(message => message.id !== messageId);
        },
        setAttachments(state, { messageId, attachments }: { messageId: string; attachments: Attachment[] }) {
            state.attachments[messageId] = attachments;
        },
        addAttachment(state, { messageId, attachment }: { messageId: string; attachment: Attachment }) {
            if (!state.attachments[messageId]) {
                state.attachments[messageId] = [];
            }
            state.attachments[messageId].push(attachment);
        },
        setLLMProviders(state, providers: LLMProvider[]) {
            state.llmProviders = providers;
        },
        addLLMProvider(state, provider: LLMProvider) {
            state.llmProviders.push(provider);
        },
        setLLMProvider(state, provider: LLMProvider) {
            const index = state.llmProviders.findIndex(p => p.id === provider.id);
            if (index !== -1) {
                state.llmProviders[index] = provider;
            } else {
                state.llmProviders.push(provider);
            }
        },
        setUserLLMConfigs(state, configs: UserLLMConfig[]) {
            state.userLLMConfigs = configs;
        },
        addUserLLMConfig(state, config: UserLLMConfig) {
            state.userLLMConfigs.push(config);
        },
        updateUserLLMConfig(state, updatedConfig: UserLLMConfig) {
            const index = state.userLLMConfigs.findIndex(config => config.id === updatedConfig.id);
            if (index !== -1) {
                state.userLLMConfigs[index] = updatedConfig;
            }
        },
        removeUserLLMConfig(state, configId: string) {
            state.userLLMConfigs = state.userLLMConfigs.filter(config => config.id !== configId);
        },
        removeConversation(state, conversationId: string) {
            state.conversations = state.conversations.filter(conv => conv.id !== conversationId);
            if (state.currentConversation && state.currentConversation.id === conversationId) {
                state.currentConversation = null;
            }
        },
        clearMessages(state) {
            state.messages = [];
        },
    },
    actions: {
        async deleteConversation({ commit }, conversationId: string) {
            try {
                await apiClient.deleteConversation(conversationId);
                commit('removeConversation', conversationId);
            } catch (error) {
                console.error('Error deleting conversation:', error);
                throw error;
            }
        },
        async getConversations({ commit }) {
            try {
                console.log('Fetching conversations');
                const response = await apiClient.listConversations();
                console.log('Conversations response:', response);
                const conversations = response.data;
                console.log('Fetched conversations:', conversations);
                commit('setConversations', conversations);
                return conversations;
            } catch (error) {
                console.error('Error fetching conversations:', error);
                if (error instanceof AxiosError && error.response) {
                    console.error('Error response:', error.response.data);
                    console.error('Error status:', error.response.status);
                }
                throw error;
            }
        },
        async createConversation({ commit, rootState }, { title, mode = 'chat' }: { title: string; mode?: string }) {
            try {
                const userId = rootState.auth.user?.user_id;
                if (!userId) {
                    throw new Error('User ID not found');
                }
                const response = await apiClient.createConversation({ user_id: userId, title, mode });
                const conversation = response.data;
                commit('addConversation', conversation);
                return conversation;
            } catch (error) {
                console.error('Error creating conversation:', error);
                throw error;
            }
        },
        async getConversation({ commit }, id: string) {
            try {
                const response = await apiClient.getConversation(id);
                const conversation = response.data;
                commit('setCurrentConversation', conversation);
                return conversation;
            } catch (error) {
                console.error('Error fetching conversation:', error);
                throw error;
            }
        },
        async createMessage({ commit }, {
            conversationId,
            role,
            content,
            providerModel,
            attachmentId,
            rawOutput,
            usageStats
        }: {
            conversationId: string;
            role: string;
            content: string;
            providerModel: string;
            attachmentId?: string;
            rawOutput?: string;
            usageStats?: any;
        }) {
            try {
                console.log('Creating message with data:', {
                    conversationId,
                    role,
                    content,
                    providerModel,
                    attachmentId,
                    rawOutput,
                    usageStats
                });
                const response = await apiClient.createMessage(
                    conversationId,
                    role,
                    content,
                    providerModel,
                    attachmentId,
                    rawOutput,
                    usageStats
                );
                console.log('Response from createMessage:', response);
                const message = response.data;
                if (message && message.id) {
                    commit('addMessage', message);
                    return message;
                } else {
                    console.error('Invalid message response:', message);
                    throw new Error('Invalid message response from server');
                }
            } catch (error: unknown) {
                console.error('Error creating message:', error);
                if (error instanceof AxiosError && error.response) {
                    console.error('Error response:', error.response.data);
                    console.error('Error status:', error.response.status);
                }
                throw error;
            }
        },
        updateMessage({ commit }, message: Message) {
            console.log('Updating message locally:', message);
            commit('updateMessage', message);
            return message;
        },
        async getMessages({ commit }, conversationId: string) {
            try {
                const response = await apiClient.getMessages(conversationId);
                const messages = response.data;
                if (Array.isArray(messages)) {
                    // Ensure that providerModel is included in each message
                    const processedMessages = messages.map(message => ({
                        ...message,
                        providerModel: message.providerModel || 'Unknown'
                    }));
                    commit('setMessages', processedMessages);
                } else {
                    console.error('Unexpected response format for messages:', messages);
                    commit('setMessages', []);
                }
                return messages;
            } catch (error) {
                console.error('Error fetching messages:', error);
                commit('setMessages', []);
                throw error;
            }
        },
        async switchConversation({ commit, dispatch }, conversationId: string) {
            try {
                commit('clearMessages');
                await dispatch('getConversation', conversationId);
                await dispatch('getMessages', conversationId);
            } catch (error) {
                console.error('Error switching conversation:', error);
                throw error;
            }
        },
        async createAttachment({ commit }, { messageId, fileType, filePath }: { messageId: string; fileType: string; filePath: string }) {
            try {
                const response = await apiClient.createAttachment(messageId, fileType, filePath);
                const attachment = response.data;
                commit('addAttachment', { messageId, attachment });
                return attachment;
            } catch (error) {
                console.error('Error creating attachment:', error);
                throw error;
            }
        },
        async getAttachments({ commit }, messageId: string) {
            try {
                const response = await apiClient.getAttachments(messageId);
                const attachments = response.data;
                commit('setAttachments', { messageId, attachments });
                return attachments;
            } catch (error) {
                console.error('Error fetching attachments:', error);
                throw error;
            }
        },
        async createLLMProvider({ commit }, { name, providerType, apiEndpoint, supportedModalities, configuration }: { name: string; providerType: string; apiEndpoint: string; supportedModalities: string[]; configuration: any }) {
            try {
                const providerData = {
                    name,
                    provider_type: providerType,
                    api_endpoint: apiEndpoint,
                    supported_modalities: supportedModalities,
                    configuration,
                };
                const response = await apiClient.createLLMProvider(providerData);
                const provider = response.data;
                commit('addLLMProvider', provider);
                return provider;
            } catch (error) {
                console.error('Error creating LLM provider:', error);
                throw error;
            }
        },
        async getLLMProvider({ commit }, id: string) {
            try {
                const response = await apiClient.getLLMProvider(id);
                const provider = response.data;
                commit('setLLMProvider', provider);
                return provider;
            } catch (error) {
                console.error('Error fetching LLM provider:', error);
                throw error;
            }
        },
        async getUserLLMConfigs({ commit }) {
            try {
                const response = await apiClient.listUserLLMConfigs();
                const configs = response.data;
                commit('setUserLLMConfigs', configs);
                return configs;
            } catch (error) {
                console.error('Error fetching user LLM configs:', error);
                throw error;
            }
        },
        async createUserLLMConfig({ commit, rootState }, { providerId, apiKeyId, description }: { providerId: string; apiKeyId: string; description?: string }) {
            try {
                const userId = rootState.auth.user?.user_id;
                if (!userId) {
                    throw new Error('User ID not found');
                }
                const configData = { user_id: userId, provider_id: providerId, api_key_id: apiKeyId, description };
                const response = await apiClient.createUserLLMConfig(configData);
                const config = response.data;
                commit('addUserLLMConfig', config);
                return config;
            } catch (error) {
                console.error('Error creating user LLM config:', error);
                throw error;
            }
        },
        async updateUserLLMConfig({ commit, rootState }, { id, providerId, apiKeyId, description }: { id: string; providerId: string; apiKeyId: string; description?: string }) {
            try {
                const userId = rootState.auth.user?.user_id;
                if (!userId) {
                    throw new Error('User ID not found');
                }
                const configData = { user_id: userId, provider_id: providerId, api_key_id: apiKeyId, description };
                const response = await apiClient.updateUserLLMConfig(id, configData);
                const updatedConfig = response.data;
                commit('updateUserLLMConfig', updatedConfig);
                return updatedConfig;
            } catch (error) {
                console.error('Error updating user LLM config:', error);
                throw error;
            }
        },
        async deleteUserLLMConfig({ commit }, id: string) {
            try {
                await apiClient.deleteUserLLMConfig(id);
                commit('removeUserLLMConfig', id);
            } catch (error) {
                console.error('Error deleting user LLM config:', error);
                throw error;
            }
        },
        async deleteMessage({ commit, state }, messageId: string) {
            try {
                if (!state.currentConversation) {
                    throw new Error('No active conversation');
                }
                console.log('Deleting message:', messageId, 'from conversation:', state.currentConversation.id);
                await apiClient.deleteMessage(state.currentConversation.id, messageId);
                commit('removeMessage', messageId);
            } catch (error) {
                console.error('Error deleting message:', error);
                throw error;
            }
        },
    },
    getters: {
        getConversationById: (state) => (id: string) => {
            return state.conversations.find(conversation => conversation.id === id);
        },
        getMessagesByConversationId: (state) => (conversationId: string) => {
            return state.messages.filter(message => message.conversationId === conversationId);
        },
        getAttachmentsByMessageId: (state) => (messageId: string) => {
            return state.attachments[messageId] || [];
        },
        getUserLLMConfigById: (state) => (id: string) => {
            return state.userLLMConfigs.find(config => config.id === id);
        },
        getCurrentConversationMessages: (state) => {
            const currentConversationId = state.currentConversation?.id;
            return currentConversationId
                ? state.messages.filter(message => message.conversationId === currentConversationId)
                : [];
        },
    },
};

export default chatModule;
