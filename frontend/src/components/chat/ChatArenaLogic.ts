import { ref, computed } from 'vue';
import { useStore } from 'vuex';
import { Message, UserLLMConfig } from '../../store/modules/chat';
import chatArenaService, { ArenaMessage } from '../../services/chatArenaService';
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

export function useChatArenaLogic() {
    const store = useStore();
    const userInput = ref('');
    const chatMessages = ref<HTMLElement | null>(null);
    const isLoading = ref(false);
    const error = ref('');
    const selectedConfigIds = ref<string[]>([]);

    const conversations = computed(() => store.state.chat.conversations);
    const currentConversation = computed(() => store.state.chat.currentConversation);
    const messages = computed(() => store.state.chat.messages);
    const currentMessages = ref<Message[]>([]);
    const userLLMConfigs = computed(() => store.state.chat.userLLMConfigs);

    async function loadMessages(conversationId: string) {
        await store.dispatch('chat/getMessages', conversationId);
        currentMessages.value = await Promise.all(messages.value
            .filter((m: Message | null): m is Message => m !== null)
            .map(async (m: Message) => ({
                ...m,
                renderedContent: await renderMarkdown(m.content)
            })));
    }

    async function selectConversation(conversationId: string) {
        await store.dispatch('chat/getConversation', conversationId);
        await loadMessages(conversationId);
    }

    async function createNewConversation(title: string) {
        try {
            const userId = store.getters.userId;
            if (!userId) {
                throw new Error('User ID not found');
            }

            const response = await chatArenaService.createArenaConversation(userId, title);
            const newConversation = response.data;
            await selectConversation(newConversation.id);
            return newConversation;
        } catch (err) {
            console.error('Error creating new arena conversation:', err);
            error.value = 'Failed to create a new arena conversation. Please try again.';
            throw err;
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
        if (userInput.value.trim() === '' || isLoading.value || !currentConversation.value || selectedConfigIds.value.length === 0) {
            return;
        }

        isLoading.value = true;
        error.value = '';

        try {
            // Create user message
            const userMessageResponse = await chatArenaService.createArenaMessage(
                currentConversation.value.id,
                'user',
                userInput.value,
                'user'
            );

            const userMessage = userMessageResponse.data;
            userMessage.renderedContent = await renderMarkdown(userMessage.content);
            currentMessages.value.push(userMessage);

            // Get provider models for selected configs
            const configProviderModels = await Promise.all(
                selectedConfigIds.value.map(async (configId) => {
                    const config = userLLMConfigs.value.find((c: UserLLMConfig) => c.id === configId);
                    if (!config) throw new Error(`Config ${configId} not found`);
                    if (!config.provider_id) throw new Error(`Provider ID not found for config ${configId}`);

                    // Fetch the LLM provider to get the provider name
                    const provider = await store.dispatch('chat/getLLMProvider', config.provider_id);
                    return { configId, providerName: provider.name };
                })
            );

            // Stream to all selected LLMs
            const streams = await chatArenaService.streamChatToMultipleLLMs(
                currentConversation.value.id,
                userInput.value,
                selectedConfigIds.value,
                currentMessages.value
            );

            // Process all responses
            const arenaMessages = await chatArenaService.processArenaResponses(
                streams,
                currentConversation.value.id,
                selectedConfigIds.value,
                configProviderModels.map(c => c.providerName)
            );

            // Add rendered content to messages
            const renderedMessages = await Promise.all(
                arenaMessages.map(async (message: ArenaMessage) => ({
                    ...message,
                    renderedContent: await renderMarkdown(message.content)
                }))
            );

            // Update messages in UI
            currentMessages.value = [...currentMessages.value, ...renderedMessages];

            // Clear input
            userInput.value = '';
        } catch (err) {
            console.error('Error in arena chat:', err);
            error.value = 'An error occurred while processing your message. Please try again.';
        } finally {
            isLoading.value = false;
        }
    }

    async function renderMarkdown(text: string): Promise<string> {
        if (!text) return '';

        const rawMarkup = marked(text);
        const sanitizedMarkup = DOMPurify.sanitize(rawMarkup);

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

    function scrollToBottom() {
        if (chatMessages.value) {
            chatMessages.value.scrollTop = chatMessages.value.scrollHeight;
        }
    }

    function newline() {
        userInput.value += '\n';
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
        selectedConfigIds,
        loadMessages,
        selectConversation,
        createNewConversation,
        deleteConversation,
        sendMessage,
        newline,
        scrollToBottom,
        renderMarkdown,
        loadUserLLMConfigs,
    };
}
