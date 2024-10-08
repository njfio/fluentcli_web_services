<template>
  <div :class="{ 'dark': isDarkMode }">
    <router-view v-slot="{ Component }">
      <component :is="Component" />
    </router-view>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onMounted } from 'vue';
import { useStore } from 'vuex';

const store = useStore();

const isDarkMode = computed(() => store.getters['theme/isDarkMode']);

watch(isDarkMode, (newValue) => {
  if (newValue) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}, { immediate: true });

onMounted(() => {
  store.dispatch('theme/initDarkMode');
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  @apply text-gray-900 dark:text-white;
}

body {
  @apply bg-white dark:bg-gray-900;
  margin: 0;
  padding: 0;
}

.dark {
  color-scheme: dark;
}
</style>