<template>
    <header class="studio-header">
      <button class="menu-button" @click="$emit('toggleSidebar')">☰</button>
      <div class="user-info">
        <span>Welcome, {{ userName }}</span>
        <button @click="logout">Logout</button>
      </div>
    </header>
  </template>
  
<script setup lang="ts">
import { computed } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import AuthService from '@/services/AuthService';

const store = useStore();
const router = useRouter();

const userName = computed(() => {
  const user = store.state.user;
  return user ? user.name || user.username : 'Guest';
});

const logout = () => {
  AuthService.logout();
  store.commit('setLoggedIn', false);
  store.commit('setUser', null);
  router.push('/');
};
</script>
  
  <style scoped>
  .studio-header {
    height: 60px;
    background-color: #ecf0f1;
    display: flex;
    align-items: center;
    padding: 0 20px;
    justify-content: space-between;
    border-bottom: 1px solid #bdc3c7;
  }
  
  .menu-button {
    font-size: 24px;
    background: none;
    border: none;
    cursor: pointer;
  }
  
  .user-info {
    display: flex;
    align-items: center;
  }
  
  .user-info span {
    margin-right: 15px;
  }
  
  .user-info button {
    padding: 5px 10px;
    cursor: pointer;
  }
  </style>