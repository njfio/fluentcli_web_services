<template>
    <div class="flex h-full">
        <Sidebar :isSidebarOpen="true" :conversations="conversations" :currentConversation="currentConversation"
            @create-new-conversation="createNewArenaConversation" @select-conversation="selectConversation"
            @delete-conversation="deleteConversation" />

        <div class="flex-1 flex flex-col min-h-0">
            <div class="flex-grow overflow-hidden relative">
                <ChatArea :isSidebarOpen="true" :isExpanded="false" :currentConversation="currentConversation"
                    :messages="currentMessages" :isLoading="isLoading" />
            </div>
            <div class="flex-shrink-0">
                <ChatArenaInput :isSidebarOpen="true" :userLLMConfigs="userLLMConfigs"
                    v-model:selectedConfigIds="selectedConfigIds" v-model:userInput="userInput"
                    :currentConversation="currentConversation" :isLoading="isLoading" @send-message="sendMessage" />
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, watch, computed } from 'vue';
import { useChatArenaLogic } from '../../components/chat/ChatArenaLogic';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { RootState } from '../../store/types';
import Sidebar from '../../components/chat/Sidebar.vue';
import ChatArea from '../../components/chat/ChatArea.vue';
import ChatArenaInput from '../../components/chat/ChatArenaInput.vue';

export default defineComponent({
    name: 'ChatArena',
    components: {
        Sidebar,
        ChatArea,
        ChatArenaInput,
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
            selectedConfigIds,
            userInput,
            loadMessages,
            selectConversation,
            createNewConversation,
            sendMessage,
            deleteConversation,
            loadUserLLMConfigs,
        } = useChatArenaLogic();

        const isAuthenticated = computed(() => store.getters.isAuthenticated);
        const userId = computed(() => store.getters.userId);

        const createNewArenaConversation = async () => {
            const title = prompt('Enter conversation title:');
            if (title) {
                try {
                    await createNewConversation(title);
                } catch (err) {
                    console.error('Error creating new arena conversation:', err);
                    error.value = 'Failed to create a new arena conversation. Please try again later.';
                }
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
                await store.dispatch('chat/getConversations');
                await loadUserLLMConfigs();

                if (userLLMConfigs.value.length > 0) {
                    selectedConfigIds.value = [userLLMConfigs.value[0].id];
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
            selectedConfigIds,
            userInput,
            selectConversation,
            createNewArenaConversation,
            sendMessage,
            deleteConversation,
        };
    },
});
</script>

<style scoped>
.h-full {
    height: 100%;
}
</style>
