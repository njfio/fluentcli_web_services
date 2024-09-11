<template>
    <div class="configuration-editor">
      <h2>Configuration Editor</h2>
      <form @submit.prevent="saveConfiguration">
        <div>
          <label for="name">Name:</label>
          <input type="text" id="name" v-model="config.name" required />
        </div>
        <div>
          <label for="data">Data:</label>
          <textarea id="data" v-model="config.data" rows="10" required></textarea>
        </div>
        <button type="submit">Save Configuration</button>
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  
  export default defineComponent({
    name: 'ConfigurationEditor',
    props: {
      data: {
        type: Object,
        required: true,
      },
    },
    setup(props) {
      const config = ref({ ...props.data });
  
      const saveConfiguration = () => {
        // Implement save logic
        console.log('Saving configuration:', config.value);
      };
  
      return {
        config,
        saveConfiguration,
      };
    },
  });
  </script>