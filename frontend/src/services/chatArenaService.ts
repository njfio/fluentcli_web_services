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
        const messages: Map<string, ArenaMessage> = new Map(); // Use Map to ensure uniqueness by configId
        const decoder = new TextDecoder();
        const finalMessages: ArenaMessage[] = [];

        const streamPromises = streams.map(async (stream, index) => {
            const reader = stream.getReader();
            let fullContent = '';

            try {
                while (true) {
                    const { done, value } = await reader.read();
                    if (done) break;

                    const chunk = decoder.decode(value);
                    fullContent += chunk;
                }

                if (fullContent.trim()) {
                    // Save final message to backend first
                    const response = await this.createArenaMessage(
                        conversationId,
                        'assistant',
                        fullContent,
                        providerModels[index],
                        fullContent // raw output
                    );

                    // Create final message with backend data
                    const savedMessage = response.data;
                    const finalMessage: ArenaMessage = {
                        ...savedMessage,
                        configId: configIds[index],
                        providerModel: providerModels[index]
                    };

                    // Store in Map to ensure uniqueness
                    messages.set(configIds[index], finalMessage);
                }
            } catch (error) {
                console.error(`Error processing stream for config ${configIds[index]}:`, error);
            } finally {
                reader.releaseLock();
            }
        });

        // Wait for all streams to complete
        await Promise.all(streamPromises);

        // Convert Map to array, maintaining order based on original configIds
        for (const configId of configIds) {
            const message = messages.get(configId);
            if (message) {
                finalMessages.push(message);
            }
        }

        return finalMessages;
    }
}

export default new ChatArenaService();
