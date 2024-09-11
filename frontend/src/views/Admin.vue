<template>
    <div>
      <h1>Admin</h1>
      <nav>
        <RouterLink to="/admin/jobs">Job List</RouterLink> |
        <RouterLink to="/admin/jobs/create">Create Job</RouterLink> 
      </nav>
      <RouterView />
    </div>
  </template>
  
  <script setup lang="ts">
    import { RouterLink, RouterView } from 'vue-router';
  </script>
  
  <style scoped>
    /* Add your styles here */
  </style>