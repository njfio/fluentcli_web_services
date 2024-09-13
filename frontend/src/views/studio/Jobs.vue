<template>
  <div class="jobs">
    <h1>Manage Jobs</h1>
    <router-link to="/studio/jobs/create" class="create-button">Create New Job</router-link>
    <JobList />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import JobList from '@/components/studio/JobList.vue';

export default defineComponent({
  name: 'Jobs',
  components: {
    JobList,
  },
});
</script>

<style scoped>
.jobs {
  padding: 20px;
}

.create-button {
  background-color: #27ae60;
  color: #fff;
  padding: 8px 12px;
  border-radius: 5px;
  text-decoration: none;
  margin-bottom: 15px;
  display: inline-block;
}
</style>