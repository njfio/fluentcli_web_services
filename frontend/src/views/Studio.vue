<template>
    <div class="studio">
      <h1>Studio</h1>
      <p>This is the studio page. Only authenticated users can access this page.</p>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({
    name: 'Studio'
  });
  </script>