<template>
    <div class="studio-layout flex h-screen bg-neutral-100">
        <StudioSidebar :isCollapsed="isSidebarCollapsed" @toggle="toggleSidebar" />
        <div class="flex flex-col flex-grow overflow-hidden">
            <StudioHeader @toggleSidebar="toggleSidebar" />
            <main class="flex-grow overflow-x-hidden overflow-y-auto bg-gradient-to-br from-neutral-50 to-primary-100">
                <div class="container mx-auto p-6">
                    <slot></slot>
                </div>
            </main>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import StudioHeader from './StudioHeader.vue';
import StudioSidebar from './StudioSidebar.vue';

const isSidebarCollapsed = ref(false);

const toggleSidebar = () => {
    isSidebarCollapsed.value = !isSidebarCollapsed.value;
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