<template>
    <div class="docker-file-editor">
      <h3>{{ isNew ? 'Create' : 'Edit' }} Docker File</h3>
      <form @submit.prevent="handleSubmit">
        <div>
          <label for="name">Name:</label>
          <input type="text" id="name" v-model="editedDockerFile.name" required>
        </div>
        <div>
          <label for="content">Content:</label>
          <textarea id="content" v-model="editedDockerFile.content" rows="10" required></textarea>
        </div>
        <div>
          <button type="submit">Save</button>
          <button type="button" @click="$emit('cancel')">Cancel</button>
        </div>
      </form>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue';
  
  interface DockerFile {
    id?: string;
    name: string;
    content: string;
  }
  
  const props = defineProps<{
    dockerFile: DockerFile | null;
  }>();
  
  const emit = defineEmits<{
    (e: 'save', dockerFile: DockerFile): void;
    (e: 'cancel'): void;
  }>();
  
  const editedDockerFile = ref<DockerFile>({
    id: props.dockerFile?.id,
    name: props.dockerFile?.name || '',
    content: props.dockerFile?.content || '',
  });
  
  const isNew = computed(() => !props.dockerFile?.id);
  
  const handleSubmit = () => {
    emit('save', editedDockerFile.value);
  };
  </script>
  
  <style scoped>
  .docker-file-editor {
    margin-top: 20px;
  }
  form > div {
    margin-bottom: 15px;
  }
  label {
    display: block;
    margin-bottom: 5px;
  }
  input[type="text"], textarea {
    width: 100%;
    padding: 5px;
  }
  button {
    margin-right: 10px;
  }
  </style>