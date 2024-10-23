<template>
    <div class="flex h-full">
        <Sidebar :isSidebarOpen="isSidebarOpen" :conversations="conversations"
            :currentConversation="currentConversation" @toggle-sidebar="toggleSidebar"
            @create-new-conversation="createNewArenaConversation" @select-conversation="selectConversation"
            @delete-conversation="deleteConversation" />
        <div class="flex-1 flex flex-col min-h-0">
            <div class="flex-grow overflow-hidden relative">
                <ChatArea :isSidebarOpen="isSidebarOpen" :isExpanded="isExpanded"
                    :currentConversation="currentConversation" :messages="currentMessages" :isLoading="isLoading" />
            </div>
            <div class="flex-shrink-0">
                <ChatArenaInput :isSidebarOpen="isSidebarOpen" :userLLMConfigs="userLLMConfigs"
                    v-model:selectedConfigIds="selectedConfigIds" v-model:userInput="userInput"
                    :currentConversation="currentConversation" :isLoading="isLoading" @send-message="sendMessage" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, watch, computed, ref } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { useChatArenaLogic } from '../../components/chat/ChatArenaLogic';
import Sidebar from '../../components/chat/Sidebar.vue';
import ChatArea from '../../components/chat/ChatArea.vue';
import ChatArenaInput from '../../components/chat/ChatArenaInput.vue';

const store = useStore();
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
const isSidebarOpen = ref(true);
const isExpanded = ref(false);

const toggleSidebar = () => {
    isSidebarOpen.value = !isSidebarOpen.value;
};

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
            // Select first config by default
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
</script>

<style scoped>
.flex {
    display: flex;
}

.h-full {
    height: 100%;
}

.flex-1 {
    flex: 1;
}

.flex-col {
    flex-direction: column;
}

.min-h-0 {
    min-height: 0;
}

.flex-grow {
    flex-grow: 1;
}

.overflow-hidden {
    overflow: hidden;
}

.relative {
    position: relative;
}

.flex-shrink-0 {
    flex-shrink: 0;
}
</style>
