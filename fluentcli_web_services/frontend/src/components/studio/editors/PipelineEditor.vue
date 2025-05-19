<template>
  <div v-if="isThemeInitialized" class="pipeline-editor dark:bg-gray-900">
    <h1 class="text-2xl font-bold mb-6 dark:text-white">{{ isNewPipeline ? 'Create New Pipeline' : 'Edit Pipeline' }}
    </h1>
    <div class="flex justify-end mb-6">
      <button type="button" @click="cancel"
        class="bg-white dark:bg-gray-700 py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-900">
        Cancel
      </button>
      <button type="submit" form="pipeline-form"
        class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 dark:bg-primary-700 dark:hover:bg-primary-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-900">
        {{ isNewPipeline ? 'Create' : 'Update' }}
      </button>
    </div>
    <form id="pipeline-form" @submit.prevent="savePipeline" class="space-y-6">
      <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Name</label>
              <input type="text" id="name" v-model="pipelineData.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white">
            </div>
            <div class="sm:col-span-6">
              <label for="data" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Pipeline Data
                (YAML)</label>
              <div class="mt-1 border border-gray-300 dark:border-gray-600 rounded-md overflow-hidden dark:bg-gray-800"
                style="height: calc(100vh - 400px);">
                <MonacoEditor :key="currentTheme" v-model="pipelineData.data" language="yaml" :theme="currentTheme"
                  :options="editorOptions" class="h-full" @update:modelValue="onEditorUpdate" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
  <div v-else class="flex justify-center items-center h-screen">
    <p class="text-xl">Loading...</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted, watch } from 'vue';
import { useStore } from 'vuex';
import { useRouter, useRoute } from 'vue-router';
import MonacoEditor from './MonacoEditor.vue';

export default defineComponent({
  name: 'PipelineEditor',
  components: {
    MonacoEditor,
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const route = useRoute();

    const pipelineData = ref({
      id: '',
      name: '',
      data: '',
    });

    const isNewPipeline = computed(() => route.params.id === 'new');
    const isDarkMode = computed(() => store.getters['theme/isDarkMode']);
    const isThemeInitialized = computed(() => store.getters['theme/isInitialized']);
    const currentTheme = computed(() => isDarkMode.value ? 'vs-dark' : 'vs-light');

    const editorOptions = {
      minimap: { enabled: false },
      lineNumbers: 'on',
      roundedSelection: false,
      scrollBeyondLastLine: false,
      readOnly: false,
    };

    onMounted(async () => {
      if (!isNewPipeline.value) {
        const id = route.params.id as string;
        console.log('Fetching pipeline with ID:', id);
        await store.dispatch('studio/fetchPipelineById', id);
        const fetchedPipeline = store.getters['studio/getCurrentPipeline'];
        console.log('Fetched pipeline:', fetchedPipeline);
        if (fetchedPipeline) {
          pipelineData.value = { ...fetchedPipeline };
        }
        console.log('Initial pipeline data:', pipelineData.value.data);
      }
    });

    const onEditorUpdate = (value: string) => {
      console.log('Editor update:', value);
      pipelineData.value.data = value;
    };

    const savePipeline = async () => {
      try {
        console.log('Saving pipeline:', pipelineData.value);
        console.log('Pipeline data before save:', pipelineData.value.data);
        if (isNewPipeline.value) {
          await store.dispatch('studio/createPipeline', pipelineData.value);
        } else {
          await store.dispatch('studio/updatePipeline', pipelineData.value);
        }
        console.log('Pipeline saved successfully');
        router.push({ name: 'Pipelines' });
      } catch (error) {
        console.error('Error saving Pipeline:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const cancel = () => {
      router.push({ name: 'Pipelines' });
    };

    watch(() => pipelineData.value.data, (newValue) => {
      console.log('Pipeline data changed:', newValue);
    });

    return {
      pipelineData,
      isNewPipeline,
      currentTheme,
      isThemeInitialized,
      editorOptions,
      savePipeline,
      cancel,
      onEditorUpdate,
    };
  },
});
</script>

<style scoped>
.pipeline-editor {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}
</style>
