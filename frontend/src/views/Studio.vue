<template>
    <div class="studio">
      <nav class="studio-sidebar">
        <router-link to="/studio/dashboard">Dashboard</router-link>
        <router-link to="/studio/jobs">Jobs</router-link>
        <router-link to="/studio/pipelines">Pipelines</router-link>
        <router-link to="/studio/dockerfiles">Docker Files</router-link>
        <router-link to="/studio/configurations">Configurations</router-link>
        <router-link to="/studio/amberstores">Amber Stores</router-link>
        <!-- Include other navigation links as needed -->
      </nav>
      <main class="studio-main">
        <router-view />
      </main>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import StudioSidebar from '@/components/studio/StudioSidebar.vue';
  import StudioHeader from '@/components/studio/StudioHeader.vue';
  
  export default defineComponent({
    name: 'Studio',
    components: {
      StudioSidebar,
      StudioHeader,
    },
    setup() {
      const isSidebarCollapsed = ref(false);
  
      const toggleSidebar = () => {
        isSidebarCollapsed.value = !isSidebarCollapsed.value;
      };
  
      return {
        isSidebarCollapsed,
        toggleSidebar,
      };
    },
  });
  </script>
  
  <style scoped>
  .studio {
    display: flex;
    height: calc(100vh - 60px);
  }
  .studio-sidebar {
    width: 200px;
    background-color: #2c3e50;
    padding: 20px;
  }
  .studio-sidebar a {
    display: block;
    color: #ecf0f1;
    text-decoration: none;
    margin-bottom: 10px;
  }
  .studio-sidebar a.router-link-exact-active {
    font-weight: bold;
  }
  .studio-main {
    flex-grow: 1;
    padding: 20px;
  }
  </style>