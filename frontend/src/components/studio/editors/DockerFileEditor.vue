<template>
  <div class="docker-file-editor bg-white rounded-lg shadow-md p-6 w-full h-full flex flex-col">
    <h3 class="text-2xl font-bold mb-6">{{ isNew ? 'Create' : 'Edit' }} Docker File</h3>
    <form @submit.prevent="handleSubmit" class="flex flex-col flex-grow">
      <div class="mb-4">
        <label for="name" class="block text-sm font-medium text-gray-700">Name:</label>
        <input id="name" v-model="editedDockerFile.name" required
               class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
      <div class="flex-grow flex flex-col">
        <label for="content" class="block text-sm font-medium text-gray-700 mb-2">Content:</label>
        <textarea id="content" v-model="editedDockerFile.content" required
                  class="flex-grow w-full p-2 border rounded-md resize-none focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"></textarea>
      </div>
      <div class="flex justify-end space-x-4 mt-4">
        <button type="button" @click="handleCancel" 
                class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
          Cancel
        </button>
        <button type="submit"
                class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
          Save
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useStore } from 'vuex';

interface DockerFile {
  id?: string;
  name: string;
  content: string;
}

const props = defineProps<{
  'docker-file'?: DockerFile;
  data?: DockerFile;
}>();

const emit = defineEmits<{
  (e: 'save', dockerFile: DockerFile): void;
  (e: 'cancel'): void;
}>();

const route = useRoute();
const router = useRouter();
const store = useStore();

const editedDockerFile = ref<DockerFile>({ name: '', content: '' });

onMounted(() => {
  if (props['docker-file']) {
    editedDockerFile.value = { ...props['docker-file'] };
  } else if (props.data) {
    editedDockerFile.value = { ...props.data };
  }
});

const isNew = computed(() => !editedDockerFile.value.id);

const handleSubmit = async () => {
  try {
    await store.dispatch('studio/saveDockerFile', editedDockerFile.value);
    
    if (route.query.returnToJobDetails) {
      router.push({ name: 'JobDetail', params: { id: route.query.returnToJobDetails as string } });
    } else {
      emit('save', editedDockerFile.value);
    }
  } catch (error) {
    console.error('Error saving Docker file:', error);
    alert('An error occurred while saving the Docker file. Please try again.');
  }
};

const handleCancel = () => {
  if (route.query.returnToJobDetails) {
    router.push({ name: 'JobDetail', params: { id: route.query.returnToJobDetails as string } });
  } else {
    emit('cancel');
  }
};
</script>