<template>
  <div class="configuration-editor flex flex-col h-full">
    <h2 class="text-2xl font-bold mb-4">{{ isNewConfiguration ? 'Create New Configuration' : 'Edit Configuration' }}</h2>
    <form @submit.prevent="saveConfiguration" class="flex flex-col flex-grow">
      <div class="mb-4">
        <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
        <input type="text" id="name" v-model="configuration.name" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
      </div>
      <div class="flex-grow flex flex-col">
        <label for="data" class="block text-sm font-medium text-gray-700 mb-2">Data (JSON)</label>
        <div class="flex-grow relative">
          <pre
            id="data"
            v-html="highlightedJson"
            class="absolute inset-0 w-full h-full overflow-auto border border-gray-300 rounded-md shadow-sm p-2 font-mono bg-white"
            contenteditable="true"
            @input="updateConfigurationData"
          ></pre>
        </div>
      </div>
      <div class="flex justify-end space-x-2 mt-4">
        <button type="button" @click="cancel" class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50">
          Cancel
        </button>
        <button type="submit" class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700">
          {{ isNewConfiguration ? 'Create' : 'Save' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRoute, useRouter } from 'vue-router';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';

hljs.registerLanguage('json', json);

export default defineComponent({
  name: 'ConfigurationEditor',
  setup() {
    const store = useStore();
    const route = useRoute();
    const router = useRouter();

    const configuration = ref({
      id: '',
      name: '',
      data: '',
    });

    const isNewConfiguration = computed(() => !route.params.id);

    const highlightedJson = computed(() => {
      try {
        const jsonObject = JSON.parse(configuration.value.data);
        const formattedJson = JSON.stringify(jsonObject, null, 2);
        return hljs.highlight(formattedJson, { language: 'json' }).value;
      } catch (error) {
        console.error('Error parsing or highlighting JSON:', error);
        return configuration.value.data;
      }
    });

    const updateConfigurationData = (event: Event) => {
      const target = event.target as HTMLPreElement;
      try {
        const jsonContent = target.innerText;
        JSON.parse(jsonContent); // Validate JSON
        configuration.value.data = jsonContent;
      } catch (error) {
        console.error('Error parsing JSON:', error);
        // Optionally, you can show an error message to the user here
      }
    };

    onMounted(async () => {
      if (!isNewConfiguration.value) {
        const configId = route.params.id as string;
        const config = await store.dispatch('studio/fetchConfigurationById', configId);
        if (config) {
          configuration.value = { ...config };
        }
      }
    });

    const saveConfiguration = async () => {
      try {
        const configToSave = {
          ...configuration.value,
        };

        if (isNewConfiguration.value) {
          await store.dispatch('studio/createConfiguration', configToSave);
        } else {
          await store.dispatch('studio/updateConfiguration', configToSave);
        }
        navigateBack();
      } catch (error) {
        console.error('Error saving configuration:', error);
        // Handle error (e.g., show error message to user)
      }
    };

    const cancel = () => {
      navigateBack();
    };

    const navigateBack = () => {
      const returnToJobDetails = route.query.returnToJobDetails;
      if (returnToJobDetails) {
        router.push({ name: 'JobDetail', params: { id: returnToJobDetails as string } });
      } else {
        router.push({ name: 'Configurations' });
      }
    };

    return {
      configuration,
      isNewConfiguration,
      highlightedJson,
      updateConfigurationData,
      saveConfiguration,
      cancel,
    };
  },
});
</script>

<style scoped>
.configuration-editor {
  height: calc(100vh - 64px); /* Adjust this value based on your layout */
}
</style>

<style>
/* Add these styles for JSON syntax highlighting */
.hljs-attr { color: #f92672; }
.hljs-string { color: #a6e22e; }
.hljs-number { color: #ae81ff; }
.hljs-boolean { color: #ae81ff; }
.hljs-null { color: #ae81ff; }
.hljs-literal { color: #ae81ff; }
</style>