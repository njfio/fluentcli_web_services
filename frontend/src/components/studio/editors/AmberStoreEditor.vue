<template>
    <div class="amber-store-editor">
      <h3>{{ isNew ? 'Create' : 'Edit' }} Amber Store</h3>
      <form @submit.prevent="handleSubmit">
        <div>
          <label for="name">Name:</label>
          <input id="name" v-model="editedAmberStore.name" required>
        </div>
        <div>
          <label for="data">Data (YAML):</label>
          <textarea id="data" v-model="yamlData" rows="10" required></textarea>
        </div>
        <div>
          <label for="secure_key_hash">Secure Key Hash:</label>
          <input id="secure_key_hash" v-model="editedAmberStore.secure_key_hash" required>
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
import * as yaml from 'js-yaml';

interface AmberStore {
  id?: string;
  name: string;
  data: string;
  secure_key_hash: string;
}

const props = defineProps<{
  data: AmberStore;
}>();

const emit = defineEmits<{
  (e: 'save', amberStore: AmberStore): void;
  (e: 'cancel'): void;
}>();

const editedAmberStore = ref<AmberStore>({ ...props.data });
const yamlData = ref(yaml.dump(yaml.load(props.data.data) || {}, { indent: 2 }));

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  editedAmberStore.value = { ...newData };
  yamlData.value = yaml.dump(yaml.load(newData.data) || {}, { indent: 2 });
}, { deep: true });

const handleSubmit = () => {
  try {
    const parsedData = yaml.load(yamlData.value);
    editedAmberStore.value.data = yaml.dump(parsedData);
    emit('save', editedAmberStore.value);
  } catch (error) {
    alert('Invalid YAML data. Please check your input.');
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