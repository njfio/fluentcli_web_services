<template>
  <div class="docker-file-editor">
    <h1 class="text-2xl font-bold mb-6">{{ isNewDockerFile ? 'Create New Docker File' : 'Edit Docker File' }}</h1>
    <div class="flex justify-end mb-6">
      <button type="button" @click="cancel"
        class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
        Cancel
      </button>
      <button type="submit" form="docker-file-form"
        class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
        {{ isNewDockerFile ? 'Create' : 'Update' }}
      </button>
    </div>
    <form id="docker-file-form" @submit.prevent="saveDockerFile" class="space-y-6">
      <div class="bg-white shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
              <input type="text" id="name" v-model="dockerFile.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
            </div>
            <div class="sm:col-span-6">
              <label for="content" class="block text-sm font-medium text-gray-700">Dockerfile Content</label>
              <div class="mt-1 border border-gray-300 rounded-md overflow-hidden" style="height: calc(100vh - 400px);">
                <MonacoEditor v-model="dockerFile.content" language="dockerfile" class="h-full" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
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

    onMounted(async () => {
      if (!isNewDockerFile.value) {
        const id = route.params.id as string;
        const fetchedDockerFile = await store.dispatch('studio/fetchDockerFileById', id);
        dockerFile.value = { ...fetchedDockerFile };
      }
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

    return {
      dockerFile,
      isNewDockerFile,
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