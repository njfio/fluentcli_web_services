<template>
  <div class="configuration-editor">
    <h3>{{ isNew ? 'Create' : 'Edit' }} Configuration</h3>
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="name">Name:</label>
        <input type="text" id="name" v-model="editedConfiguration.name" required>
      </div>
      <div class="form-group">
        <label for="description">Description:</label>
        <textarea id="description" v-model="editedConfiguration.description" rows="3" required></textarea>
      </div>
      <div class="form-group">
        <label for="type">Type:</label>
        <input type="text" id="type" v-model="editedConfiguration.type" required>
      </div>
      <div class="form-group">
        <label for="status">Status:</label>
        <select id="status" v-model="editedConfiguration.status" required>
          <option value="active">Active</option>
          <option value="inactive">Inactive</option>
          <option value="draft">Draft</option>
        </select>
      </div>
      <div class="form-actions">
        <button type="submit" class="save-button">Save</button>
        <button type="button" @click="handleCancel" class="cancel-button">Cancel</button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useStore } from 'vuex';
import { StudioConfiguration } from '@/store/modules/studio';

export default defineComponent({
  name: 'ConfigurationEditor',
  setup() {
    const route = useRoute();
    const router = useRouter();
    const store = useStore();

    const defaultConfiguration: StudioConfiguration = {
      id: '',
      name: '',
      description: '',
      type: '',
      status: 'draft',
      createdAt: '',
      updatedAt: '',
    };

    const editedConfiguration = ref<StudioConfiguration>({ ...defaultConfiguration });

    const isNew = computed(() => !editedConfiguration.value.id);

    const fetchConfiguration = async (id: string) => {
      try {
        const config = await store.dispatch('studio/fetchConfigurationById', id);
        editedConfiguration.value = { ...defaultConfiguration, ...config };
      } catch (error) {
        console.error('Error fetching configuration:', error);
        alert('Failed to fetch configuration. Please try again.');
      }
    };

    onMounted(() => {
      const id = route.params.id as string;
      if (id) {
        fetchConfiguration(id);
      }
    });

    const handleSubmit = async () => {
      try {
        await store.dispatch('studio/saveConfiguration', editedConfiguration.value);
        router.push({ name: 'Configurations' });
      } catch (error) {
        console.error('Error saving configuration:', error);
        alert('Failed to save configuration. Please try again.');
      }
    };

    const handleCancel = () => {
      router.push({ name: 'Configurations' });
    };

    return {
      editedConfiguration,
      isNew,
      handleSubmit,
      handleCancel,
    };
  },
});
</script>

<style scoped>
.configuration-editor {
  background-color: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.configuration-editor h3 {
  margin-top: 0;
  margin-bottom: 20px;
  font-size: 1.5rem;
}

.form-group {
  margin-bottom: 20px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input[type="text"], textarea, select {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 3px;
  font-size: 1rem;
}

textarea {
  resize: vertical;
  min-height: 100px;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
}

.save-button, .cancel-button {
  padding: 10px 20px;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.3s ease;
}

.save-button {
  background-color: #2ecc71;
  color: #fff;
  margin-right: 10px;
}

.save-button:hover {
  background-color: #27ae60;
}

.cancel-button {
  background-color: #e74c3c;
  color: #fff;
}

.cancel-button:hover {
  background-color: #c0392b;
}
</style>