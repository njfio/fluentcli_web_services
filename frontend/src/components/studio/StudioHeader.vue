<template>
  <header class="bg-gradient-to-r from-primary-600 to-primary-800 dark:from-gray-800 dark:to-gray-900 shadow-md">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center py-4">
        <div class="flex items-center">
          <button @click="$emit('toggleSidebar')"
            class="text-white mr-4 focus:outline-none hover:text-primary-200 transition-colors duration-200">
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <h2 class="text-2xl font-semibold text-white">{{ pageTitle }}</h2>
        </div>
        <div class="flex items-center space-x-4">
          <button @click="toggleDarkMode"
            class="text-white focus:outline-none hover:text-primary-200 transition-colors duration-200">
            <svg v-if="isDarkMode" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            <svg v-else class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
            </svg>
          </button>
          <!-- Rest of the header content -->
        </div>
      </div>
    </div>
    <!-- Breadcrumb navigation -->
  </header>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useStore } from 'vuex';

const route = useRoute();
const router = useRouter();
const store = useStore();

const pageTitle = computed(() => {
  const routeName = route.name as string;
  return routeName.charAt(0).toUpperCase() + routeName.slice(1);
});

const user = computed(() => store.state.user);
const showUserMenu = ref(false);
const isSearchFocused = ref(false);

const toggleUserMenu = () => {
  showUserMenu.value = !showUserMenu.value;
};

const logout = () => {
  store.dispatch('logout');
  router.push('/login');
};

// Close user menu when clicking outside
const closeUserMenu = (e: MouseEvent) => {
  if (showUserMenu.value && !(e.target as HTMLElement).closest('.user-menu')) {
    showUserMenu.value = false;
  }
};

// Add event listener for closing user menu
onMounted(() => {
  window.addEventListener('click', closeUserMenu);
});

// Remove event listener when component is unmounted
onUnmounted(() => {
  window.removeEventListener('click', closeUserMenu);
});

const props = defineProps<{
  isDarkMode: boolean
}>();

const emit = defineEmits(['toggleSidebar', 'toggleDarkMode']);

const toggleDarkMode = () => {
  console.log('Dark mode toggle clicked. Current state:', props.isDarkMode);
  emit('toggleDarkMode');
};
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>