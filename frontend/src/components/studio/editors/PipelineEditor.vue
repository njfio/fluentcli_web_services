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

// Create a custom type to handle all tags
const customTags = ['!Command', '!ShellCommand', '!Condition', '!Loop', '!Map', '!SubPipeline', '!HumanInTheLoop', '!RepeatUntil', '!PrintOutput', '!ForEach', '!TryCatch', '!Parallel', '!Timeout'];

const CUSTOM_SCHEMA = new yaml.Schema({
  include: [yaml.DEFAULT_SCHEMA],
  explicit: customTags.map(tag => new yaml.Type(tag, {
    kind: 'mapping',
  }))
} as yaml.SchemaDefinition);


function customStringify(obj: any): string {
  if (typeof obj === 'object' && obj !== null) {
    const key = Object.keys(obj)[0];
    if (customTags.includes(`!${key}`)) {
      return `!${key}\n${yaml.dump(obj[key], { schema: CUSTOM_SCHEMA, indent: 2 }).trim()}`;
    }
  }
  return yaml.dump(obj, { schema: CUSTOM_SCHEMA, indent: 2 }).trim();
}

function formatYaml(data: string): string {
  try {
    const parsedData = yaml.load(data, { schema: CUSTOM_SCHEMA }) as any;
    return customStringify(parsedData);
  } catch (error) {
    console.error('Error formatting YAML:', error);
    return data;
  }
}

const editedPipeline = ref<Pipeline>({ ...props.data });
const yamlData = ref(formatYaml(props.data.data));
const yamlError = ref('');

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  console.log('New data received:', newData);
  editedPipeline.value = { ...newData };
  yamlData.value = formatYaml(newData.data);
}, { deep: true });

function validateYaml() {
  try {
    yaml.load(yamlData.value, { 
      schema: CUSTOM_SCHEMA,
    });
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
  console.log('Handling submit');
  if (!yamlError.value) {
    editedPipeline.value.data = yamlData.value;
    const parsedData = yaml.load(yamlData.value, { schema: CUSTOM_SCHEMA });
    editedPipeline.value.data = JSON.stringify(parsedData);
    console.log('Emitting save with:', editedPipeline.value);
    emit('save', editedPipeline.value);
  } else {
    console.error('Cannot submit due to YAML error:', yamlError.value);
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