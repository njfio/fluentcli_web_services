<template>
    <div class="studio">
      <StudioSidebar :isCollapsed="isSidebarCollapsed" @toggle="toggleSidebar" />
      <div class="studio-main">
        <StudioHeader @toggleSidebar="toggleSidebar" />
        <router-view></router-view>
      </div>
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
  
  .studio-main {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    background-color: #f5f5f5;
  }
  </style>