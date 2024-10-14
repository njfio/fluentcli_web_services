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
        const url = new URL('/chat/stream', axiosInstance.defaults.baseURL);

        // Filter out invalid messages
        const validMessages = messages.filter(msg => msg && msg.role && msg.content);

        // Add query parameters
        url.searchParams.append('provider_id', providerId);
        url.searchParams.append('messages', JSON.stringify(validMessages));

        console.log('Request URL:', url.toString());
        console.log('Valid Messages:', JSON.stringify(validMessages, null, 2));

        const response = await fetch(url.toString(), {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('token')}`,
            },
        });

        if (!response.ok) {
            const errorText = await response.text();
            console.error('Error response:', errorText);
            console.error('Response status:', response.status);
            console.error('Response headers:', JSON.stringify(Object.fromEntries(response.headers.entries()), null, 2));
            throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
        }

        if (!response.body) {
            throw new Error('No response body');
        }

        // Create a TransformStream to handle the response and remove duplicates
        const transformStream = new TransformStream({
            transform: (chunk, controller) => {
                const text = new TextDecoder().decode(chunk);
                const lines = text.split('\n');
                let buffer = '';

                for (const line of lines) {
                    if (line.startsWith('data: ')) {
                        const jsonStr = line.slice(6).trim();
                        if (jsonStr === '[DONE]') {
                            controller.enqueue(new TextEncoder().encode(line + '\n'));
                            continue;
                        }
                        buffer += jsonStr;
                        try {
                            const parsedChunk = JSON.parse(buffer);
                            console.log('Parsed chunk:', parsedChunk);
                            if (parsedChunk.choices && parsedChunk.choices[0].delta.content) {
                                controller.enqueue(new TextEncoder().encode(line + '\n'));
                            }
                            buffer = '';
                        } catch (e) {
                            // If parsing fails, it might be an incomplete chunk. We'll keep it in the buffer.
                            console.log('Incomplete chunk, buffering:', buffer);
                        }
                    } else {
                        controller.enqueue(new TextEncoder().encode(line + '\n'));
                    }
                }
            }
        });

        return response.body.pipeThrough(transformStream);
    }
}

export default new LLMService();
