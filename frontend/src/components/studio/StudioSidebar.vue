<template>
  <div :class="['sidebar', { collapsed: isCollapsed }]">
    <div class="toggle-button" @click="$emit('toggle')">
      <span v-if="!isCollapsed">«</span>
      <span v-else>»</span>
    </div>
    <nav>
      <ul>
        <li>
          <router-link to="/studio/dashboard">Dashboard</router-link>
        </li>
        <li>
          <router-link to="/studio/jobs">Jobs</router-link>
        </li>
        <li>
          <router-link to="/studio/pipelines">Pipelines</router-link>
        </li>
        <li>
          <router-link to="/studio/dockerfiles">Docker Files</router-link>
        </li>
        <li>
          <router-link to="/studio/configurations">Configurations</router-link>
        </li>
        <li>
          <router-link to="/studio/amberstores">Amber Stores</router-link>
        </li>
        <li>
          <router-link to="/studio/settings">Settings</router-link>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'StudioSidebar',
  props: {
    isCollapsed: {
      type: Boolean,
      required: true,
    },
  },
});
</script>

<style scoped>
.sidebar {
  width: 250px;
  background-color: #2c3e50;
  color: #ecf0f1;
  transition: width 0.3s;
  position: relative;
}

.sidebar.collapsed {
  width: 80px;
}

.toggle-button {
  position: absolute;
  top: 10px;
  right: -15px;
  background-color: #34495e;
  border-radius: 50%;
  padding: 5px;
  cursor: pointer;
}

nav ul {
  list-style: none;
  padding: 0;
  margin-top: 50px;
}

nav ul li {
  padding: 15px 20px;
}

nav ul li a {
  color: #ecf0f1;
  text-decoration: none;
  display: block;
}

nav ul li a.router-link-exact-active {
  background-color: #1abc9c;
}
</style>