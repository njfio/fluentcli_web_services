<template>
  <div class="pipeline-editor bg-white rounded-lg shadow-md p-6 w-full h-full flex flex-col">
    <h3 class="text-2xl font-bold mb-6">{{ isNew ? 'Create' : 'Edit' }} Pipeline</h3>
    <form @submit.prevent="handleSubmit" class="flex flex-col flex-grow">
      <div class="mb-4">
        <label for="name" class="block text-sm font-medium text-gray-700">Name:</label>
        <input id="name" v-model="editedPipeline.name" required
               class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
      <div class="flex-grow flex flex-col">
        <label for="data" class="block text-sm font-medium text-gray-700 mb-2">Data (YAML):</label>
        <div class="relative flex-grow">
          <textarea id="data" v-model="yamlData" required @input="handleInput"
                    class="absolute inset-0 w-full h-full resize-none opacity-0"></textarea>
          <pre class="absolute inset-0 w-full h-full overflow-auto"><code class="language-yaml" v-html="highlightedYaml"></code></pre>
        </div>
      </div>
      <div v-if="yamlError" class="text-red-600 text-sm mt-2">
        {{ yamlError }}
      </div>
      <div class="flex justify-end space-x-4 mt-4">
        <button type="button" @click="$emit('cancel')" 
                class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
          Cancel
        </button>
        <button type="submit" :disabled="!!yamlError"
                class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:bg-gray-400 disabled:cursor-not-allowed">
          Save
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import * as yaml from 'js-yaml';
import hljs from 'highlight.js/lib/core';
import yamlLang from 'highlight.js/lib/languages/yaml';
import 'highlight.js/styles/github.css';

hljs.registerLanguage('yaml', yamlLang);

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
const highlightedYaml = ref('');

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  console.log('New data received:', newData);
  editedPipeline.value = { ...newData };
  yamlData.value = formatYaml(newData.data);
  updateHighlightedYaml();
}, { deep: true });

function updateHighlightedYaml() {
  highlightedYaml.value = hljs.highlight(yamlData.value, { language: 'yaml' }).value;
}

function handleInput(event: Event) {
  const target = event.target as HTMLTextAreaElement;
  yamlData.value = target.value;
  validateYaml();
  updateHighlightedYaml();
}

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
  editedPipeline.value.data = yamlData.value; // Save as text
  console.log('Emitting save with:', editedPipeline.value);
  emit('save', editedPipeline.value);
};

onMounted(() => {
  updateHighlightedYaml();
});
</script>

