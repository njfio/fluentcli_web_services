<template>
  <div class="studio">
    <nav class="studio-sidebar" :class="{ 'collapsed': isSidebarCollapsed }">
      <div class="sidebar-header">
        <button @click="toggleSidebar" class="toggle-sidebar">
          <i class="fas fa-bars"></i>
        </button>
        <h1>FluentCLI Studio</h1>
      </div>
      <router-link to="/studio/dashboard">
        <i class="fas fa-tachometer-alt"></i>
        <span>Dashboard</span>
      </router-link>
      <router-link to="/studio/jobs">
        <i class="fas fa-tasks"></i>
        <span>Jobs</span>
      </router-link>
      <router-link to="/studio/pipelines">
        <i class="fas fa-project-diagram"></i>
        <span>Pipelines</span>
      </router-link>
      <router-link to="/studio/dockerfiles">
        <i class="fab fa-docker"></i>
        <span>Docker Files</span>
      </router-link>
      <router-link to="/studio/configurations">
        <i class="fas fa-cogs"></i>
        <span>Configurations</span>
      </router-link>
      <router-link to="/studio/amberstores">
        <i class="fas fa-database"></i>
        <span>Amber Stores</span>
      </router-link>
    </nav>
    <main class="studio-main">
      <StudioHeader @toggleSidebar="toggleSidebar" />
      <div class="content-area">
        <router-view />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute } from 'vue-router';
import StudioHeader from '@/components/studio/StudioHeader.vue';

const isSidebarCollapsed = ref(false);
const route = useRoute();

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value;
};

computed(() => {
  const routeName = route.name as string;
  return routeName.charAt(0).toUpperCase() + routeName.slice(1);
});
</script>

<style scoped>
.studio {
  display: flex;
  height: 100vh;
  font-family: 'Arial', sans-serif;
}

.studio-sidebar {
  width: 250px;
  background-color: #2c3e50;
  color: #ecf0f1;
  transition: width 0.3s ease;
}

.studio-sidebar.collapsed {
  width: 60px;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
}

.sidebar-header h1 {
  font-size: 1.2rem;
  margin: 0;
}

.toggle-sidebar {
  background: none;
  border: none;
  color: #ecf0f1;
  cursor: pointer;
  font-size: 1.2rem;
}

.studio-sidebar a {
  display: flex;
  align-items: center;
  color: #ecf0f1;
  text-decoration: none;
  padding: 15px 20px;
  transition: background-color 0.3s ease;
}

.studio-sidebar a:hover {
  background-color: #34495e;
}

.studio-sidebar a.router-link-active {
  background-color: #3498db;
}

.studio-sidebar i {
  margin-right: 10px;
  width: 20px;
  text-align: center;
}

.studio-main {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  background-color: #f5f5f5;
}

.studio-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.studio-header h2 {
  margin: 0;
  font-size: 1.5rem;
}

.user-menu {
  display: flex;
  align-items: center;
}

.user-menu span {
  margin-right: 10px;
}

.logout-button {
  background-color: #e74c3c;
  color: #fff;
  border: none;
  padding: 5px 10px;
  border-radius: 3px;
  cursor: pointer;
}

.content-area {
  flex-grow: 1;
  padding: 20px;
  overflow-y: auto;
}
</style>