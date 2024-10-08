<template>
  <div :class="['sidebar', { 'collapsed': isCollapsed }]">
    <div class="toggle-button" @click="$emit('toggle')">
      <svg v-if="!isCollapsed" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
      </svg>
      <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 5l7 7-7 7M5 5l7 7-7 7" />
      </svg>
    </div>
    <nav>
      <ul>
        <li>
          <router-link to="/studio/dashboard" class="nav-item">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>
            <span v-if="!isCollapsed">Dashboard</span>
          </router-link>
        </li>
        <li>
          <router-link to="/studio/jobs" class="nav-item">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <span v-if="!isCollapsed">Jobs</span>
          </router-link>
        </li>
        <li>
          <router-link to="/studio/pipelines" class="nav-item">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
            <span v-if="!isCollapsed">Pipelines</span>
          </router-link>
        </li>
        <li>
          <router-link to="/studio/dockerfiles" class="nav-item">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
            <span v-if="!isCollapsed">Docker Files</span>
          </router-link>
        </li>
        <li>
          <router-link to="/studio/configurations" class="nav-item">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span v-if="!isCollapsed">Configurations</span>
          </router-link>
        </li>
        <li>
          <div class="amber-stores-dropdown nav-item" @click="toggleAmberStoresDropdown">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
            </svg>
            <span v-if="!isCollapsed">Amber Stores</span>
            <svg v-if="!isCollapsed" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-auto" :class="{ 'transform rotate-180': showAmberStoresDropdown }" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </div>
          <transition name="fade">
            <ul v-if="!isCollapsed && showAmberStoresDropdown" class="amber-stores-list">
              <li v-for="store in amberStores" :key="store.id">
                <a @click="selectAmberStore(store.id)" class="amber-store-item">{{ store.name }}</a>
              </li>
            </ul>
          </transition>
        </li>
        <li>
          <router-link to="/studio/settings" class="nav-item">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span v-if="!isCollapsed">Settings</span>
          </router-link>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';

export default defineComponent({
  name: 'StudioSidebar',
  props: {
    isCollapsed: {
      type: Boolean,
      required: true,
    },
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const showAmberStoresDropdown = ref(false);

    const amberStores = computed(() => store.getters['studio/getAmberStores']);

    onMounted(async () => {
      await store.dispatch('studio/fetchAmberStores');
    });

    const selectAmberStore = (id: string) => {
      router.push({ name: 'AmberStores', params: { id } });
    };

    const toggleAmberStoresDropdown = () => {
      showAmberStoresDropdown.value = !showAmberStoresDropdown.value;
    };

    return {
      amberStores,
      selectAmberStore,
      showAmberStoresDropdown,
      toggleAmberStoresDropdown,
    };
  },
});
</script>

<style scoped>
.sidebar {
  @apply h-full bg-gradient-to-b from-indigo-700 to-indigo-900 text-white transition-all duration-300 ease-in-out;
  width: 250px;
}

.sidebar.collapsed {
  width: 80px;
}

.toggle-button {
  @apply absolute top-4 -right-3 bg-indigo-500 rounded-full p-1 cursor-pointer shadow-lg transition-colors duration-300 hover:bg-indigo-600;
}

nav ul {
  @apply list-none p-0 mt-16;
}

.nav-item {
  @apply flex items-center px-6 py-3 text-sm font-medium text-indigo-100 transition-colors duration-300 hover:bg-indigo-600;
}

.nav-item svg {
  @apply mr-3 flex-shrink-0;
}

.nav-item.router-link-exact-active {
  @apply bg-indigo-800 text-white;
}

.amber-stores-dropdown {
  @apply cursor-pointer;
}

.amber-stores-list {
  @apply mt-1 ml-6 space-y-1;
}

.amber-store-item {
  @apply block px-4 py-2 text-sm text-indigo-100 hover:bg-indigo-600 rounded transition-colors duration-300;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@media (max-width: 768px) {
  .sidebar {
    width: 100%;
    height: auto;
  }

  .sidebar.collapsed {
    width: 100%;
  }

  .toggle-button {
    display: none;
  }

  nav ul {
    @apply flex flex-wrap justify-center mt-4;
  }

  .nav-item {
    @apply px-4 py-2;
  }

  .amber-stores-list {
    @apply absolute bg-indigo-800 rounded-md shadow-lg z-10;
    top: 100%;
    left: 0;
  }
}
</style>