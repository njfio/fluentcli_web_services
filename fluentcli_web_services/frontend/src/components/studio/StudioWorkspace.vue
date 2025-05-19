<template>
    <div class="studio-workspace">
      <component
        :is="activeComponent"
        v-if="activeComponent"
        :key="activeTabId"
        :data="activeTabData"
      />
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, computed } from 'vue';
  import { useStore } from 'vuex';
  import ConfigurationEditor from '@/components/studio/editors/ConfigurationEditor.vue';
  import PipelineEditor from '@/components/studio/editors/PipelineEditor.vue';
  import DockerFileEditor from '@/components/studio/editors/DockerFileEditor.vue';
  
  export default defineComponent({
    name: 'StudioWorkspace',
    components: {
      ConfigurationEditor,
      PipelineEditor,
      DockerFileEditor,
    },
    setup() {
      const store = useStore();
  
      const activeTabId = computed(() => store.state.studio.activeTabId);
      const activeTab = computed(() => store.getters['studio/activeTab']);
  
      const activeComponent = computed(() => {
        if (!activeTab.value) return null;
        switch (activeTab.value.type) {
          case 'configuration':
            return 'ConfigurationEditor';
          case 'pipeline':
            return 'PipelineEditor';
          case 'dockerFile':
            return 'DockerFileEditor';
          default:
            return null;
        }
      });
  
      const activeTabData = computed(() => activeTab.value?.data || {});
  
      return {
        activeTabId,
        activeComponent,
        activeTabData,
      };
    },
  });
  </script>
  
  <style scoped>
  .studio-workspace {
    flex-grow: 1;
    padding: 1rem;
    overflow-y: auto;
  }
  </style>