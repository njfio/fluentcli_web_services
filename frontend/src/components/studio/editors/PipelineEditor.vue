<template>
  <div class="pipeline-editor bg-white rounded-lg shadow-md p-6 w-full h-full flex flex-col">
    <h3 class="text-2xl font-bold mb-6">{{ isNew ? 'Create' : 'Edit' }} Pipeline</h3>
    <form @submit.prevent="handleSubmit" class="flex flex-col flex-grow">
      <div class="mb-4">
        <label for="name" class="block text-sm font-medium text-gray-700">Name:</label>
        <input id="name" v-model="editedPipeline.name" required
               class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
      <div class="flex-grow flex flex-col">
        <label for="data" class="block text-sm font-medium text-gray-700 mb-2">Data (YAML):</label>
        <div class="relative flex-grow">
          <textarea id="data" v-model="formattedYaml" required @input="handleInput"
                    class="absolute inset-0 w-full h-full resize-none font-mono p-2"></textarea>
        </div>
      </div>
      <div v-if="yamlError" class="text-red-600 text-sm mt-2">
        {{ yamlError }}
      </div>
      <div class="flex justify-end space-x-4 mt-4">
        <button type="button" @click="handleCancel" 
                class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
          Cancel
        </button>
        <button type="submit" :disabled="!!yamlError"
                class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:bg-gray-400 disabled:cursor-not-allowed">
          Save
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useStore } from 'vuex';
import * as yaml from 'js-yaml';

interface Pipeline {
  id?: string;
  name: string;
  data: string;
}

const route = useRoute();
const router = useRouter();
const store = useStore();

const editedPipeline = ref<Pipeline>({ name: '', data: '' });
const yamlError = ref('');

const isNew = computed(() => !route.params.id);

const formattedYaml = computed({
  get: () => {
    try {
      const parsedData = yaml.load(editedPipeline.value.data);
      return yaml.dump(parsedData, { indent: 2 });
    } catch (error) {
      return editedPipeline.value.data;
    }
  },
  set: (value: string) => {
    editedPipeline.value.data = value;
  },
});

onMounted(async () => {
  if (!isNew.value) {
    try {
      const response = await store.dispatch('studio/fetchPipelineById', route.params.id);
      editedPipeline.value = response;
    } catch (error) {
      console.error('Error fetching pipeline:', error);
      alert('Failed to fetch pipeline data. Please try again.');
      router.push({ name: 'Pipelines' });
    }
  }
});

function handleInput() {
  validateYaml();
}

function validateYaml() {
  try {
    yaml.load(editedPipeline.value.data);
    yamlError.value = '';
  } catch (error) {
    if (error instanceof Error) {
      yamlError.value = `Invalid YAML: ${error.message}`;
    } else {
      yamlError.value = 'Invalid YAML';
    }
  }
}

const handleSubmit = async () => {
  try {
    await store.dispatch('studio/savePipeline', editedPipeline.value);
    router.push({ name: 'Pipelines' });
  } catch (error) {
    console.error('Error saving pipeline:', error);
    alert('An error occurred while saving the pipeline. Please try again.');
  }
};

const handleCancel = () => {
  router.push({ name: 'Pipelines' });
};

watch(formattedYaml, (newValue) => {
  editedPipeline.value.data = newValue;
});
</script>

<style scoped>
.pipeline-editor {
  height: calc(100vh - 100px); /* Adjust this value based on your layout */
}
</style>
