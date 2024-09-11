<template>
    <div class="dockerfile-editor">
      <h2>Dockerfile Editor</h2>
      <form @submit.prevent="saveDockerfile">
        <div>
          <label for="name">Name:</label>
          <input type="text" id="name" v-model="dockerfile.name" required />
        </div>
        <div>
          <label for="content">Content:</label>
          <textarea id="content" v-model="dockerfile.content" rows="20" required></textarea>
        </div>
        <button type="submit">Save Dockerfile</button>
      </form>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  
  export default defineComponent({
    name: 'DockerFileEditor',
    props: {
      data: {
        type: Object,
        required: true,
      },
    },
    setup(props) {
      const dockerfile = ref({ ...props.data });
  
      const saveDockerfile = () => {
        // Implement save logic
        console.log('Saving Dockerfile:', dockerfile.value);
      };
  
      return {
        dockerfile,
        saveDockerfile,
      };
    },
  });
  </script>