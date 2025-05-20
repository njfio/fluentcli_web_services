<template>
    <div class="flex h-full">
        <Sidebar :isSidebarOpen="isSidebarOpen" :conversations="conversations"
            :currentConversation="currentConversation" @toggle-sidebar="toggleSidebar"
            @create-new-conversation="createNewConversation" @select-conversation="handleSelectConversation"
            @delete-conversation="deleteConversation" />
        <div class="flex-1 flex flex-col min-h-0">
            <div class="flex-grow overflow-hidden relative">
                <ChatArea :isSidebarOpen="isSidebarOpen" :isExpanded="isExpanded"
                    :currentConversation="currentConversation" :messages="currentMessages" :isLoading="isLoading" />
            </div>
            <div class="flex-shrink-0">
                <ChatInput :isSidebarOpen="isSidebarOpen" :userLLMConfigs="userLLMConfigs"
                    v-model:selectedConfigId="selectedConfigId" v-model:userInput="userInput"
                    :currentConversation="currentConversation" :isLoading="isLoading"
                    @send-message="sendMessage"
                    @send-message-with-tools="sendMessageWithTools" />
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, computed, ref } from 'vue';
import { useChatLogic } from '../../components/chat/ChatLogic';
import { useStore } from 'vuex';
import { useRouter, useRoute } from 'vue-router';
import { RootState } from '../../store/types';
import Sidebar from '../../components/chat/Sidebar.vue';
import ChatArea from '../../components/chat/ChatArea.vue';
import ChatInput from '../../components/chat/ChatInput.vue';
import { renderMarkdown } from '../../utils/markdown';

