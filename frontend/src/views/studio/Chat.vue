<template>
    <div class="chat-container flex h-full bg-gray-100 dark:bg-gray-900">
        <Sidebar :isSidebarOpen="isSidebarOpen" :conversations="conversations"
            :currentConversation="currentConversation" @toggle-sidebar="toggleSidebar"
            @create-new-conversation="createNewConversation" @select-conversation="selectConversation"
            @delete-conversation="deleteConversation" />
        <div class="flex-1 flex flex-col min-h-0">
            <div class="flex-grow overflow-hidden relative">
                <ChatArea :isSidebarOpen="isSidebarOpen" :isExpanded="isExpanded"
                    :currentConversation="currentConversation" :messages="currentMessages" :isLoading="isLoading" />
            </div>
            <div class="flex-shrink-0">
                <ChatInput :isSidebarOpen="isSidebarOpen" :userLLMConfigs="userLLMConfigs"
                    v-model:selectedConfigId="selectedConfigId" v-model:userInput="userInput"
                    :currentConversation="currentConversation" :isLoading="isLoading" @send-message="sendMessage" />
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, computed, ref } from 'vue';
import { useChatLogic } from '../../components/chat/ChatLogic';
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
            userLLMConfigs,
            selectedConfigId,
            userInput,
            loadMessages,
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
                await store.dispatch('chat/getConversations');
                await loadUserLLMConfigs();

                if (userLLMConfigs.value.length > 0) {
                    selectedConfigId.value = userLLMConfigs.value[0].id;
                } else {
                    error.value = 'No User LLM Configs available. Please create one in the settings.';
                }
            } catch (err) {
                console.error('Error in onMounted:', err);
                error.value = 'Failed to fetch conversations or User LLM Configs. Please try again later.';
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
            userLLMConfigs,
            selectedConfigId,
            userInput,
            isExpanded,
            isSidebarOpen,
            selectConversation,
            createNewConversation,
            sendMessage,
            retryLastMessage,
            deleteConversation,
            toggleSidebar,
        };
    },
});
</script>

<style scoped>
.chat-container {
    height: calc(100vh - 4rem);
    overflow: hidden;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

.chat-container>div {
    transition: all 0.3s ease-in-out;
}
</style>
