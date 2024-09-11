<template>
    <div class="studio-header">
      <div class="tabs">
        <div
          v-for="tab in openTabs"
          :key="tab.id"
          :class="['tab', { active: tab.id === activeTabId }]"
          @click="setActiveTab(tab.id)"
        >
          {{ tab.name }}
          <button class="close-tab" @click.stop="closeTab(tab.id)">&times;</button>
        </div>
      </div>
      <div class="actions">
        <button @click="saveCurrentTab">Save</button>
        <button @click="runCurrentTab">Run</button>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, computed } from 'vue';
  import { useStore } from 'vuex';
  
  export default defineComponent({
    name: 'StudioHeader',
    setup() {
      const store = useStore();
  
      const openTabs = computed(() => store.state.studio.openTabs);
      const activeTabId = computed(() => store.state.studio.activeTabId);
  
      const setActiveTab = (tabId: string) => {
        store.commit('studio/setActiveTab', tabId);
      };
  
      const closeTab = (tabId: string) => {
        store.commit('studio/closeTab', tabId);
      };
  
      const saveCurrentTab = () => {
        // Implement save logic
      };
  
      const runCurrentTab = () => {
        // Implement run logic
      };
  
      return {
        openTabs,
        activeTabId,
        setActiveTab,
        closeTab,
        saveCurrentTab,
        runCurrentTab,
      };
    },
  });
  </script>
  
  <style scoped>
  .studio-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: #e0e0e0;
    padding: 0.5rem;
  }
  
  .tabs {
    display: flex;
  }
  
  .tab {
    padding: 0.5rem 1rem;
    background-color: #d0d0d0;
    margin-right: 0.5rem;
    cursor: pointer;
  }
  
  .tab.active {
    background-color: #ffffff;
  }
  
  .close-tab {
    margin-left: 0.5rem;
    border: none;
    background: none;
    cursor: pointer;
  }
  
  .actions button {
    margin-left: 0.5rem;
  }
  </style>