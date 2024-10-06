<template>
  <header class="bg-white shadow-sm">
    <div class="flex justify-between items-center p-4">
      <h2 class="text-2xl font-semibold text-gray-800">{{ pageTitle }}</h2>
      <div class="flex items-center">
        <span v-if="user" class="mr-4 text-gray-600">{{ user.name }}</span>
        <button @click="logout" class="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded transition duration-200">
          Logout
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue';
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

const logout = () => {
  store.dispatch('logout');
  router.push('/login');
};
</script>