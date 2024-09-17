<template>
    <div class="amber-store-editor">
      <h3>{{ isNew ? 'Create' : 'Edit' }} Amber Store</h3>
      <form @submit.prevent="handleSubmit">
        <div>
          <label for="data">Data (JSON):</label>
          <textarea id="data" v-model="jsonData" rows="10" required></textarea>
        </div>
        <div>
          <button type="submit">Save</button>
          <button type="button" @click="$emit('cancel')">Cancel</button>
        </div>
      </form>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, watch } from 'vue';
  
  interface AmberStore {
    id?: string;
    data: any;
  }
  
  const props = defineProps<{
    data: AmberStore;
  }>();
  
  const emit = defineEmits<{
    (e: 'save', amberStore: AmberStore): void;
    (e: 'cancel'): void;
  }>();
  
  const editedAmberStore = ref<AmberStore>({ ...props.data });
  const jsonData = ref(JSON.stringify(props.data.data, null, 2));
  
  const isNew = computed(() => !props.data.id);
  
  watch(() => props.data, (newData) => {
    editedAmberStore.value = { ...newData };
    jsonData.value = JSON.stringify(newData.data, null, 2);
  }, { deep: true });
  
  const handleSubmit = () => {
    try {
      editedAmberStore.value.data = JSON.parse(jsonData.value);
      emit('save', editedAmberStore.value);
    } catch (error) {
      alert('Invalid JSON data. Please check your input.');
    }
  };
  </script>
  
  <style scoped>
  .amber-store-editor {
    margin-top: 20px;
  }
  form > div {
    margin-bottom: 15px;
  }
  label {
    display: block;
    margin-bottom: 5px;
  }
  textarea {
    width: 100%;
    padding: 5px;
  }
  button {
    margin-right: 10px;
  }
  </style>