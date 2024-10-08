<template>
  <div ref="editorContainer" class="yaml-editor"></div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, onMounted, onBeforeUnmount } from 'vue';
import * as monaco from 'monaco-editor';

export default defineComponent({
  name: 'YamlEditor',
  props: {
    modelValue: {
      type: String,
      default: '',
    },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    const editorContainer = ref<HTMLElement | null>(null);
    let editor: monaco.editor.IStandaloneCodeEditor | null = null;

    const initMonaco = () => {
      if (editorContainer.value) {
        editor = monaco.editor.create(editorContainer.value, {
          value: props.modelValue,
          language: 'yaml',
          theme: 'vs-light',
          automaticLayout: true,
          minimap: { enabled: false },
          scrollBeyondLastLine: false,
        });

        editor.onDidChangeModelContent(() => {
          emit('update:modelValue', editor?.getValue());
        });
      }
    };

    onMounted(() => {
      initMonaco();
    });

    onBeforeUnmount(() => {
      if (editor) {
        editor.dispose();
      }
    });

    watch(() => props.modelValue, (newValue) => {
      if (editor && newValue !== editor.getValue()) {
        editor.setValue(newValue);
      }
    });

    return {
      editorContainer,
    };
  },
});
</script>

<style scoped>
.yaml-editor {
  width: 100%;
  height: 100%;
}
</style>