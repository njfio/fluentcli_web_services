<template>
  <div class="configuration-editor">
    <h3>{{ isNew ? 'Create' : 'Edit' }} Configuration</h3>
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="name">Name:</label>
        <input type="text" id="name" v-model="editedConfiguration.name" required>
      </div>
      <div class="form-group">
        <label for="data">Data (JSON):</label>
        <textarea id="data" v-model="jsonData" rows="10" required></textarea>
      </div>
      <div class="form-actions">
        <button type="submit" class="save-button">Save</button>
        <button type="button" @click="$emit('cancel')" class="cancel-button">Cancel</button>
      </div>
    </form>
  </div>
</template>
  
<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface Configuration {
  id?: string;
  name: string;
  data: any;
}

const props = defineProps<{
  data: Configuration;
}>();

const emit = defineEmits<{
  (e: 'save', configuration: Configuration): void;
  (e: 'cancel'): void;
}>();

const editedConfiguration = ref<Configuration>({ ...props.data });
const jsonData = ref(JSON.stringify(props.data.data, null, 2));

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  console.log('ConfigurationEditor received new data:', newData);
  editedConfiguration.value = { ...newData };
  jsonData.value = JSON.stringify(newData.data, null, 2);
}, { deep: true });

const handleSubmit = () => {
  try {
    editedConfiguration.value.data = JSON.parse(jsonData.value);
    console.log('ConfigurationEditor submitting:', editedConfiguration.value);
    emit('save', editedConfiguration.value);
  } catch (error) {
    alert('Invalid JSON data. Please check your input.');
  }
};
</script>
  
<style scoped>
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


.configuration-editor {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.configuration-editor h3 {
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