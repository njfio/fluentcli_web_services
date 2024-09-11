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
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  
  interface Pipeline {
    name: string;
    stages: Array<{ name: string; command: string }>;
  }
  
  export default defineComponent({
    name: 'PipelineEditor',
    props: {
      data: {
        type: Object as () => Pipeline,
        required: true,
      },
    },
    setup(props) {
      const pipeline = ref<Pipeline>({ 
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
        // Implement save logic
        console.log('Saving pipeline:', pipeline.value);
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