<template>
  <div class="docker-file-editor flex flex-col h-full">
    <h2 class="text-2xl font-bold mb-4">{{ isNewDockerFile ? 'Create New Docker File' : 'Edit Docker File' }}</h2>
    <form @submit.prevent="saveDockerFile" class="flex flex-col flex-grow">
      <div class="grid grid-cols-2 gap-4 mb-4">
        <div>
          <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
          <input type="text" id="name" v-model="dockerFile.name" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
        </div>
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
          <input type="text" id="description" v-model="dockerFile.description" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
        </div>
      </div>
      <div class="flex-grow flex flex-col">
        <label for="content" class="block text-sm font-medium text-gray-700 mb-2">Content</label>
        <div class="flex-grow relative">
          <textarea
            id="content"
            v-model="dockerFile.content"
            class="absolute inset-0 w-full h-full resize-none border border-gray-300 rounded-md shadow-sm p-2 font-mono"
          ></textarea>
        </div>
      </div>
      <div class="flex justify-end space-x-2 mt-4">
        <button type="button" @click="cancel" class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50">
          Cancel
        </button>
        <button type="submit" class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700">
          {{ isNewDockerFile ? 'Create' : 'Save' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRoute, useRouter } from 'vue-router';

export default defineComponent({
  name: 'DockerFileEditor',
  setup() {
    const store = useStore();
    const route = useRoute();
    const router = useRouter();

    const dockerFile = ref({
      id: '',
      name: '',
      description: '',
      content: '',
    });

    const isNewDockerFile = computed(() => !route.params.id);

    onMounted(async () => {
      if (!isNewDockerFile.value) {
        const dockerFileId = route.params.id as string;
        const fetchedDockerFile = await store.dispatch('studio/fetchDockerFileById', dockerFileId);
        if (fetchedDockerFile) {
          dockerFile.value = { ...fetchedDockerFile };
        }
      }
    });

    const saveDockerFile = async () => {
      try {
        if (isNewDockerFile.value) {
          await store.dispatch('studio/createDockerFile', dockerFile.value);
        } else {
          await store.dispatch('studio/updateDockerFile', dockerFile.value);
        }
        navigateBack();
      } catch (error) {
        console.error('Error saving Docker file:', error);
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
        router.push({ name: 'DockerFiles' });
      }
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
  height: calc(100vh - 64px); /* Adjust this value based on your layout */
}
</style>