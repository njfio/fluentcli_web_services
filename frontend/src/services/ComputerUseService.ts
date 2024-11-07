import { axiosInstance } from './apiClient';

export interface ComputerUseMessage {
    role: 'user' | 'assistant';
    content: string;
}

class ComputerUseService {
    async chat(messages: ComputerUseMessage[]) {
        const response = await axiosInstance.post('/computer-use/chat', {
            messages: messages.map(msg => ({
                role: msg.role,
                content: msg.content
            }))
        }, {
            responseType: 'text',
            headers: {
                'Accept': 'text/event-stream',
                'Cache-Control': 'no-cache',
            }
        });

        return response.data;
    }
}

export default new ComputerUseService();
