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
          <div class="amber-stores-dropdown">
            <router-link to="/studio/amberstores">Amber Stores</router-link>
            <ul v-if="!isCollapsed" class="amber-stores-list">
              <li v-for="store in amberStores" :key="store.id">
                <a @click="selectAmberStore(store.id)">{{ store.name }}</a>
              </li>
            </ul>
          </div>
        </li>
        <li>
          <router-link to="/studio/settings">Settings</router-link>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';

export default defineComponent({
  name: 'StudioSidebar',
  props: {
    isCollapsed: {
      type: Boolean,
      required: true,
    },
  },
  setup() {
    const store = useStore();
    const router = useRouter();

    const amberStores = computed(() => store.getters['studio/getAmberStores']);

    onMounted(async () => {
      await store.dispatch('studio/fetchAmberStores');
    });

    const selectAmberStore = (id: string) => {
      router.push({ name: 'AmberStores', params: { id } });
    };

    return {
      amberStores,
      selectAmberStore,
    };
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

.amber-stores-dropdown {
  position: relative;
}

.amber-stores-list {
  display: none;
  position: absolute;
  top: 100%;
  left: 0;
  background-color: #34495e;
  padding: 10px;
  border-radius: 5px;
  z-index: 10;
}

.amber-stores-dropdown:hover .amber-stores-list {
  display: block;
}

.amber-stores-list li {
  padding: 5px 0;
}

.amber-stores-list a {
  cursor: pointer;
}

.amber-stores-list a:hover {
  color: #1abc9c;
}
</style>