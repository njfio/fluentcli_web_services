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
              <input type="text" id="name" v-model="dockerFile.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white">
            </div>
            <div class="sm:col-span-6">
              <label for="content" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Dockerfile
                Content</label>
              <div class="mt-1 border border-gray-300 dark:border-gray-600 rounded-md overflow-hidden dark:bg-gray-800"
                style="height: calc(100vh - 400px);">
                <MonacoEditor :key="currentTheme" v-model="dockerFile.content" language="dockerfile"
                  :theme="currentTheme" :options="editorOptions" class="h-full" />
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

    const dockerFile = ref({
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

    console.log('DockerFileEditor setup, initial isDarkMode:', isDarkMode.value);
    console.log('DockerFileEditor setup, initial currentTheme:', currentTheme.value);
    console.log('DockerFileEditor setup, isThemeInitialized:', isThemeInitialized.value);

    onMounted(async () => {
      if (!isNewDockerFile.value) {
        const id = route.params.id as string;
        const fetchedDockerFile = await store.dispatch('studio/fetchDockerFileById', id);
        dockerFile.value = { ...fetchedDockerFile };
      }
      console.log('DockerFileEditor mounted, current theme:', currentTheme.value);
    });

    const saveDockerFile = async () => {
      try {
        if (isNewDockerFile.value) {
          await store.dispatch('studio/createDockerFile', dockerFile.value);
        } else {
          await store.dispatch('studio/updateDockerFile', dockerFile.value);
        }
        router.push({ name: 'DockerFiles' });
      } catch (error) {
        console.error('Error saving Docker File:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const cancel = () => {
      router.push({ name: 'DockerFiles' });
    };

    // Watch for theme changes
    watch(isDarkMode, (newValue) => {
      console.log('DockerFileEditor: Dark mode changed:', newValue, 'New theme:', currentTheme.value);
    });

    // Watch for changes in the current theme
    watch(currentTheme, (newTheme) => {
      console.log('DockerFileEditor: Current theme changed to:', newTheme);
    });

    // Watch for theme initialization
    watch(isThemeInitialized, (initialized) => {
      console.log('DockerFileEditor: Theme initialization state:', initialized);
    });

    return {
      dockerFile,
      isNewDockerFile,
      currentTheme,
      isThemeInitialized,
      editorOptions,
      saveDockerFile,
      cancel,
    };
  },
});
</script>

<style scoped>
.docker-file-editor {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}
</style>