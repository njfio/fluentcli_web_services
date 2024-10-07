<template>
  <div class="pipeline-editor flex flex-col h-full">
    <h2 class="text-2xl font-bold mb-4">{{ isNewPipeline ? 'Create New Pipeline' : 'Edit Pipeline' }}</h2>
    <div v-if="loading" class="text-center py-4">Loading pipeline data...</div>
    <div v-else-if="error" class="text-red-500 py-4">{{ error }}</div>
    <form v-else @submit.prevent="savePipeline" class="flex flex-col flex-grow">
      <div class="grid grid-cols-2 gap-4 mb-4">
        <div>
          <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
          <input type="text" id="name" v-model="pipeline.name" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
        </div>
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
          <input type="text" id="description" v-model="pipeline.description" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
        </div>
      </div>
      <div class="flex-grow flex flex-col">
        <label for="data" class="block text-sm font-medium text-gray-700 mb-2">Pipeline Data</label>
        <div class="flex-grow relative">
          <textarea
            id="data"
            v-model="formattedData"
            class="absolute inset-0 w-full h-full resize-none border border-gray-300 rounded-md shadow-sm p-2 font-mono"
            @input="handleInput"
          ></textarea>
        </div>
      </div>
      <div class="flex justify-end space-x-2 mt-4">
        <button type="button" @click="cancel" class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50">
          Cancel
        </button>
        <button type="submit" class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700">
          {{ isNewPipeline ? 'Create' : 'Save' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted, watch } from 'vue';
import { useStore } from 'vuex';
import { useRoute, useRouter } from 'vue-router';
import yaml from 'js-yaml';

export default defineComponent({
  name: 'PipelineEditor',
  setup() {
    const store = useStore();
    const route = useRoute();
    const router = useRouter();

    const pipeline = ref({
      id: '',
      name: '',
      description: '',
      data: '',
    });
    const formattedData = ref('');
    const loading = ref(false);
    const error = ref('');

    const isNewPipeline = computed(() => !route.params.id);

    onMounted(async () => {
      console.log('PipelineEditor mounted');
      console.log('Route params:', route.params);
      if (!isNewPipeline.value) {
        await fetchPipeline();
      }
    });

    const fetchPipeline = async () => {
      loading.value = true;
      error.value = '';
      try {
        const pipelineId = route.params.id as string;
        console.log('Fetching pipeline with ID:', pipelineId);
        const fetchedPipeline = await store.dispatch('studio/fetchPipelineById', pipelineId);
        console.log('Fetched pipeline:', fetchedPipeline);
        if (fetchedPipeline) {
          pipeline.value = { ...fetchedPipeline };
          formattedData.value = formatYaml(pipeline.value.data);
          console.log('Pipeline data set:', pipeline.value);
        } else {
          console.error('Fetched pipeline is null or undefined');
          error.value = 'Failed to fetch pipeline data';
        }
      } catch (err: any) {
        console.error('Error fetching pipeline:', err);
        error.value = `Error fetching pipeline: ${err.message || 'Unknown error'}`;
      } finally {
        loading.value = false;
      }
    };

    const formatYaml = (data: string): string => {
      try {
        const parsedData = yaml.load(data);
        return yaml.dump(parsedData);
      } catch (error) {
        console.error('Error formatting YAML:', error);
        return data;
      }
    };

    const handleInput = (event: Event) => {
      const target = event.target as HTMLTextAreaElement;
      pipeline.value.data = target.value;
    };

    const savePipeline = async () => {
      try {
        console.log('Saving pipeline:', pipeline.value);
        if (isNewPipeline.value) {
          await store.dispatch('studio/createPipeline', pipeline.value);
        } else {
          await store.dispatch('studio/updatePipeline', pipeline.value);
        }
        console.log('Pipeline saved successfully');
        navigateBack();
      } catch (err: any) {
        console.error('Error saving pipeline:', err);
        error.value = `Error saving pipeline: ${err.message || 'Unknown error'}`;
      }
    };

    const cancel = () => {
      console.log('Cancelling pipeline edit');
      navigateBack();
    };

    const navigateBack = () => {
      const returnToJobDetails = route.query.returnToJobDetails;
      console.log('Navigating back, returnToJobDetails:', returnToJobDetails);
      if (returnToJobDetails) {
        router.push({ name: 'JobDetail', params: { id: returnToJobDetails as string } });
      } else {
        router.push({ name: 'Pipelines' });
      }
    };

    return {
      pipeline,
      formattedData,
      isNewPipeline,
      loading,
      error,
      savePipeline,
      cancel,
      handleInput,
    };
  },
});
</script>

<style scoped>
.pipeline-editor {
  height: calc(100vh - 64px); /* Adjust this value based on your layout */
}
</style>
