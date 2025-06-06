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

const ChatArenaIcon = defineComponent({
  name: 'ChatArenaIcon',
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
        d: 'M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z',
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

const LLMProviderIcon = defineComponent({
  name: 'LLMProviderIcon',
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
        d: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z',
      }),
    ]);
  },
});

const UserLLMConfigIcon = defineComponent({
  name: 'UserLLMConfigIcon',
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
        d: 'M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4',
      }),
    ]);
  },
});

const UnifiedLLMConfigIcon = defineComponent({
  name: 'UnifiedLLMConfigIcon',
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
        d: 'M13 10V3L4 14h7v7l9-11h-7z',
      }),
    ]);
  },
});

const GalleryIcon = defineComponent({
  name: 'GalleryIcon',
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
        d: 'M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z',
      }),
    ]);
  },
});

const AgentIcon = defineComponent({
  name: 'AgentIcon',
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
        d: 'M9.75 3.104v5.714a2.25 2.25 0 01-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 014.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0112 15a9.065 9.065 0 00-6.23-.693L5 14.5m14.8.8l1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0112 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5',
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
  { label: 'Chat Arena', route: '/studio/chat/arena', routeName: 'ChatArena', icon: ChatArenaIcon },
  { label: 'Agents', route: '/studio/agents', routeName: 'Agents', icon: AgentIcon },
  { label: 'Image Gallery', route: '/studio/gallery', routeName: 'Gallery', icon: GalleryIcon },
  { label: 'API Keys', route: '/studio/apikeys', routeName: 'ApiKeys', icon: ApiKeyIcon },
  { label: 'LLM Providers', route: '/studio/llmproviders', routeName: 'LLMProviders', icon: LLMProviderIcon },
  { label: 'User LLM Configs', route: '/studio/userllmconfigs', routeName: 'UserLLMConfigs', icon: UserLLMConfigIcon },
  { label: 'Unified LLM Config', route: '/studio/unifiedllmconfig', routeName: 'UnifiedLLMConfig', icon: UnifiedLLMConfigIcon },
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
