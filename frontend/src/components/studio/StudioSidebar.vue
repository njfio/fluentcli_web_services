<template>
  <nav class="bg-gray-800 text-white w-64 flex-shrink-0">
    <div class="p-4">
      <h2 class="text-2xl font-semibold mb-4">Studio</h2>
      <ul>
        <li class="mb-2">
          <router-link
            to="/studio/dashboard"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'Dashboard' }"
          >
            Dashboard
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/jobs"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'Jobs' || $route.name === 'JobDetail' }"
          >
            Jobs
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/pipelines"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'Pipelines' || $route.name === 'PipelineEditor' }"
          >
            Pipelines
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/configurations"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'Configurations' || $route.name === 'ConfigurationEditor' }"
          >
            Configurations
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/dockerfiles"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'DockerFiles' || $route.name === 'DockerFileEditor' }"
          >
            Docker Files
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/amberstores"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'AmberStores' || $route.name === 'AmberStoreEditor' }"
          >
            Amber Stores
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/statefiles"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'StateFiles' }"
          >
            State Files
          </router-link>
        </li>
        <li class="mb-2">
          <router-link
            to="/studio/settings"
            class="block py-2 px-4 rounded transition-colors duration-200"
            :class="{ 'bg-primary-600': $route.name === 'Settings' }"
          >
            Settings
          </router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'StudioSidebar',
});
</script>

<style scoped>
.router-link-active {
  @apply bg-primary-600;
}
</style>