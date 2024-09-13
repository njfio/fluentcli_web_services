<template>
    <div class="pipeline-editor">
      <h2>Pipeline Editor</h2>
      <form @submit.prevent="savePipeline">
        <div>
          <label for="name">Name:</label>
          <input type="text" id="name" v-model="pipeline.name" required />
        </div>
        <div>
          <label for="stages">Stages:</label>
          <div v-for="(stage, index) in pipeline.stages" :key="index">
            <input type="text" v-model="stage.name" placeholder="Stage name" />
            <textarea v-model="stage.command" placeholder="Stage command"></textarea>
            <button type="button" @click="removeStage(index)">Remove Stage</button>
          </div>
          <button type="button" @click="addStage">Add Stage</button>
        </div>
        <button type="submit">Save Pipeline</button>
        <button type="button" @click="$emit('cancel')">Cancel</button>
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  
  interface Stage {
    name: string;
    command: string;
  }
  
  interface Pipeline {
    id?: string; // Optional for new pipelines
    name: string;
    stages: Stage[];
  }
  
  export default defineComponent({
    name: 'PipelineEditor',
    props: {
      data: {
        type: Object as () => Pipeline,
        required: true,
      },
    },
    setup(props, { emit }) {
      const pipeline = ref<Pipeline>({ 
        id: props.data.id,
        name: props.data.name || '',
        stages: props.data.stages || []
      });
  
      const addStage = () => {
        pipeline.value.stages.push({ name: '', command: '' });
      };
  
      const removeStage = (index: number) => {
        pipeline.value.stages.splice(index, 1);
      };
  
      const savePipeline = () => {
        emit('save', pipeline.value);
      };
  
      return {
        pipeline,
        addStage,
        removeStage,
        savePipeline,
      };
    },
  });
  </script>
  
  <style scoped>
  .pipeline-editor {
    padding: 20px;
  }
  
  .pipeline-editor form {
    display: flex;
    flex-direction: column;
  }
  
  .pipeline-editor label {
    margin-top: 10px;
  }
  
  .pipeline-editor input,
  .pipeline-editor textarea {
    padding: 8px;
    margin-top: 5px;
    border: 1px solid #bdc3c7;
    border-radius: 3px;
  }
  
  .pipeline-editor button {
    margin-top: 15px;
    padding: 10px;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }
  
  .pipeline-editor button[type="submit"] {
    background-color: #27ae60;
    color: #fff;
  }
  
  .pipeline-editor button[type="button"] {
    background-color: #c0392b;
    color: #fff;
    margin-left: 10px;
  }
  </style>