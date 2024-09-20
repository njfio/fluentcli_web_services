<template>
  <div class="pipeline-editor">
    <h3>{{ isNew ? 'Create' : 'Edit' }} Pipeline</h3>
    <form @submit.prevent="handleSubmit">
      <div>
        <label for="name">Name:</label>
        <input id="name" v-model="editedPipeline.name" required>
      </div>
      <div>
        <label for="data">Data (YAML):</label>
        <textarea id="data" v-model="yamlData" rows="10" required @input="validateYaml"></textarea>
      </div>
      <div v-if="yamlError" class="error">
        {{ yamlError }}
      </div>
      <div>
        <button type="submit" :disabled="!!yamlError">Save</button>
        <button type="button" @click="$emit('cancel')">Cancel</button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import * as yaml from 'js-yaml';

interface Pipeline {
  id: string;
  name: string;
  data: string;
}

const props = defineProps<{
  data: Pipeline;
}>();

const emit = defineEmits<{
  (e: 'save', pipeline: Pipeline): void;
  (e: 'cancel'): void;
}>();

const editedPipeline = ref<Pipeline>({ ...props.data });
const yamlData = ref(formatYaml(props.data.data));
const yamlError = ref('');

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  editedPipeline.value = { ...newData };
  yamlData.value = formatYaml(newData.data);
}, { deep: true });

function formatYaml(data: string): string {
  try {
    const parsedData = yaml.load(data);
    return yaml.dump(parsedData, { indent: 2 });
  } catch (error) {
    console.error('Error formatting YAML:', error);
    return data;
  }
}

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
    editedPipeline.value.data = yamlData.value;
    emit('save', editedPipeline.value);
  }
};
</script>

<style scoped>
.pipeline-editor {
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
.error {
  color: red;
  margin-bottom: 10px;
}
</style>