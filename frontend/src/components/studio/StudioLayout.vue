<template>
  <div class="studio-layout flex h-screen bg-neutral-100">
    <StudioSidebar :isCollapsed="isSidebarCollapsed" @toggle="toggleSidebar" />
    <div class="flex-1 flex flex-col overflow-hidden">
      <StudioHeader @toggleSidebar="toggleSidebar" />
      <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gradient-to-br from-neutral-50 to-primary-100 p-6">
        <div class="container mx-auto">
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
  --sidebar-width: 250px;
  --sidebar-collapsed-width: 80px;
}

.studio-layout :deep(.sidebar) {
  width: var(--sidebar-width);
  transition: width 0.3s ease-in-out;
}

.studio-layout :deep(.sidebar.collapsed) {
  width: var(--sidebar-collapsed-width);
}

@media (max-width: 768px) {
  .studio-layout :deep(.sidebar) {
    position: fixed;
    z-index: 40;
    height: 100vh;
  }

  .studio-layout :deep(.sidebar.collapsed) {
    transform: translateX(-100%);
  }
}
</style>