<template>
    <div class="border-t border-gray-200 p-4 bg-white">
        <div class="mb-4">
            <div class="flex flex-wrap gap-2 mb-2">
                <div v-for="config in userLLMConfigs" :key="config.id"
                    class="flex items-center space-x-2 p-2 border rounded"
                    :class="selectedConfigIds.includes(config.id) ? 'border-blue-500 bg-blue-50' : 'border-gray-300'">
                    <input type="checkbox" :id="config.id" :value="config.id"
                        :checked="selectedConfigIds.includes(config.id)" @change="updateSelectedConfigs(config.id)"
                        class="form-checkbox h-4 w-4 text-blue-600">
                    <label :for="config.id" class="text-sm">{{ config.description || 'Unnamed Config' }}</label>
                </div>
            </div>
        </div>
        <div class="flex items-end space-x-4">
            <div class="flex-grow">
                <textarea :value="userInput" @input="updateUserInput($event)" @keydown.enter.exact.prevent="sendMessage"
                    @keydown.enter.shift.exact="newline" placeholder="Type your message..."
                    class="w-full p-2 border border-gray-300 rounded-lg focus:outline-none focus:border-blue-500 resize-none"
                    :rows="3"></textarea>
            </div>
            <button @click="sendMessage" :disabled="isLoading || !currentConversation || selectedConfigIds.length === 0"
                class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none disabled:opacity-50">
                Send
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { UserLLMConfig } from '../../store/modules/chat';

const props = defineProps<{
    isSidebarOpen: boolean;
    userLLMConfigs: UserLLMConfig[];
    selectedConfigIds: string[];
    userInput: string;
    currentConversation: any;
    isLoading: boolean;
}>();

const emit = defineEmits<{
    (e: 'update:selectedConfigIds', value: string[]): void;
    (e: 'update:userInput', value: string): void;
    (e: 'send-message'): void;
}>();

function updateSelectedConfigs(configId: string) {
    const newSelectedConfigs = props.selectedConfigIds.includes(configId)
        ? props.selectedConfigIds.filter(id => id !== configId)
        : [...props.selectedConfigIds, configId];
    emit('update:selectedConfigIds', newSelectedConfigs);
}

function updateUserInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    emit('update:userInput', target.value);
}

function newline() {
    emit('update:userInput', props.userInput + '\n');
}

function sendMessage() {
    if (props.userInput.trim() && !props.isLoading && props.currentConversation && props.selectedConfigIds.length > 0) {
        emit('send-message');
    }
}
</script>
