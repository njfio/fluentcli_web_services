<template>
  <nav :class="['sidebar bg-gray-800 text-white', { 'collapsed': isCollapsed }]">
    <div class="p-4">
      <div class="flex items-center justify-between mb-4">
        <h2 v-if="!isCollapsed" class="text-2xl font-semibold">Studio</h2>
        <button @click="toggleSidebar" class="text-white focus:outline-none">
          <ChevronDoubleLeftIcon v-if="!isCollapsed" class="h-6 w-6" />
          <ChevronDoubleRightIcon v-else class="h-6 w-6" />
        </button>
      </div>
      <ul>
        <li v-for="(item, index) in menuItems" :key="index" class="mb-2">
          <router-link :to="item.route" class="flex items-center py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === item.routeName }">
            <component :is="item.icon" class="h-5 w-5" :class="{ 'mr-3': !isCollapsed }" />
            <span v-if="!isCollapsed">{{ item.label }}</span>
          </router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import {
  HomeIcon,
  BriefcaseIcon,
  CogIcon,
  ServerIcon,
  CircleStackIcon,
  DocumentIcon,
  AdjustmentsHorizontalIcon,
  ChevronDoubleLeftIcon,
  ChevronDoubleRightIcon
} from '@heroicons/vue/24/outline';

defineProps<{
  isCollapsed: boolean
}>();

const emit = defineEmits(['toggle']);

const toggleSidebar = () => {
  emit('toggle');
};

const menuItems = [
  { label: 'Dashboard', route: '/studio/dashboard', routeName: 'Dashboard', icon: HomeIcon },
  { label: 'Jobs', route: '/studio/jobs', routeName: 'Jobs', icon: BriefcaseIcon },
  { label: 'Pipelines', route: '/studio/pipelines', routeName: 'Pipelines', icon: CogIcon },
  { label: 'Configurations', route: '/studio/configurations', routeName: 'Configurations', icon: AdjustmentsHorizontalIcon },
  { label: 'Docker Files', route: '/studio/dockerfiles', routeName: 'DockerFiles', icon: ServerIcon },
  { label: 'Amber Stores', route: '/studio/amberstores', routeName: 'AmberStores', icon: CircleStackIcon },
  { label: 'State Files', route: '/studio/statefiles', routeName: 'StateFiles', icon: DocumentIcon },
  { label: 'Settings', route: '/studio/settings', routeName: 'Settings', icon: CogIcon },
];
</script>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  transition: width 0.3s ease-in-out;
}

.sidebar.collapsed {
  width: var(--sidebar-collapsed-width);
}

.sidebar.collapsed .flex {
  justify-content: center;
}

@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    z-index: 40;
    height: 100vh;
  }

  .sidebar.collapsed {
    transform: translateX(-100%);
  }
}
</style>