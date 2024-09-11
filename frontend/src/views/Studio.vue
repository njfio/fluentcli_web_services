<template>
    <div class="studio">
      <StudioSidebar />
      <div class="studio-main">
        <StudioHeader />
        <StudioWorkspace />
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  import StudioSidebar from '@/components/studio/StudioSidebar.vue';
  import StudioHeader from '@/components/studio/StudioHeader.vue';
  import StudioWorkspace from '@/components/studio/StudioWorkspace.vue';
  
  export default defineComponent({
    name: 'Studio',
    components: {
      StudioSidebar,
      StudioHeader,
      StudioWorkspace,
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
  }
  </style>