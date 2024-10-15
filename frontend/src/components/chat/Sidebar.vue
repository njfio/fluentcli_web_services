<template>
    <div :class="['bg-gray-800 flex flex-col transition-all duration-300 ease-in-out',
        isSidebarOpen ? 'w-64' : 'w-16']">
        <!-- Sidebar header -->
        <div class="flex items-center justify-between p-4">
            <h2 class="text-xl font-bold text-gray-200" v-if="isSidebarOpen">Conversations</h2>
            <button @click="toggleSidebar" class="text-gray-300 hover:text-white focus:outline-none">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                </svg>
            </button>
        </div>

        <!-- New Conversation button -->
        <button @click="createNewConversation"
            class="mb-4 mx-4 px-3 py-2 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 transition duration-150 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50">
            {{ isSidebarOpen ? 'New Conversation' : '+' }}
        </button>

        <!-- Conversation list -->
        <div class="overflow-y-auto flex-grow">
            <ul class="space-y-2 px-2">
                <li v-for="conversation in conversations" :key="conversation.id"
                    @click="selectConversation(conversation.id)"
                    :class="{ 'bg-gray-700': currentConversation && conversation.id === currentConversation.id }"
                    class="cursor-pointer hover:bg-gray-700 p-2 rounded-lg transition duration-150 ease-in-out flex flex-col relative">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center">
                            <svg class="w-4 h-4 mr-2 text-gray-300" fill="none" stroke="currentColor"
                                viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z">
                                </path>
                            </svg>
                            <span v-if="isSidebarOpen" class="text-sm text-gray-300 truncate">{{ conversation.title
                                }}</span>
                        </div>
                        <button v-if="isSidebarOpen" @click.stop="deleteConversation(conversation.id)"
                            class="text-gray-400 hover:text-red-500 focus:outline-none">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    <span v-if="isSidebarOpen" class="text-xs text-gray-400 mt-1">{{ formatDate(conversation.updated_at)
                        }}</span>
                </li>
            </ul>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';

interface Conversation {
    id: string;
    title: string;
    updated_at: string;
}

export default defineComponent({
    name: 'Sidebar',
    props: {
        isSidebarOpen: {
            type: Boolean,
            required: true,
        },
        conversations: {
            type: Array as PropType<Conversation[]>,
            required: true,
        },
        currentConversation: {
            type: Object as PropType<Conversation | null>,
            required: false,
            default: null,
        },
    },
    emits: {
        'toggle-sidebar': () => true,
        'create-new-conversation': () => true,
        'select-conversation': (id: string) => typeof id === 'string',
        'delete-conversation': (id: string) => typeof id === 'string',
    },
    setup(props, { emit }) {
        const toggleSidebar = () => {
            emit('toggle-sidebar');
        };

        const createNewConversation = () => {
            emit('create-new-conversation');
        };

        const selectConversation = (id: string) => {
            emit('select-conversation', id);
        };

        const deleteConversation = (id: string) => {
            emit('delete-conversation', id);
        };

        const formatDate = (dateString: string) => {
            const date = new Date(dateString);
            return date.toLocaleString('en-US', {
                year: 'numeric',
                month: 'short',
                day: 'numeric',
                hour: '2-digit',
                minute: '2-digit'
            });
        };

        return {
            toggleSidebar,
            createNewConversation,
            selectConversation,
            deleteConversation,
            formatDate,
        };
    },
});
</script>
