import { axiosInstance } from './apiClient';

export interface LLMProvider {
    id: string;
    name: string;
    apiEndpoint: string;
}

export interface UserLLMConfig {
    id: string;
    name?: string;
    user_id: string;
    provider_id: string;
    api_key_id: string;
}

export interface LLMMessage {
    role: 'system' | 'user' | 'assistant';
    content: string;
}

class LLMService {
    async getUserLLMConfigs(): Promise<UserLLMConfig[]> {
        const response = await axiosInstance.get('/llm/user-configs');
        return response.data;
    }

    async getUserLLMConfig(configId: string): Promise<UserLLMConfig> {
        const response = await axiosInstance.get(`/llm/user-configs/${configId}`);
        return response.data;
    }

    async chat(userLLMConfigId: string, messages: LLMMessage[]): Promise<string> {
        const response = await axiosInstance.post('/llm/chat', {
            user_llm_config_id: userLLMConfigId,
            messages,
        });
        return response.data.message;
    }

    async streamChat(userLLMConfigId: string, conversationId: string, messages: LLMMessage[]): Promise<ReadableStream<Uint8Array>> {
        const userLLMConfig = await this.getUserLLMConfig(userLLMConfigId);
        console.log('User LLM Config:', JSON.stringify(userLLMConfig, null, 2));

        if (!userLLMConfig.provider_id) {
            console.error('Provider ID is missing from the user LLM config');
            throw new Error('Provider ID is missing from the user LLM config');
        }

        const url = new URL('/llm/stream_chat', axiosInstance.defaults.baseURL);

        // Filter out invalid messages
        const validMessages = messages.filter(msg => msg && msg.role && msg.content);

        // Add query parameters
        url.searchParams.append('user_llm_config_id', userLLMConfigId);
        url.searchParams.append('provider_id', userLLMConfig.provider_id);
        url.searchParams.append('conversation_id', conversationId);
        url.searchParams.append('messages', JSON.stringify(validMessages));

        console.log('LLMService streamChat - Request URL:', url.toString());
        console.log('LLMService streamChat - Valid Messages:', JSON.stringify(validMessages, null, 2));

        const response = await fetch(url.toString(), {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('token')}`,
            },
        });

        if (!response.ok) {
            const errorText = await response.text();
            console.error('LLMService streamChat - Error response:', errorText);
            console.error('LLMService streamChat - Response status:', response.status);
            console.error('LLMService streamChat - Response headers:', JSON.stringify(Object.fromEntries(response.headers.entries()), null, 2));
            throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
        }

        if (!response.body) {
            throw new Error('No response body');
        }

        // Create a TransformStream to handle the response
        const transformStream = new TransformStream({
            transform: (chunk, controller) => {
                const text = new TextDecoder().decode(chunk);
                controller.enqueue(new TextEncoder().encode(text));
            }
        });

        return response.body.pipeThrough(transformStream);
    }
}

export default new LLMService();
