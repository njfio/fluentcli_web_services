<template>
  <div class="amber-store-editor">
    <h3>{{ isNew ? 'Create' : 'Edit' }} Amber Store</h3>
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="name">Name:</label>
        <input id="name" v-model="editedAmberStore.name" required>
      </div>
      <div class="form-group">
        <label for="data">Data (YAML):</label>
        <textarea id="data" v-model="yamlData" rows="10" required @input="validateYaml"></textarea>
      </div>
      <div class="form-group">
        <label for="secure_key_hash">Secure Key Hash:</label>
        <input id="secure_key_hash" v-model="editedAmberStore.secure_key_hash" required>
      </div>
      <div v-if="yamlError" class="error">
        {{ yamlError }}
      </div>
      <div class="form-actions">
        <button type="submit" :disabled="!!yamlError" class="save-button">Save</button>
        <button type="button" @click="$emit('cancel')" class="cancel-button">Cancel</button>
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
const yamlError = ref('');

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  editedAmberStore.value = { ...newData };
  yamlData.value = yaml.dump(yaml.load(newData.data) || {}, { indent: 2 });
}, { deep: true });

function validateYaml() {
  try {
    yaml.load(yamlData.value);
    yamlError.value = '';
  } catch (error) {
    if (error instanceof Error) {
      yamlError.value = `Invalid YAML: ${error.message}`;
    } else {
      yamlError.value = 'Invalid YAML';
    }
  }
}

const handleSubmit = () => {
  if (!yamlError.value) {
    editedAmberStore.value.data = yamlData.value;
    emit('save', editedAmberStore.value);
  }
};
</script>

<style scoped>
.amber-store-editor {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.amber-store-editor h3 {
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

.error {
  color: #e74c3c;
  margin-bottom: 10px;
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

.save-button:hover:not(:disabled) {
  background-color: #27ae60;
}

.save-button:disabled {
  background-color: #95a5a6;
  cursor: not-allowed;
}

.cancel-button {
  background-color: #e74c3c;
  color: #fff;
}

.cancel-button:hover {
  background-color: #c0392b;
}
</style>