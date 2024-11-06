import { axiosInstance } from './apiClient';

export interface ComputerUseMessage {
    role: 'user' | 'assistant';
    content: string;
}

class ComputerUseService {
    async chat(messages: ComputerUseMessage[]): Promise<ReadableStream<Uint8Array>> {
        const baseUrl = axiosInstance.defaults.baseURL?.replace(/\/$/, '');
        const response = await fetch(`${baseUrl}/computer-use/chat`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('token')}`,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                messages: messages.map(msg => ({
                    role: msg.role,
                    content: msg.content
                }))
            })
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
        }

        if (!response.body) {
            throw new Error('No response body');
        }

        return response.body;
    }
}

export default new ComputerUseService();
