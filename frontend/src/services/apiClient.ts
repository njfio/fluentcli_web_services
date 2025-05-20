import axios, { AxiosInstance, AxiosResponse } from 'axios';
import { API_URL } from '../config';
import AuthService from './AuthService';
import { StudioConfiguration } from '../store/modules/studio';
import store from '../store';

export const axiosInstance: AxiosInstance = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
    Accept: 'application/json',
  },
  withCredentials: true,
});

axiosInstance.interceptors.request.use(
  (config) => {
    const token = AuthService.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
      const userId = store.getters.userId;
      if (userId) {
        config.headers['X-User-ID'] = userId;
      }
    }
    console.log(`Making ${config.method?.toUpperCase()} request to ${config.url}`);
    return config;
  },
  (error) => {
    console.error('Request interceptor error:', error);
    return Promise.reject(error);
  }
);

axiosInstance.interceptors.response.use(
  (response) => {
    console.log(`Received response from ${response.config.url} with status ${response.status}`);
    return response;
  },
  (error) => {
    console.error('API Error:', error);
    if (error.response && error.response.status === 401) {
      console.log('Unauthorized access detected, logging out...');
      AuthService.logout();
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

interface ApiClient {
  // User routes
  validateToken: () => Promise<AxiosResponse<any>>;
  refreshToken: () => Promise<AxiosResponse<any>>;
  createUser: (userData: any) => Promise<AxiosResponse<any>>;
  listUsers: () => Promise<AxiosResponse<any>>;
  getUser: (id: string) => Promise<AxiosResponse<any>>;
  updateUser: (id: string, userData: any) => Promise<AxiosResponse<any>>;
  deleteUser: (id: string) => Promise<AxiosResponse<any>>;
  login: (credentials: any) => Promise<AxiosResponse<any>>;

  // Job routes
  createJob: (jobData: any) => Promise<AxiosResponse<any>>;
  listJobs: () => Promise<AxiosResponse<any>>;
  getJob: (id: string) => Promise<AxiosResponse<any>>;
  updateJob: (id: string, jobData: any) => Promise<AxiosResponse<any>>;
  deleteJob: (id: string) => Promise<AxiosResponse<any>>;
  getJobData: (id: string) => Promise<AxiosResponse<any>>;
  getJobLogs: (id: string) => Promise<AxiosResponse<any>>;
  fetchJobData: (id: string) => Promise<AxiosResponse<any>>;
  fetchJobLogs: (id: string) => Promise<AxiosResponse<any>>;
  fetchJobs: () => Promise<AxiosResponse<any>>;
  startJob: (id: string) => Promise<AxiosResponse<any>>;
  stopJob: (id: string) => Promise<AxiosResponse<any>>;

  // Pipeline routes
  createPipeline: (pipelineData: any) => Promise<AxiosResponse<any>>;
  listPipelines: () => Promise<AxiosResponse<any>>;
  getPipeline: (id: string) => Promise<AxiosResponse<any>>;
  updatePipeline: (id: string, pipelineData: any) => Promise<AxiosResponse<any>>;
  deletePipeline: (id: string) => Promise<AxiosResponse<any>>;
  fetchPipelines: () => Promise<AxiosResponse<any>>;

  // Docker File routes
  createDockerFile: (dockerFileData: any) => Promise<AxiosResponse<any>>;
  listDockerFiles: () => Promise<AxiosResponse<any>>;
  getDockerFile: (id: string) => Promise<AxiosResponse<any>>;
  updateDockerFile: (id: string, dockerFileData: any) => Promise<AxiosResponse<any>>;
  deleteDockerFile: (id: string) => Promise<AxiosResponse<any>>;
  fetchDockerFiles: () => Promise<AxiosResponse<any>>;

  // Configuration routes
  createConfiguration: (configurationData: any) => Promise<AxiosResponse<StudioConfiguration>>;
  listConfigurations: () => Promise<AxiosResponse<StudioConfiguration[]>>;
  getConfiguration: (id: string) => Promise<AxiosResponse<StudioConfiguration>>;
  updateConfiguration: (id: string, configurationData: any) => Promise<AxiosResponse<StudioConfiguration>>;
  deleteConfiguration: (id: string) => Promise<AxiosResponse<void>>;
  fetchConfigurations: () => Promise<AxiosResponse<StudioConfiguration[]>>;

  // Amber Store routes
  createAmberStore: (amberStoreData: any) => Promise<AxiosResponse<any>>;
  listAmberStores: () => Promise<AxiosResponse<any>>;
  getAmberStore: (id: string) => Promise<AxiosResponse<any>>;
  updateAmberStore: (id: string, amberStoreData: any) => Promise<AxiosResponse<any>>;
  deleteAmberStore: (id: string) => Promise<AxiosResponse<any>>;
  fetchAmberStores: () => Promise<AxiosResponse<any>>;

  // Chat routes
  createConversation: (data: any) => Promise<AxiosResponse<any>>;
  listConversations: () => Promise<AxiosResponse<any>>;
  getConversation: (id: string) => Promise<AxiosResponse<any>>;
  deleteConversation: (id: string) => Promise<AxiosResponse<any>>;
  createMessage: (conversationId: string, role: string, content: string, providerModel: string, attachmentId?: string, rawOutput?: string, usageStats?: any) => Promise<AxiosResponse<any>>;
  getMessages: (conversationId: string) => Promise<AxiosResponse<any>>;
  deleteMessage: (conversationId: string, messageId: string) => Promise<AxiosResponse<any>>;

  // Attachment routes
  createAttachment: (messageId: string, fileType: string, filePath: string) => Promise<AxiosResponse<any>>;
  getAttachment: (attachmentId: string) => Promise<AxiosResponse<any>>;
  getAttachments: (messageId: string) => Promise<AxiosResponse<any>>;
  deleteAttachment: (attachmentId: string) => Promise<AxiosResponse<void>>;

  // LLM Provider routes
  createLLMProvider: (providerData: any) => Promise<AxiosResponse<any>>;
  updateLLMProvider: (id: string, providerData: any) => Promise<AxiosResponse<any>>;
  listLLMProviders: () => Promise<AxiosResponse<any>>;
  getLLMProvider: (id: string) => Promise<AxiosResponse<any>>;
  deleteLLMProvider: (id: string) => Promise<AxiosResponse<any>>;

  // User LLM Config routes
  createUserLLMConfig: (configData: any) => Promise<AxiosResponse<any>>;
  updateUserLLMConfig: (id: string, configData: any) => Promise<AxiosResponse<any>>;
  listUserLLMConfigs: () => Promise<AxiosResponse<any>>;
  getUserLLMConfig: (id: string) => Promise<AxiosResponse<any>>;
  deleteUserLLMConfig: (id: string) => Promise<AxiosResponse<any>>;

  // LLM Chat routes
  llmChat: (providerId: string, messages: any[]) => Promise<AxiosResponse<any>>;
  streamChat: (providerId: string, messages: any[]) => Promise<AxiosResponse<any>>;

  // API Key routes
  createApiKey: (key_value: string, description: string) => Promise<AxiosResponse<any>>;
  listApiKeys: () => Promise<AxiosResponse<any>>;
  getApiKey: (id: string) => Promise<AxiosResponse<any>>;
  updateApiKey: (id: string, key_value: string, description: string) => Promise<AxiosResponse<any>>;
  deleteApiKey: (id: string) => Promise<AxiosResponse<any>>;

  // LLM Templates and Unified Config routes
  getLLMTemplates: () => Promise<AxiosResponse<any>>;
  getLLMTemplate: (id: string) => Promise<AxiosResponse<any>>;
  createUnifiedLLMConfig: (configData: any) => Promise<AxiosResponse<any>>;
  listUnifiedLLMConfigs: () => Promise<AxiosResponse<any>>;
  deleteUnifiedLLMConfig: (id: string) => Promise<AxiosResponse<any>>;
}

const apiClient: ApiClient = {
  // User routes
  validateToken: () => axiosInstance.get('/users/validate-token'),
  refreshToken: () => axiosInstance.post('/users/refresh'),
  createUser: (userData) => axiosInstance.post('/users', userData),
  listUsers: () => axiosInstance.get('/users'),
  getUser: (id) => axiosInstance.get(`/users/${id}`),
  updateUser: (id, userData) => axiosInstance.put(`/users/${id}`, userData),
  deleteUser: (id) => axiosInstance.delete(`/users/${id}`),
  login: (credentials) => axiosInstance.post('/users/login', credentials),

  // Job routes
  createJob: (jobData) => axiosInstance.post('/jobs', jobData),
  listJobs: () => axiosInstance.get('/jobs'),
  getJob: (id) => axiosInstance.get(`/jobs/${id}`),
  updateJob: (id, jobData) => axiosInstance.put(`/jobs/${id}`, jobData),
  deleteJob: (id) => axiosInstance.delete(`/jobs/${id}`),
  getJobData: (id) => axiosInstance.get(`/jobs/${id}/data`),
  getJobLogs: (id) => axiosInstance.get(`/jobs/${id}/logs`),
  fetchJobData: (id) => axiosInstance.get(`/jobs/${id}/data`),
  fetchJobLogs: (id) => axiosInstance.get(`/jobs/${id}/logs`),
  fetchJobs: () => axiosInstance.get('/jobs'),
  startJob: (id) => axiosInstance.post(`/jobs/${id}/start`),
  stopJob: (id) => axiosInstance.post(`/jobs/${id}/stop`),

  // Pipeline routes
  createPipeline: (pipelineData) => axiosInstance.post('/pipelines', pipelineData),
  listPipelines: () => axiosInstance.get('/pipelines'),
  getPipeline: (id) => axiosInstance.get(`/pipelines/${id}`),
  updatePipeline: (id, pipelineData) => axiosInstance.put(`/pipelines/${id}`, pipelineData),
  deletePipeline: (id) => axiosInstance.delete(`/pipelines/${id}`),
  fetchPipelines: () => axiosInstance.get('/pipelines'),

  // Docker File routes
  createDockerFile: (dockerFileData) => axiosInstance.post('/docker_files', dockerFileData),
  listDockerFiles: () => axiosInstance.get('/docker_files'),
  getDockerFile: (id) => axiosInstance.get(`/docker_files/${id}`),
  updateDockerFile: (id, dockerFileData) => axiosInstance.put(`/docker_files/${id}`, dockerFileData),
  deleteDockerFile: (id) => axiosInstance.delete(`/docker_files/${id}`),
  fetchDockerFiles: () => axiosInstance.get('/docker_files'),

  // Configuration routes
  createConfiguration: (configurationData) => axiosInstance.post('/configurations', configurationData),
  listConfigurations: () => axiosInstance.get('/configurations'),
  getConfiguration: (id) => axiosInstance.get(`/configurations/${id}`),
  updateConfiguration: (id, configurationData) => axiosInstance.put(`/configurations/${id}`, configurationData),
  deleteConfiguration: (id) => axiosInstance.delete(`/configurations/${id}`),
  fetchConfigurations: () => axiosInstance.get('/configurations'),

  // Amber Store routes
  createAmberStore: (amberStoreData) => axiosInstance.post('/amber_stores', amberStoreData),
  listAmberStores: () => axiosInstance.get('/amber_stores'),
  getAmberStore: (id) => axiosInstance.get(`/amber_stores/${id}`),
  updateAmberStore: (id, amberStoreData) => axiosInstance.put(`/amber_stores/${id}`, amberStoreData),
  deleteAmberStore: (id) => axiosInstance.delete(`/amber_stores/${id}`),
  fetchAmberStores: () => axiosInstance.get('/amber_stores'),

  // Chat routes
  createConversation: (data) => axiosInstance.post('/chat/conversations', data),
  listConversations: () => axiosInstance.get('/chat/conversations'),
  getConversation: (id) => axiosInstance.get(`/chat/conversations/${id}`),
  deleteConversation: (id) => axiosInstance.delete(`/chat/conversations/${id}`),
  createMessage: (conversationId, role, content, providerModel, attachmentId, rawOutput, usageStats) => {
    // Create the request payload without the attachment_id field if it's undefined
    const payload: any = {
      conversation_id: conversationId,
      role,
      content,
      provider_model: providerModel,
      raw_output: rawOutput,
      usage_stats: usageStats,
    };

    // Only add attachment_id if it's defined
    if (attachmentId !== undefined) {
      payload.attachment_id = attachmentId;
    }

    console.log('Creating message with payload:', payload);
    return axiosInstance.post('/chat/messages', payload);
  },
  getMessages: (conversationId) => axiosInstance.get(`/chat/conversations/${conversationId}/messages`),
  deleteMessage: (conversationId, messageId) => axiosInstance.delete(`/chat/conversations/${conversationId}/messages/${messageId}`),

  // Attachment routes
  createAttachment: (messageId, fileType, filePath) =>
    axiosInstance.post('/chat/attachments', { message_id: messageId, file_type: fileType, file_path: filePath }),
  getAttachment: (attachmentId) => axiosInstance.get(`/chat/attachments/${attachmentId}`, { responseType: 'arraybuffer' }),
  getAttachments: (messageId) => axiosInstance.get(`/chat/messages/${messageId}/attachments`),
  deleteAttachment: (attachmentId) => axiosInstance.delete(`/chat/attachments/${attachmentId}`),

  // LLM Provider routes
  createLLMProvider: (providerData) => axiosInstance.post('/llm/providers', providerData),
  updateLLMProvider: (id, providerData) => axiosInstance.put(`/llm/providers/${id}`, providerData),
  listLLMProviders: () => axiosInstance.get('/llm/providers'),
  getLLMProvider: (id) => axiosInstance.get(`/llm/providers/${id}`),
  deleteLLMProvider: (id) => axiosInstance.delete(`/llm/providers/${id}`),

  // User LLM Config routes
  createUserLLMConfig: (configData) => axiosInstance.post('/llm/user-configs', configData),
  updateUserLLMConfig: (id, configData) => axiosInstance.put(`/llm/user-configs/${id}`, configData),
  listUserLLMConfigs: () => axiosInstance.get('/llm/user-configs'),
  getUserLLMConfig: (id) => axiosInstance.get(`/llm/user-configs/${id}`),
  deleteUserLLMConfig: (id) => axiosInstance.delete(`/llm/user-configs/${id}`),

  // LLM Chat routes
  llmChat: (providerId, messages) => axiosInstance.post('/llm/chat', { provider_id: providerId, messages }),
  streamChat: (providerId, messages) => axiosInstance.get('/llm/stream_chat', {
    params: { provider_id: providerId, messages: JSON.stringify(messages) }
  }),

  // API Key routes
  createApiKey: (key_value, description) => axiosInstance.post('/api_keys', { key_value, description }),
  listApiKeys: () => axiosInstance.get('/api_keys'),
  getApiKey: (id) => axiosInstance.get(`/api_keys/${id}`),
  updateApiKey: (id, key_value, description) => axiosInstance.put(`/api_keys/${id}`, { key_value, description }),
  deleteApiKey: (id) => axiosInstance.delete(`/api_keys/${id}`),

  // LLM Templates and Unified Config routes
  getLLMTemplates: () => axiosInstance.get('/llm/templates'),
  getLLMTemplate: (id) => axiosInstance.get(`/llm/templates/${id}`),
  createUnifiedLLMConfig: (configData) => axiosInstance.post('/llm/unified-configs', configData),
  listUnifiedLLMConfigs: () => axiosInstance.get('/llm/unified-configs'),
  deleteUnifiedLLMConfig: (id) => axiosInstance.delete(`/llm/unified-configs/${id}`),
};

export default apiClient;
