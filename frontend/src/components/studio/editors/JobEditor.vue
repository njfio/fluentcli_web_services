<template>
  <div class="job-editor">
    <h3>{{ isNew ? 'Create' : 'Edit' }} Job</h3>
    <form @submit.prevent="handleSubmit">
      <div>
        <label for="worker_type">Worker Type:</label>
        <select id="worker_type" v-model="editedJob.worker_type" required>
          <option v-for="dockerFile in dockerFiles" :key="dockerFile.id" :value="dockerFile.id">
            {{ dockerFile.name }}
          </option>
        </select>
      </div>
      <div>
        <label for="config">Configuration:</label>
        <select id="config" v-model="editedJob.config" required>
          <option v-for="config in configurations" :key="config.id" :value="config.id">
            {{ config.name }}
          </option>
        </select>
      </div>
      <div>
        <label for="pipeline">Pipeline:</label>
        <select id="pipeline" v-model="editedJob.pipeline_id" required>
          <option v-for="pipeline in pipelines" :key="pipeline.id" :value="pipeline.id">
            {{ pipeline.name }}
          </option>
        </select>
      </div>
      <div>
        <label for="amber_store">Amber Store:</label>
        <select id="amber_store" v-model="editedJob.amber_id">
          <option :value="null">None</option>
          <option v-for="amberStore in amberStores" :key="amberStore.id" :value="amberStore.id">
            {{ amberStore.name }}
          </option>
        </select>
      </div>
      <div>
        <label for="state_file_content">State File Content:</label>
        <textarea id="state_file_content" v-model="editedJob.state_file_content"></textarea>
      </div>
      <div>
        <label for="status">Status:</label>
        <input id="status" v-model="editedJob.status" required />
      </div>
      <div>
        <button type="submit" class="save-button">Save</button>
        <button type="button" @click="$emit('cancel')" class="cancel-button">Cancel</button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

interface Job {
  id?: string;
  config: string;
  amber_id?: string | null;
  state_file_content?: string;
  worker_type: string;
  status: string;
  pipeline_id: string;
}

const props = defineProps<{
  job: Job | null;
  dockerFiles: { id: string; name: string }[];
  configurations: { id: string; name: string }[];
  pipelines: { id: string; name: string }[];
  amberStores: { id: string; name: string }[];
}>();

const emit = defineEmits<{
  (e: 'save', job: Job): void;
  (e: 'cancel'): void;
}>();

const editedJob = ref<Job>(props.job ? { ...props.job } : {
  config: '',
  worker_type: '',
  status: '',
  pipeline_id: '',
} as Job);

const isNew = computed(() => !props.job?.id);

const handleSubmit = () => {
  emit('save', editedJob.value);
};
</script>