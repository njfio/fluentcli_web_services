<template>
  <div class="job-editor">
    <h3>{{ isNew ? 'Create' : 'Edit' }} Job</h3>
    <form @submit.prevent="handleSubmit">
      <div>
        <label for="uri">URI:</label>
        <input type="text" id="uri" v-model="editedJob.uri" required>
      </div>
      <div>
        <label for="worker_type">Worker Type:</label>
        <select id="worker_type" v-model="editedJob.worker_type" required>
          <option v-for="dockerFile in dockerFiles" :key="dockerFile.id" :value="dockerFile.id">
            {{ dockerFile.name }}
          </option>
        </select>
        <router-link to="/studio/dockerfiles">Manage Docker Files</router-link>
      </div>
      <div>
        <label for="config">Configuration:</label>
        <select id="config" v-model="editedJob.config" required>
          <option v-for="config in configurations" :key="config.id" :value="config.id">
            {{ config.name }}
          </option>
        </select>
        <router-link to="/studio/configurations">Manage Configurations</router-link>
      </div>
      <div>
        <label for="pipeline">Pipeline:</label>
        <select id="pipeline" v-model="editedJob.data_path" required>
          <option v-for="pipeline in pipelines" :key="pipeline.id" :value="pipeline.id">
            {{ pipeline.name }}
          </option>
        </select>
        <router-link to="/studio/pipelines">Manage Pipelines</router-link>
      </div>
      <div>
        <label for="amber_store">Amber Store:</label>
        <select id="amber_store" v-model="editedJob.amber_id">
          <option :value="null">None</option>
          <option v-for="amberStore in amberStores" :key="amberStore.id" :value="amberStore.id">
            {{ amberStore.id }}
          </option>
        </select>
        <router-link to="/studio/amberstores">Manage Amber Stores</router-link>
      </div>
      <div>
        <button type="submit">Save</button>
        <button type="button" @click="$emit('cancel')">Cancel</button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';

interface Job {
  id?: string;
  uri: string;
  worker_type: string;
  config: string;
  data_path: string;
  amber_id: string | null;
  status: string;
}

const props = defineProps<{
  data: Job;
  dockerFiles: any[];
  configurations: any[];
  pipelines: any[];
  amberStores: any[];
}>();

const emit = defineEmits<{
  (e: 'save', job: Job): void;
  (e: 'cancel'): void;
}>();

const editedJob = ref<Job>({ ...props.data });

const isNew = computed(() => !props.data.id);

watch(() => props.data, (newData) => {
  editedJob.value = { ...newData };
}, { deep: true });

const handleSubmit = () => {
  emit('save', editedJob.value);
};
</script>

<style scoped>
/* ... (keep existing styles) ... */
</style>