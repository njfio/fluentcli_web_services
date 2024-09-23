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
  
<<style scoped>
.docker-file-editor {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.docker-file-editor h3 {
  margin-top: 0;
  margin-bottom: 20px;
  font-size: 1.5rem;
}

.form-group {
  margin-bottom: 20px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input[type="text"], textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 3px;
  font-size: 1rem;
}

textarea {
  resize: vertical;
  min-height: 300px;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
}

.save-button, .cancel-button {
  padding: 10px 20px;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.3s ease;
}

.save-button {
  background-color: #2ecc71;
  color: #fff;
  margin-right: 10px;
}

.save-button:hover {
  background-color: #27ae60;
}

.cancel-button {
  background-color: #e74c3c;
  color: #fff;
}

.cancel-button:hover {
  background-color: #c0392b;
}
</style>