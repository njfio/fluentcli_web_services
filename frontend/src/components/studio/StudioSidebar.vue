<template>
    <div class="studio-sidebar">
      <h3>Project Explorer</h3>
      <TreeView :items="projectStructure" @item-click="handleItemClick" />
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import TreeView from '@/components/common/TreeView.vue';
  
  export default defineComponent({
    name: 'StudioSidebar',
    components: { TreeView },
    setup() {
      const projectStructure = ref([
        {
          name: 'Project',
          children: [
            { name: 'Configurations', children: [] },
            { name: 'Pipelines', children: [] },
            { name: 'Docker Files', children: [] },
          ],
        },
      ]);
  
      const handleItemClick = (item: any) => {
        console.log('Clicked item:', item);
        // Implement logic to open the clicked item in the workspace
      };
  
      return { projectStructure, handleItemClick };
    },
  });
  </script>
  
  <style scoped>
  .studio-sidebar {
    width: 250px;
    background-color: #f0f0f0;
    padding: 1rem;
    overflow-y: auto;
  }
  </style>