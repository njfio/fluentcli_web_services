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
        const url = new URL('/llm/stream_chat', axiosInstance.defaults.baseURL);

        // Filter out invalid messages
        const validMessages = messages.filter(msg => msg && msg.role && msg.content);

        // Add query parameters
        url.searchParams.append('provider_id', providerId);
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
                const lines = text.split('\n');

                for (const line of lines) {
                    if (line.startsWith('data: ')) {
                        const content = line.slice(6).trim();
                        if (content === '[DONE]') {
                            controller.enqueue(new TextEncoder().encode(content + '\n'));
                        } else {
                            controller.enqueue(new TextEncoder().encode(content + '\n'));
                        }
                    }
                }
            }
        });

        return response.body.pipeThrough(transformStream);
    }
}

export default new LLMService();
