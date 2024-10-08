<template>
    <div :class="['studio-layout flex h-screen', { 'dark': isDarkMode }]">
        <StudioSidebar :isCollapsed="isSidebarCollapsed" @toggle="toggleSidebar" />
        <div class="flex-1 flex flex-col overflow-hidden">
            <StudioHeader @toggleSidebar="toggleSidebar" @toggleDarkMode="toggleDarkMode" :isDarkMode="isDarkMode" />
            <main
                class="flex-grow overflow-x-hidden overflow-y-auto bg-gradient-to-br from-neutral-50 to-primary-100 dark:from-gray-900 dark:to-gray-800 text-gray-900 dark:text-white">
                <div class="container mx-auto p-6">
                    <slot></slot>
                </div>
            </main>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useStore } from 'vuex';
import StudioHeader from './StudioHeader.vue';
import StudioSidebar from './StudioSidebar.vue';

const store = useStore();
const isSidebarCollapsed = ref(false);

const isDarkMode = computed(() => store.getters['theme/isDarkMode']);

const toggleSidebar = () => {
    isSidebarCollapsed.value = !isSidebarCollapsed.value;
};

const toggleDarkMode = () => {
    store.dispatch('theme/toggleDarkMode');
};
</script>

<style scoped>
.studio-layout {
    --sidebar-width: 16rem;
    --sidebar-collapsed-width: 5rem;
    --transition-duration: 0.3s;
}

:deep(.sidebar) {
    flex-basis: var(--sidebar-width);
    flex-shrink: 0;
    transition: flex-basis var(--transition-duration) ease-in-out;
}

:deep(.sidebar.collapsed) {
    flex-basis: var(--sidebar-collapsed-width);
}

@media (max-width: 768px) {
    :deep(.sidebar) {
        position: fixed;
        z-index: 40;
        height: 100vh;
        left: 0;
        top: 0;
        transform: translateX(0);
        transition: transform var(--transition-duration) ease-in-out;
    }

    :deep(.sidebar.collapsed) {
        transform: translateX(-100%);
    }
}
</style>