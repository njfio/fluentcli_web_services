<script setup lang="ts">

</script>

<template>
  <div class="container">
    <router-view></router-view>
    <nav>
      <router-link to="/">Home</router-link> |
      <router-link to="/admin">Admin</router-link> |
      <router-link to="/studio">Studio</router-link>
    </nav>


  </div>
</template>