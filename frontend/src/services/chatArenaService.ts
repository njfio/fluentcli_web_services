import { Message } from '../store/modules/chat';
import apiClient from './apiClient';
import LLMService from './LLMService';
import { AxiosResponse } from 'axios';

export interface ArenaMessage extends Message {
    configId: string;
}

class ChatArenaService {
    async streamChatToMultipleLLMs(
        conversationId: string,
        userInput: string,
        configIds: string[],
        messages: Message[]
    ): Promise<ReadableStream<Uint8Array>[]> {
        const streams: ReadableStream<Uint8Array>[] = [];

        // Convert messages to LLM format
        const llmMessages = messages.map(m => ({
            role: m.role as 'system' | 'user' | 'assistant',
            content: m.content,
        }));

        // Add the new user input
        llmMessages.push({
            role: 'user',
            content: userInput,
        });

        // Create a stream for each selected LLM config
        for (const configId of configIds) {
            try {
                const stream = await LLMService.streamChat(
                    configId,
                    conversationId,
                    llmMessages
                );
                streams.push(stream);
            } catch (error) {
                console.error(`Error streaming to LLM config ${configId}:`, error);
                // Continue with other streams even if one fails
            }
        }

        return streams;
    }

    async createArenaConversation(userId: string, title: string): Promise<AxiosResponse> {
        return apiClient.createConversation({
            user_id: userId,
            title,
            mode: 'arena'
        });
    }

    async createArenaMessage(
        conversationId: string,
        role: string,
        content: string,
        providerModel: string,
        rawOutput?: string,
        usageStats?: any
    ): Promise<AxiosResponse> {
        return apiClient.createMessage(
            conversationId,
            role,
            content,
            providerModel,
            undefined, // attachmentId
            rawOutput,
            usageStats
        );
    }

    async processArenaResponses(
        streams: ReadableStream<Uint8Array>[],
        conversationId: string,
        configIds: string[],
        providerModels: string[]
    ): Promise<ArenaMessage[]> {
        const messages: ArenaMessage[] = [];
        const decoder = new TextDecoder();

        const streamPromises = streams.map(async (stream, index) => {
            const reader = stream.getReader();
            let fullContent = '';

            try {
                while (true) {
                    const { done, value } = await reader.read();
                    if (done) break;

                    const chunk = decoder.decode(value);
                    fullContent += chunk;

                    // Create or update message for this stream
                    const message: ArenaMessage = {
                        id: '', // Will be set when saved to backend
                        conversationId,
                        role: 'assistant',
                        content: fullContent,
                        providerModel: providerModels[index],
                        configId: configIds[index],
                        createdAt: new Date().toISOString(),
                    };

                    // Replace existing message or add new one
                    const existingIndex = messages.findIndex(m => m.configId === configIds[index]);
                    if (existingIndex !== -1) {
                        messages[existingIndex] = message;
                    } else {
                        messages.push(message);
                    }
                }

                // Save final message to backend
                if (fullContent.trim()) {
                    const response = await this.createArenaMessage(
                        conversationId,
                        'assistant',
                        fullContent,
                        providerModels[index],
                        fullContent // raw output
                    );

                    // Update message with backend-generated ID
                    const savedMessage = response.data;
                    const messageIndex = messages.findIndex(m => m.configId === configIds[index]);
                    if (messageIndex !== -1) {
                        messages[messageIndex] = {
                            ...messages[messageIndex],
                            id: savedMessage.id,
                        };
                    }
                }
            } catch (error) {
                console.error(`Error processing stream for config ${configIds[index]}:`, error);
            } finally {
                reader.releaseLock();
            }
        });

        await Promise.all(streamPromises);
        return messages;
    }
}

export default new ChatArenaService();
