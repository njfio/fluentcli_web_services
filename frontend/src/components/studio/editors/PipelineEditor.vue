<template>
    <div class="pipeline-editor">
      <h2>Pipeline Editor</h2>
      <form @submit.prevent="savePipeline">
        <div class="form-group">
          <label for="name">Pipeline Name</label>
          <input
            type="text"
            id="name"
            v-model="pipeline.name"
            required
            class="form-control"
          />
        </div>
        <div class="form-group">
          <label for="editor">Pipeline Configuration</label>
          <EditorContent :editor="editor" />
        </div>
        <div class="button-group">
          <button type="submit" class="btn btn-primary">Save Pipeline</button>
          <button type="button" @click="cancel" class="btn btn-secondary">Cancel</button>
        </div>
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, watch } from 'vue';
  import { useEditor, EditorContent } from '@tiptap/vue-3';
  import StarterKit from '@tiptap/starter-kit';
  import { DataPill } from '@/extensions/DataPill';
  
  interface Pipeline {
    id?: string;
    name: string;
    data: any;
  }
  
  export default defineComponent({
    name: 'PipelineEditor',
    components: { EditorContent },
    props: {
      data: {
        type: Object as () => Pipeline,
        required: true,
      },
    },
    setup(props, { emit }) {
      const pipeline = ref<Pipeline>({ ...props.data });
      const isSaving = ref(false);
      const errorMessage = ref('');
  
      const editor = useEditor({
        extensions: [StarterKit, DataPill],
        content: pipeline.value.data || {
          type: 'doc',
          content: [],
        },
      });
  
      watch(
        () => props.data,
        (newValue) => {
          pipeline.value = { ...newValue };
          if (editor.value) {
            editor.value.commands.setContent(newValue.data || { type: 'doc', content: [] });
          }
        },
        { deep: true }
      );
  
      const savePipeline = () => {
        if (editor.value) {
          const updatedPipeline: Pipeline = {
            id: pipeline.value.id, // 'id' may be undefined for new pipelines
            name: pipeline.value.name,
            data: editor.value.getJSON(), // Use getJSON for structured data
          };
          emit('save', updatedPipeline);
        }
      };
  
      const cancel = () => {
        emit('cancel');
      };
  
      return {
        pipeline,
        isSaving,
        errorMessage,
        savePipeline,
        cancel,
        editor,
      };
    },
  });
  </script>
  
  <style scoped>
  .pipeline-editor {
    padding: 20px;
  }
  .form-group {
    margin-bottom: 15px;
  }
  .button-group {
    margin-top: 20px;
  }
  </style>