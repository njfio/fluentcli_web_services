import { axiosInstance } from './apiClient';

export interface LLMProvider {
    id: string;
    name: string;
    apiEndpoint: string;
}

export interface LLMMessage {
    role: 'system' | 'user' | 'assistant';
    content: string;
}

class LLMService {
    async getProviders(): Promise<LLMProvider[]> {
        const response = await axiosInstance.get('/llm/providers');
        return response.data;
    }

    async chat(providerId: string, messages: LLMMessage[]): Promise<string> {
        const response = await axiosInstance.post('/llm/chat', {
            provider_id: providerId,
            messages,
        });
        return response.data.message;
    }

    async streamChat(providerId: string, messages: LLMMessage[]): Promise<ReadableStream<Uint8Array>> {
        const url = new URL('/llm/stream-chat', axiosInstance.defaults.baseURL);
        const response = await fetch(url.toString(), {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${localStorage.getItem('token')}`,
            },
            body: JSON.stringify({
                provider_id: providerId,
                messages,
            }),
        });

        if (!response.body) {
            throw new Error('No response body');
        }

        return response.body;
    }
}

export default new LLMService();
