<template>
  <div class="configuration-editor">
    <h1 class="text-2xl font-bold mb-6">{{ isNewConfiguration ? 'Create New Configuration' : 'Edit Configuration' }}
    </h1>
    <form @submit.prevent="saveConfiguration" class="space-y-6">
      <div class="bg-white shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
              <input type="text" id="name" v-model="configuration.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
            </div>
            <div class="sm:col-span-6">
              <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
              <textarea id="description" v-model="configuration.description" rows="3"
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"></textarea>
            </div>
            <div class="sm:col-span-6">
              <label for="data" class="block text-sm font-medium text-gray-700">Configuration Data (JSON)</label>
              <div class="mt-1 border border-gray-300 rounded-md overflow-hidden" style="height: calc(100vh - 400px);">
                <MonacoEditor v-model="editorContent" language="json" :options="editorOptions" @change="updateContent"
                  class="h-full" />
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="flex justify-end">
        <button type="button" @click="cancel"
          class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
          Cancel
        </button>
        <button type="submit"
          class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
          {{ isNewConfiguration ? 'Create' : 'Update' }}
        </button>
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
  name: 'ConfigurationEditor',
  components: {
    MonacoEditor,
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const route = useRoute();

    const configuration = ref({
      id: '',
      name: '',
      description: '',
      data: '',
    });

    const editorContent = ref('');

    const isNewConfiguration = computed(() => route.params.id === 'new');

    const editorOptions = {
      minimap: { enabled: false },
      lineNumbers: 'on',
      roundedSelection: false,
      scrollBeyondLastLine: false,
      readOnly: false,
      theme: 'vs-light',
    };

    onMounted(async () => {
      if (!isNewConfiguration.value) {
        const id = route.params.id as string;
        console.log('Fetching configuration with ID:', id);
        const fetchedConfiguration = await store.dispatch('studio/fetchConfigurationById', id);
        console.log('Fetched configuration:', fetchedConfiguration);
        configuration.value = { ...fetchedConfiguration };
        editorContent.value = typeof configuration.value.data === 'string'
          ? configuration.value.data
          : JSON.stringify(configuration.value.data, null, 2);
        console.log('Editor content set to:', editorContent.value);
      }
    });

    const updateContent = (value: string) => {
      console.log('Updating content:', value);
      editorContent.value = value;
      try {
        configuration.value.data = JSON.parse(value);
      } catch (error) {
        console.error('Error parsing JSON:', error);
        configuration.value.data = value;
      }
    };

    const saveConfiguration = async () => {
      try {
        console.log('Saving configuration:', configuration.value);
        const result = isNewConfiguration.value
          ? await store.dispatch('studio/createConfiguration', configuration.value)
          : await store.dispatch('studio/updateConfiguration', configuration.value);
        console.log('Save result:', result);
        router.push({ name: 'Configurations' });
      } catch (error) {
        console.error('Error saving Configuration:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const cancel = () => {
      router.push({ name: 'Configurations' });
    };

    // Watch for changes in the editor content
    watch(editorContent, (newValue) => {
      console.log('Editor content changed:', newValue);
      try {
        configuration.value.data = JSON.parse(newValue);
      } catch (error) {
        console.error('Error parsing JSON:', error);
        configuration.value.data = newValue;
      }
    });

    return {
      configuration,
      editorContent,
      isNewConfiguration,
      editorOptions,
      saveConfiguration,
      cancel,
      updateContent,
    };
  },
});
</script>

<style scoped>
.configuration-editor {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}
</style>