import { Module } from 'vuex';
import { RootState } from '../types';
import apiClient from '../../services/apiClient';
import { AxiosError } from 'axios';

export interface Conversation {
    id: string;
    title: string;
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
            state.messages = messages;
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
        async createConversation({ commit, rootState }, title: string) {
            try {
                const userId = rootState.auth.user?.user_id;
                if (!userId) {
                    throw new Error('User ID not found');
                }
                const response = await apiClient.createConversation({ user_id: userId, title });
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
        async createMessage({ commit }, { conversationId, role, content }: { conversationId: string; role: string; content: string }) {
            try {
                const response = await apiClient.createMessage(conversationId, role, content);
                const message = response.data;
                commit('addMessage', message);
                return message;
            } catch (error) {
                console.error('Error creating message:', error);
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
                commit('setMessages', messages);
                return messages;
            } catch (error) {
                console.error('Error fetching messages:', error);
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
        async createUserLLMConfig({ commit, rootState }, { providerId, apiKeyId }: { providerId: string; apiKeyId: string }) {
            try {
                const userId = rootState.auth.user?.user_id;
                if (!userId) {
                    throw new Error('User ID not found');
                }
                const configData = { user_id: userId, provider_id: providerId, api_key_id: apiKeyId };
                const response = await apiClient.createUserLLMConfig(configData);
                const config = response.data;
                commit('addUserLLMConfig', config);
                return config;
            } catch (error) {
                console.error('Error creating user LLM config:', error);
                throw error;
            }
        },
        async updateUserLLMConfig({ commit, rootState }, { id, providerId, apiKeyId }: { id: string; providerId: string; apiKeyId: string }) {
            try {
                const userId = rootState.auth.user?.user_id;
                if (!userId) {
                    throw new Error('User ID not found');
                }
                const configData = { user_id: userId, provider_id: providerId, api_key_id: apiKeyId };
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
    },
};

export default chatModule;
