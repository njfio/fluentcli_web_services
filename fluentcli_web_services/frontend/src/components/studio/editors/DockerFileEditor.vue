<template>
  <div v-if="isThemeInitialized" class="docker-file-editor dark:bg-gray-900">
    <h1 class="text-2xl font-bold mb-6 dark:text-white">
      {{ isNewDockerFile ? 'Create New Docker File' : 'Edit Docker File' }}
    </h1>
    <div class="flex justify-end mb-6">
      <button type="button" @click="cancel"
        class="bg-white dark:bg-gray-700 py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-900">
        Cancel
      </button>
      <button type="submit" form="docker-file-form"
        class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 dark:bg-primary-700 dark:hover:bg-primary-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-900">
        {{ isNewDockerFile ? 'Create' : 'Update' }}
      </button>
    </div>
    <form id="docker-file-form" @submit.prevent="saveDockerFile" class="space-y-6">
      <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Name</label>
              <input type="text" id="name" v-model="dockerFileData.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white">
            </div>
            <div class="sm:col-span-6">
              <label for="content" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Docker File
                Content</label>
              <div class="mt-1 border border-gray-300 dark:border-gray-600 rounded-md overflow-hidden dark:bg-gray-800"
                style="height: calc(100vh - 400px);">
                <MonacoEditor :key="currentTheme" v-model="dockerFileData.content" language="dockerfile"
                  :theme="currentTheme" :options="editorOptions" class="h-full" @update:modelValue="onEditorUpdate" />
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
  name: 'DockerFileEditor',
  components: {
    MonacoEditor,
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const route = useRoute();

    const dockerFileData = ref({
      id: '',
      name: '',
      content: '',
    });

    const isNewDockerFile = computed(() => route.params.id === 'new');
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
      if (!isNewDockerFile.value) {
        const id = route.params.id as string;
        console.log('Fetching docker file with ID:', id);
        await store.dispatch('studio/fetchDockerFileById', id);
        const fetchedDockerFile = store.getters['studio/getCurrentDockerFile'];
        console.log('Fetched docker file:', fetchedDockerFile);
        if (fetchedDockerFile) {
          dockerFileData.value = { ...fetchedDockerFile };
        }
        console.log('Initial docker file content:', dockerFileData.value.content);
      }
    });

    const onEditorUpdate = (value: string) => {
      console.log('Editor update:', value);
      dockerFileData.value.content = value;
    };

    const saveDockerFile = async () => {
      try {
        console.log('Saving docker file:', dockerFileData.value);
        console.log('Docker file content before save:', dockerFileData.value.content);
        if (isNewDockerFile.value) {
          await store.dispatch('studio/createDockerFile', dockerFileData.value);
        } else {
          await store.dispatch('studio/updateDockerFile', dockerFileData.value);
        }
        console.log('Docker file saved successfully');
        router.push({ name: 'DockerFiles' });
      } catch (error) {
        console.error('Error saving Docker File:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const cancel = () => {
      router.push({ name: 'DockerFiles' });
    };

    watch(() => dockerFileData.value.content, (newValue) => {
      console.log('Docker file content changed:', newValue);
    });

    return {
      dockerFileData,
      isNewDockerFile,
      currentTheme,
      isThemeInitialized,
      editorOptions,
      saveDockerFile,
      cancel,
      onEditorUpdate,
    };
  },
});
</script>

<style scoped>
.docker-file-editor {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}
</style>