import { ref, computed, watch } from 'vue';
import { useStore } from 'vuex';
import LLMService, { LLMMessage } from '../../services/LLMService';
import { Message, UserLLMConfig } from '../../store/modules/chat';
import DOMPurify from 'dompurify';
import { marked } from 'marked';
import hljs from 'highlight.js';

// Configure marked options for GitHub Flavored Markdown
(marked as any).setOptions({
    gfm: true,
    breaks: true,
    headerIds: false,
    highlight: function (code: string, lang: string) {
        if (lang && hljs.getLanguage(lang)) {
            try {
                return hljs.highlight(code, { language: lang }).value;
            } catch (__) { }
        }
        return hljs.highlightAuto(code).value;
    }
});

export function useChatLogic() {
    const store = useStore();
    const userInput = ref('');
    const chatMessages = ref<HTMLElement | null>(null);
    const isLoading = ref(false);
    const error = ref('');
    let abortController: AbortController | null = null;
    let retryCount = 0;
    const maxRetries = 3;

    const conversations = computed(() => store.state.chat.conversations);
    const currentConversation = computed(() => store.state.chat.currentConversation);
    const messages = computed(() => store.state.chat.messages);
    const currentMessages = ref<Message[]>([]);

    const userLLMConfigs = computed(() => store.state.chat.userLLMConfigs);
    const selectedConfigId = ref<string>('');

    watch(currentMessages, () => {
        console.log('Current Messages updated:', JSON.stringify(currentMessages.value, null, 2));
        scrollToBottom();
    }, { deep: true });

    async function loadMessages(conversationId: string) {
        await store.dispatch('chat/getMessages', conversationId);
        currentMessages.value = await Promise.all(messages.value
            .filter((m: Message | null): m is Message => m !== null)
            .map(async (m: Message) => ({
                ...m,
                renderedContent: await renderMarkdown(m.content)
            })));
        console.log('Messages loaded:', JSON.stringify(currentMessages.value, null, 2));
    }

    async function selectConversation(conversationId: string) {
        await store.dispatch('chat/getConversation', conversationId);
        await loadMessages(conversationId);
    }

    async function createNewConversation() {
        const title = prompt('Enter conversation title:');
        if (title) {
            try {
                const newConversation = await store.dispatch('chat/createConversation', title);
                await selectConversation(newConversation.id);
            } catch (err) {
                console.error('Error creating new conversation:', err);
                error.value = 'Failed to create a new conversation. Please try again.';
            }
        }
    }

    async function deleteConversation(conversationId: string) {
        try {
            await store.dispatch('chat/deleteConversation', conversationId);
            if (currentConversation.value && currentConversation.value.id === conversationId) {
                currentMessages.value = [];
            }
        } catch (err) {
            console.error('Error deleting conversation:', err);
            error.value = 'Failed to delete conversation. Please try again.';
        }
    }

    async function sendMessage() {
        if (userInput.value.trim() === '' || isLoading.value || !currentConversation.value || !selectedConfigId.value) return;
        await processMessage(userInput.value);
    }

    function extractMessage(result: any): Message | null {
        if (Array.isArray(result) && result.length > 0 && typeof result[0] === 'object' && 'id' in result[0]) {
            return result[0] as Message;
        } else if (typeof result === 'object' && 'id' in result) {
            return result as Message;
        }
        console.error('Invalid message structure:', result);
        return null;
    }

    async function processMessage(message: string, retry = false) {
        console.log('Processing message:', message);
        if (!message || !selectedConfigId.value || !currentConversation.value) {
            error.value = message ? 'Please select a User LLM Config and ensure a conversation is active before sending a message.' : 'Cannot process empty message.';
            return;
        }

        error.value = '';
        isLoading.value = true;

        try {
            if (abortController) {
                abortController.abort();
            }

            abortController = new AbortController();

            let userMessage: Message | null = null;
            if (!retry && currentConversation.value) {
                const selectedConfig = userLLMConfigs.value.find((config: UserLLMConfig) => config.id === selectedConfigId.value);
                if (!selectedConfig) {
                    throw new Error('Selected User LLM Config not found');
                }

                console.log('Selected config:', JSON.stringify(selectedConfig, null, 2));
                console.log('Provider ID:', selectedConfig.provider_id);

                if (!selectedConfig.provider_id) {
                    throw new Error('Provider ID is undefined');
                }

                // Fetch the LLM provider to get the provider name
                const provider = await store.dispatch('chat/getLLMProvider', selectedConfig.provider_id);
                console.log('Fetched provider:', provider);
                const providerName = provider.name;

                const result = await store.dispatch('chat/createMessage', {
                    conversationId: currentConversation.value.id,
                    role: 'user',
                    content: message,
                    providerModel: providerName,
                    attachmentId: undefined,
                    rawOutput: undefined,
                    usageStats: undefined
                });
                console.log('User message created:', result);
                userMessage = extractMessage(result);
                if (userMessage) {
                    userMessage.renderedContent = await renderMarkdown(userMessage.content);
                    currentMessages.value.push(userMessage);
                    userInput.value = '';
                }
            }

            const llmMessages: LLMMessage[] = currentMessages.value
                .filter(m => m && typeof m === 'object' && 'role' in m && 'content' in m)
                .map(m => ({
                    role: m.role as 'system' | 'user' | 'assistant',
                    content: m.content,
                }));

            if (llmMessages.length === 0) {
                throw new Error('No valid messages to send to LLM');
            }

            console.log('Sending request to LLM service...');
            const stream = await LLMService.streamChat(selectedConfigId.value, currentConversation.value.id, llmMessages);
            console.log('Received stream from LLM service');
            const reader = stream.getReader();
            const decoder = new TextDecoder();

            let assistantMessage: Message | null = null;
            let fullContent = '';

            while (true) {
                const { done, value } = await reader.read();
                if (done) {
                    console.log('Stream reading completed');
                    break;
                }

                const chunk = decoder.decode(value);
                console.log('Received chunk:', chunk);
                fullContent += chunk;

                if (!assistantMessage && fullContent.trim() !== '') {
                    assistantMessage = {
                        id: '',
                        conversationId: currentConversation.value!.id,
                        role: 'assistant',
                        content: fullContent,
                        providerModel: '',
                        attachmentId: undefined,
                        rawOutput: undefined,
                        usageStats: undefined,
                        createdAt: new Date().toISOString(),
                        renderedContent: await renderMarkdown(fullContent)
                    };
                    currentMessages.value.push(assistantMessage);
                } else if (assistantMessage) {
                    assistantMessage.content = fullContent;
                    assistantMessage.renderedContent = await renderMarkdown(fullContent);
                }

                // Update the last message in the array
                const lastIndex = currentMessages.value.length - 1;
                if (lastIndex >= 0 && assistantMessage) {
                    currentMessages.value = [
                        ...currentMessages.value.slice(0, lastIndex),
                        { ...assistantMessage }
                    ];
                }
                console.log('Updated assistant message:', JSON.stringify(assistantMessage, null, 2));
            }

            // Create the final assistant message on the backend
            if (assistantMessage && fullContent.trim() !== '') {
                const selectedConfig = userLLMConfigs.value.find((config: UserLLMConfig) => config.id === selectedConfigId.value);
                if (!selectedConfig) {
                    throw new Error('Selected User LLM Config not found');
                }
                const provider = await store.dispatch('chat/getLLMProvider', selectedConfig.provider_id);
                const providerName = provider.name;

                try {
                    const newMessageResult = await store.dispatch('chat/createMessage', {
                        conversationId: currentConversation.value!.id,
                        role: 'assistant',
                        content: fullContent,
                        providerModel: providerName,
                        attachmentId: undefined,
                        rawOutput: fullContent,
                        usageStats: undefined
                    });
                    console.log('Final assistant message created:', newMessageResult);
                    const createdMessage = extractMessage(newMessageResult);
                    if (createdMessage) {
                        createdMessage.renderedContent = await renderMarkdown(createdMessage.content);
                        // Update the message in currentMessages
                        const lastIndex = currentMessages.value.length - 1;
                        if (lastIndex >= 0) {
                            currentMessages.value = [
                                ...currentMessages.value.slice(0, lastIndex),
                                createdMessage
                            ];
                        }
                    }
                } catch (error) {
                    console.error('Error creating assistant message:', error);
                    throw error;
                }
            }

            console.log('Message processing completed');
            isLoading.value = false;
            retryCount = 0;

        } catch (err: any) {
            console.error('Error sending message:', err);
            handleError(err);
        }
    }

    function handleError(err: any) {
        if (retryCount < maxRetries) {
            retryCount++;
            const lastMessage = currentMessages.value[currentMessages.value.length - 1];
            if (lastMessage && typeof lastMessage.content === 'string') {
                setTimeout(() => processMessage(lastMessage.content, true), 1000 * retryCount);
            } else {
                error.value = 'Unable to retry: Invalid last message';
            }
        } else {
            if (err.message === 'Failed to fetch') {
                error.value = 'Network error. Please check your internet connection and try again.';
            } else if (err.response && err.response.status === 401) {
                error.value = 'Authentication error. Please log in again.';
            } else {
                error.value = `Error: ${err.message || 'Unknown error occurred'}. Please try again.`;
            }
            isLoading.value = false;
            retryCount = 0;
        }
    }

    function retryLastMessage() {
        error.value = '';
        const lastUserMessage = currentMessages.value.filter((m) => m.role === 'user').pop();
        if (lastUserMessage && typeof lastUserMessage.content === 'string') {
            processMessage(lastUserMessage.content, true);
        } else {
            error.value = 'No valid message to retry';
        }
    }

    function scrollToBottom() {
        if (chatMessages.value) {
            chatMessages.value.scrollTop = chatMessages.value.scrollHeight;
        }
    }

    function newline() {
        userInput.value += '\n';
    }

    async function renderMarkdown(text: string): Promise<string> {
        if (!text) return '';

        const rawMarkup = marked(text);
        const sanitizedMarkup = DOMPurify.sanitize(rawMarkup);

        // Wrap the sanitized markup in a div with allowed classes and attributes
        return `<div class="markdown-content">
            ${sanitizedMarkup}
        </div>`;
    }

    async function loadUserLLMConfigs() {
        try {
            await store.dispatch('chat/getUserLLMConfigs');
        } catch (err) {
            console.error('Error loading User LLM Configs:', err);
            error.value = 'Failed to load User LLM Configs. Please try again.';
        }
    }

    return {
        userInput,
        chatMessages,
        isLoading,
        error,
        conversations,
        currentConversation,
        currentMessages,
        userLLMConfigs,
        selectedConfigId,
        loadMessages,
        selectConversation,
        createNewConversation,
        deleteConversation,
        sendMessage,
        retryLastMessage,
        newline,
        scrollToBottom,
        renderMarkdown,
        loadUserLLMConfigs,
    };
}
