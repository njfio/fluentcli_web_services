import { ref, computed, watch } from 'vue';
import { useStore } from 'vuex';
import LLMService, { LLMProvider, LLMMessage } from '../../services/LLMService';
import { Message } from '../../store/modules/chat';
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

    const llmProviders = ref<LLMProvider[]>([]);
    const selectedProviderId = ref<string>('');

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
            // If the deleted conversation was the current one, clear the current messages
            if (currentConversation.value && currentConversation.value.id === conversationId) {
                currentMessages.value = [];
            }
            // Optionally, you can select another conversation or clear the current conversation
            // depending on your app's requirements
        } catch (err) {
            console.error('Error deleting conversation:', err);
            error.value = 'Failed to delete conversation. Please try again.';
        }
    }

    async function sendMessage() {
        if (userInput.value.trim() === '' || isLoading.value || !currentConversation.value || !selectedProviderId.value) return;
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
        if (!message || !selectedProviderId.value) {
            error.value = message ? 'Please select an LLM provider before sending a message.' : 'Cannot process empty message.';
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
                const result = await store.dispatch('chat/createMessage', {
                    conversationId: currentConversation.value.id,
                    role: 'user',
                    content: message,
                });
                console.log('User message created:', result);
                userMessage = extractMessage(result);
                if (userMessage) {
                    userMessage.renderedContent = await renderMarkdown(userMessage.content);
                    currentMessages.value.push(userMessage);
                    userInput.value = '';
                }
            }

            console.log('Current Messages before filtering:', JSON.stringify(currentMessages.value, null, 2));

            const llmMessages: LLMMessage[] = currentMessages.value
                .filter(m => m && typeof m === 'object' && 'role' in m && 'content' in m)
                .map(m => ({
                    role: m.role as 'system' | 'user' | 'assistant',
                    content: m.content,
                }));

            console.log('Current Messages:', JSON.stringify(currentMessages.value, null, 2));
            console.log('LLM Messages:', JSON.stringify(llmMessages, null, 2));
            console.log('Selected Provider ID:', selectedProviderId.value);

            if (llmMessages.length === 0) {
                throw new Error('No valid messages to send to LLM');
            }

            console.log('Sending request to LLM service...');
            const stream = await LLMService.streamChat(selectedProviderId.value, llmMessages);
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
                const lines = chunk.split('\n');

                for (const line of lines) {
                    if (line.startsWith('data: ')) {
                        const jsonStr = line.slice(6).trim();
                        if (jsonStr === '[DONE]') {
                            console.log('Stream completed');
                            continue;
                        }
                        try {
                            const parsedChunk = JSON.parse(jsonStr);
                            console.log('Parsed chunk:', parsedChunk);
                            if (parsedChunk.choices && parsedChunk.choices[0].delta.content) {
                                const newContent = parsedChunk.choices[0].delta.content;
                                fullContent += newContent;

                                if (!assistantMessage) {
                                    assistantMessage = {
                                        id: '', // This will be set when we create the message on the backend
                                        conversationId: currentConversation.value!.id,
                                        role: 'assistant',
                                        content: fullContent,
                                        createdAt: new Date().toISOString(),
                                        renderedContent: await renderMarkdown(fullContent)
                                    };
                                    currentMessages.value.push(assistantMessage);
                                } else {
                                    assistantMessage.content = fullContent;
                                    assistantMessage.renderedContent = await renderMarkdown(fullContent);
                                }

                                // Update the last message in the array
                                const lastIndex = currentMessages.value.length - 1;
                                if (lastIndex >= 0) {
                                    currentMessages.value = [
                                        ...currentMessages.value.slice(0, lastIndex),
                                        { ...assistantMessage }
                                    ];
                                }
                                console.log('Updated assistant message:', JSON.stringify(assistantMessage, null, 2));
                            }
                        } catch (e) {
                            console.error('Error parsing chunk:', e, 'Raw data:', jsonStr);
                        }
                    }
                }
            }

            // Create the final assistant message on the backend
            if (assistantMessage) {
                const newMessageResult = await store.dispatch('chat/createMessage', {
                    conversationId: currentConversation.value!.id,
                    role: 'assistant',
                    content: fullContent,
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
    return {
        userInput,
        chatMessages,
        isLoading,
        error,
        conversations,
        currentConversation,
        currentMessages,
        llmProviders,
        selectedProviderId,
        loadMessages,
        selectConversation,
        createNewConversation,
        deleteConversation,
        sendMessage,
        retryLastMessage,
        newline,
        scrollToBottom,
        renderMarkdown,
    };
}