export default defineComponent({
    name: 'Chat',
    components: {
        Sidebar,
        ChatArea,
        ChatInput,
    },
    props: {
        conversationId: {
            type: String,
            required: false,
            default: null
        }
    },
    setup(props) {
        const store = useStore<RootState>();
        const router = useRouter();
        const route = useRoute();
        const {
            isLoading,
            error,
            conversations,
            currentConversation,
            currentMessages,
            userLLMConfigs,
            selectedConfigId,
            userInput,

            selectConversation,
            createNewConversation,
            sendMessage,
            retryLastMessage,
            deleteConversation,
            loadUserLLMConfigs,
        } = useChatLogic();

        const isSidebarOpen = ref(true);
        const isExpanded = ref(false);

        const isAuthenticated = computed(() => store.getters.isAuthenticated);
        const userId = computed(() => store.getters.userId);

        const toggleSidebar = () => {
            isSidebarOpen.value = !isSidebarOpen.value;
        };

        const handleSelectConversation = async (id: string) => {
            try {
                await selectConversation(id);
                router.push({ name: 'Chat', params: { conversationId: id } });
            } catch (err) {
                console.error('Error selecting conversation:', err);
            }
        };

        // Function to handle conversation loading
        const loadConversation = async (conversationId: string) => {
            try {
                console.log('Loading conversation:', conversationId);
                isLoading.value = true;

                // First get the conversations if they haven't been loaded
                if (conversations.value.length === 0) {
                    await store.dispatch('chat/getConversations');
                }

                // Use switchConversation to properly set the current conversation and load messages
                await store.dispatch('chat/switchConversation', conversationId);

                // Update local messages state
                const messages = store.getters['chat/getCurrentConversationMessages'];
                currentMessages.value = messages;

                // Ensure the conversation is selected in the store
                const conversation = store.getters['chat/getConversationById'](conversationId);
                if (conversation) {
                    store.commit('chat/setCurrentConversation', conversation);
                }

            } catch (err) {
                console.error('Error loading conversation:', err);
                error.value = 'Failed to load conversation. Please try again later.';
            } finally {
                isLoading.value = false;
            }
        };

        onMounted(async () => {
            try {
                if (!isAuthenticated.value) {
                    router.push('/login');
                    return;
                }
                if (!userId.value) {
                    console.error('User is authenticated but user ID is missing');
                    return;
                }

                // Load initial data
                await store.dispatch('chat/getConversations');
                await loadUserLLMConfigs();

                if (userLLMConfigs.value.length > 0) {
                    selectedConfigId.value = userLLMConfigs.value[0].id;
                } else {
                    error.value = 'No User LLM Configs available. Please create one in the settings.';
                }

                // Handle initial conversation selection from props or route
                const targetConversationId = props.conversationId || route.params.conversationId as string;
                if (targetConversationId) {
                    await loadConversation(targetConversationId);
                }
            } catch (err) {
                console.error('Error in onMounted:', err);
                error.value = 'Failed to fetch conversations or User LLM Configs. Please try again later.';
            }
        });

        // Watch for changes in route params
        watch(() => route.params.conversationId, async (newId) => {
            if (newId && (!currentConversation.value || currentConversation.value.id !== newId)) {
                console.log('Route conversation ID changed:', newId);
                await loadConversation(newId as string);
            }
        });

        // Function to send a message with function calling
        const sendMessageWithTools = async () => {
            try {
                if (!currentConversation.value || !selectedConfigId.value || userInput.value.trim() === '') {
                    return;
                }

                isLoading.value = true;

                // Get the agent from the store
                const agent = store.getters['agent/selectedAgent'];
                if (!agent) {
                    console.error('No agent selected');
                    return;
                }

                // Get the tools for the agent
                const tools = agent.tools.map((toolId: string) => {
                    return store.state.tool.tools.find((tool: any) => tool.id === toolId);
                }).filter(Boolean);

                // Create a user message
                const userMessageResponse = await store.dispatch('chat/createMessage', {
                    conversationId: currentConversation.value.id,
                    role: 'user',
                    content: userInput.value,
                    sender: 'User',
                });

                // Add the user message to the current messages
                const userMessage = userMessageResponse.data;
                userMessage.renderedContent = await renderMarkdown(userMessage.content);
                store.commit('chat/addMessage', userMessage);

                // Clear the user input
                userInput.value = '';

                // Prepare the messages for the LLM
                const messages = currentMessages.value
                    .filter(m => m && typeof m === 'object' && 'role' in m && 'content' in m)
                    .map(m => ({
                        role: m.role,
                        content: m.content,
                    }));

                // Add the system prompt from the agent if available
                if (agent.system_prompt) {
                    messages.unshift({
                        role: 'system',
                        content: agent.system_prompt,
                    });
                }

                // Send the message to the backend with tools
                const response = await store.dispatch('chat/sendMessageWithTools', {
                    conversationId: currentConversation.value.id,
                    userLLMConfigId: selectedConfigId.value,
                    messages,
                    tools,
                });

                // Process the response
                if (response && response.data) {
                    // Add the assistant message to the current messages
                    const assistantMessage = response.data;
                    assistantMessage.renderedContent = await renderMarkdown(assistantMessage.content || '');
                    store.commit('chat/addMessage', assistantMessage);

                    // Process tool calls if any
                    if (assistantMessage.tool_calls && assistantMessage.tool_calls.length > 0) {
                        for (const toolCall of assistantMessage.tool_calls) {
                            // Execute the tool call
                            await store.dispatch('tool/executeToolCall', toolCall);

                            // Update the tool call in the message with the result
                            const updatedToolCall = store.getters['tool/getToolCallById'](toolCall.id);
                            if (updatedToolCall) {
                                const toolCallIndex = assistantMessage.tool_calls.findIndex((tc: any) => tc.id === toolCall.id);
                                if (toolCallIndex !== -1) {
                                    assistantMessage.tool_calls[toolCallIndex] = updatedToolCall;
                                }
                            }
                        }

                        // Update the message in the store
                        store.commit('chat/updateMessage', assistantMessage);
                    }
                }
            } catch (error) {
                console.error('Error sending message with tools:', error);
            } finally {
                isLoading.value = false;
            }
        };

        return {
            isLoading,
            error,
            conversations,
            currentConversation,
            currentMessages,
            userLLMConfigs,
            selectedConfigId,
            userInput,
            isExpanded,
            isSidebarOpen,
            handleSelectConversation,
            createNewConversation,
            sendMessage,
            sendMessageWithTools,
            retryLastMessage,
            deleteConversation,
            toggleSidebar,
        };
    },
});
</script>
