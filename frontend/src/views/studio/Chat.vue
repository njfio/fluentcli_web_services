<template>
    <div class="chat-container flex h-full bg-gray-100 dark:bg-gray-900">
        <Sidebar :isSidebarOpen="isSidebarOpen" :conversations="conversations"
            :currentConversation="currentConversation" @toggle-sidebar="toggleSidebar"
            @create-new-conversation="createNewConversation" @select-conversation="selectConversation"
            @delete-conversation="deleteConversation" />
        <ChatArea :isSidebarOpen="isSidebarOpen" :isExpanded="isExpanded" :currentConversation="currentConversation"
            :messages="currentMessages" :isLoading="isLoading" />
        <ChatInput :isSidebarOpen="isSidebarOpen" :isExpanded="isExpanded" :llmProviders="llmProviders"
            :selectedProviderId="selectedProviderId || ''" :currentConversation="currentConversation"
            :isLoading="isLoading" @toggle-expand="toggleExpand" @update:selectedProviderId="updateSelectedProvider"
            @send-message="sendMessage" />
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, computed, ref } from 'vue';
import { useChatLogic } from '../../components/chat/ChatLogic';
import LLMService from '../../services/LLMService';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { RootState } from '../../store/types';
import Sidebar from '../../components/chat/Sidebar.vue';
import ChatArea from '../../components/chat/ChatArea.vue';
import ChatInput from '../../components/chat/ChatInput.vue';

export default defineComponent({
    name: 'Chat',
    components: {
        Sidebar,
        ChatArea,
        ChatInput,
    },
    setup() {
        const store = useStore<RootState>();
        const router = useRouter();
        const {
            isLoading,
            error,
            conversations,
            currentConversation,
            currentMessages,
            llmProviders,
            loadMessages,
            selectConversation,
            createNewConversation,
            sendMessage,
            retryLastMessage,
            deleteConversation,
        } = useChatLogic();

        const selectedProviderId = ref<string>('');
        const isSidebarOpen = ref(true);
        const isExpanded = ref(false);

        const isAuthenticated = computed(() => store.getters.isAuthenticated);
        const userId = computed(() => store.getters.userId);

        const toggleExpand = () => {
            isExpanded.value = !isExpanded.value;
        };

        const toggleSidebar = () => {
            isSidebarOpen.value = !isSidebarOpen.value;
        };

        const updateSelectedProvider = (providerId: string) => {
            selectedProviderId.value = providerId;
        };

        const fetchLLMProviders = async () => {
            try {
                llmProviders.value = await LLMService.getProviders();
                if (llmProviders.value.length > 0) {
                    selectedProviderId.value = llmProviders.value[0].id;
                } else {
                    error.value = 'No LLM providers available. Please contact the administrator.';
                }
            } catch (err) {
                console.error('Error fetching LLM providers:', err);
                error.value = 'Failed to fetch LLM providers. Please try again later.';
            }
        };

        onMounted(async () => {
            try {
                console.log('Chat component mounted');
                console.log('Is authenticated:', isAuthenticated.value);
                console.log('User ID:', userId.value);
                console.log('Fetching conversations...');
                if (!isAuthenticated.value) {
                    console.log('User not authenticated. Redirecting to login...');
                    router.push('/login');
                    return;
                }
                if (!userId.value) {
                    console.error('User is authenticated but user ID is missing');
                    return;
                }
                await store.dispatch('chat/getConversations');
                console.log('Conversations fetched:', conversations.value);

                await fetchLLMProviders();
            } catch (err) {
                console.error('Error in onMounted:', err);
                error.value = 'Failed to fetch conversations. Please try again later.';
            }
        });

        watch(currentConversation, async (newConversation) => {
            if (newConversation) {
                await loadMessages(newConversation.id);
            }
        });

        return {
            isLoading,
            error,
            conversations,
            currentConversation,
            currentMessages,
            llmProviders,
            selectedProviderId,
            isExpanded,
            isSidebarOpen,
            selectConversation,
            createNewConversation,
            sendMessage,
            retryLastMessage,
            deleteConversation,
            toggleExpand,
            toggleSidebar,
            updateSelectedProvider,
        };
    },
});
</script>

<style scoped>
.chat-container {
    height: calc(99vh - 140px);
    overflow: hidden;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

.chat-container {
    padding-bottom: 20px;
    margin-bottom: 20px;
}

.chat-container>div {
    transition: all 0.3s ease-in-out;
}
</style>
