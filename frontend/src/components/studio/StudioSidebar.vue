<template>
  <nav :class="['sidebar bg-white dark:bg-gray-800 text-gray-900 dark:text-white', { 'collapsed': isCollapsed }]">
    <div class="p-4 flex flex-col h-full">
      <div class="flex items-center justify-end mb-4">
        <button @click="toggleSidebar" class="text-gray-500 dark:text-gray-400 focus:outline-none">
          <ChevronDoubleLeftIcon v-if="!isCollapsed" class="h-6 w-6" />
          <ChevronDoubleRightIcon v-else class="h-6 w-6" />
        </button>
      </div>
      <ul class="flex-grow">
        <li v-for="(item, index) in mainMenuItems" :key="index" class="mb-2">
          <router-link :to="item.route" class="flex items-center py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-100 dark:bg-primary-800 text-primary-900 dark:text-primary-100': $route.name === item.routeName, 'hover:bg-gray-100 dark:hover:bg-gray-700': $route.name !== item.routeName }">
            <component :is="item.icon" class="h-5 w-5" :class="{ 'mr-3': !isCollapsed }" />
            <span v-if="!isCollapsed">{{ item.label }}</span>
          </router-link>
        </li>
      </ul>
      <ul class="mt-auto">
        <li class="mb-2">
          <router-link :to="settingsMenuItem.route"
            class="flex items-center py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-100 dark:bg-primary-800 text-primary-900 dark:text-primary-100': $route.name === settingsMenuItem.routeName, 'hover:bg-gray-100 dark:hover:bg-gray-700': $route.name !== settingsMenuItem.routeName }">
            <component :is="settingsMenuItem.icon" class="h-5 w-5" :class="{ 'mr-3': !isCollapsed }" />
            <span v-if="!isCollapsed">{{ settingsMenuItem.label }}</span>
          </router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, defineComponent, h } from 'vue';
import {
  HomeIcon,
  BriefcaseIcon,
  CogIcon,
  ServerIcon,
  CircleStackIcon,
  DocumentIcon,
  AdjustmentsHorizontalIcon,
  ChevronDoubleLeftIcon,
  ChevronDoubleRightIcon,
} from '@heroicons/vue/24/outline';

const ChatIcon = defineComponent({
  name: 'ChatIcon',
  render() {
    return h('svg', {
      xmlns: 'http://www.w3.org/2000/svg',
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
      class: 'w-6 h-6',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z',
      }),
    ]);
  },
});

const ApiKeyIcon = defineComponent({
  name: 'ApiKeyIcon',
  render() {
    return h('svg', {
      xmlns: 'http://www.w3.org/2000/svg',
      fill: 'none',
      viewBox: '0 0 24 24',
      stroke: 'currentColor',
      class: 'w-6 h-6',
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z',
      }),
    ]);
  },
});

defineProps<{
  isCollapsed: boolean
}>();

const emit = defineEmits(['toggle']);

const toggleSidebar = () => {
  emit('toggle');
};

const mainMenuItems = [
  { label: 'Dashboard', route: '/studio/dashboard', routeName: 'Dashboard', icon: HomeIcon },
  { label: 'Jobs', route: '/studio/jobs', routeName: 'Jobs', icon: BriefcaseIcon },
  { label: 'Pipelines', route: '/studio/pipelines', routeName: 'Pipelines', icon: CogIcon },
  { label: 'Configurations', route: '/studio/configurations', routeName: 'Configurations', icon: AdjustmentsHorizontalIcon },
  { label: 'Docker Files', route: '/studio/dockerfiles', routeName: 'DockerFiles', icon: ServerIcon },
  { label: 'Amber Stores', route: '/studio/amberstores', routeName: 'AmberStores', icon: CircleStackIcon },
  { label: 'State Files', route: '/studio/statefiles', routeName: 'StateFiles', icon: DocumentIcon },
  { label: 'Chat', route: '/studio/chat', routeName: 'Chat', icon: ChatIcon },
  { label: 'API Keys', route: '/studio/apikeys', routeName: 'ApiKeys', icon: ApiKeyIcon },
];

const settingsMenuItem = { label: 'Settings', route: '/studio/settings', routeName: 'Settings', icon: CogIcon };
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
