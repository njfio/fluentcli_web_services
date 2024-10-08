<template>
  <div class="pipeline-editor">
    <h1 class="text-2xl font-bold mb-6">{{ isNewPipeline ? 'Create New Pipeline' : 'Edit Pipeline' }}</h1>
    <div class="flex justify-end mb-6">
      <button type="button" @click="cancel"
        class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
        Cancel
      </button>
      <button type="submit" form="pipeline-form"
        class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
        {{ isNewPipeline ? 'Create' : 'Update' }}
      </button>
    </div>
    <form id="pipeline-form" @submit.prevent="savePipeline" class="space-y-6">
      <div class="bg-white shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
              <input type="text" id="name" v-model="pipeline.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
            </div>
            <div class="sm:col-span-6">
              <label for="data" class="block text-sm font-medium text-gray-700">Pipeline Data (YAML)</label>
              <div class="mt-1 border border-gray-300 rounded-md overflow-hidden" style="height: calc(100vh - 400px);">
                <MonacoEditor v-model="pipeline.data" language="yaml" :options="editorOptions" class="h-full"
                  @update:modelValue="onEditorUpdate" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
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

    const pipeline = ref({
      id: '',
      name: '',
      data: '',
    });

    const isNewPipeline = computed(() => route.params.id === 'new');

    const editorOptions = {
      minimap: { enabled: false },
      lineNumbers: 'on',
      roundedSelection: false,
      scrollBeyondLastLine: false,
      readOnly: false,
      theme: 'vs-light',
    };

    onMounted(async () => {
      if (!isNewPipeline.value) {
        const id = route.params.id as string;
        console.log('Fetching pipeline with ID:', id);
        const fetchedPipeline = await store.dispatch('studio/fetchPipelineById', id);
        console.log('Fetched pipeline:', fetchedPipeline);
        pipeline.value = { ...fetchedPipeline };
        console.log('Initial pipeline data:', pipeline.value.data);
      }
    });

    const onEditorUpdate = (value: string) => {
      console.log('Editor update:', value);
      pipeline.value.data = value;
    };

    const savePipeline = async () => {
      try {
        console.log('Saving pipeline:', pipeline.value);
        console.log('Pipeline data before save:', pipeline.value.data);
        if (isNewPipeline.value) {
          await store.dispatch('studio/createPipeline', pipeline.value);
        } else {
          await store.dispatch('studio/updatePipeline', pipeline.value);
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

    watch(() => pipeline.value.data, (newValue) => {
      console.log('Pipeline data changed:', newValue);
    });

    return {
      pipeline,
      isNewPipeline,
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
