<template>
  <div class="configuration-editor">
    <h2 class="text-2xl font-bold mb-4">{{ isNewConfiguration ? 'Create New Configuration' : 'Edit Configuration' }}</h2>
    <form @submit.prevent="saveConfiguration" class="space-y-4">
      <div>
        <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
        <input type="text" id="name" v-model="configuration.name" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
      </div>
      <div>
        <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
        <textarea id="description" v-model="configuration.description" rows="3" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"></textarea>
      </div>
      <div>
        <label for="type" class="block text-sm font-medium text-gray-700">Type</label>
        <input type="text" id="type" v-model="configuration.type" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
      </div>
      <div>
        <label for="status" class="block text-sm font-medium text-gray-700">Status</label>
        <input type="text" id="status" v-model="configuration.status" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
      </div>
      <div class="flex justify-end space-x-2">
        <button type="button" @click="cancel" class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50">
          Cancel
        </button>
        <button type="submit" class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700">
          {{ isNewConfiguration ? 'Create' : 'Save' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRoute, useRouter } from 'vue-router';

export default defineComponent({
  name: 'ConfigurationEditor',
  setup() {
    const store = useStore();
    const route = useRoute();
    const router = useRouter();

    const configuration = ref({
      id: '',
      name: '',
      description: '',
      type: '',
      status: '',
    });

    const isNewConfiguration = computed(() => !route.params.id);

    onMounted(async () => {
      if (!isNewConfiguration.value) {
        const configId = route.params.id as string;
        const config = await store.dispatch('studio/fetchConfigurationById', configId);
        if (config) {
          configuration.value = { ...config };
        }
      }
    });

    const saveConfiguration = async () => {
      try {
        if (isNewConfiguration.value) {
          await store.dispatch('studio/createConfiguration', configuration.value);
        } else {
          await store.dispatch('studio/updateConfiguration', configuration.value);
        }
        navigateBack();
      } catch (error) {
        console.error('Error saving configuration:', error);
        // Handle error (e.g., show error message to user)
      }
    };

    const cancel = () => {
      navigateBack();
    };

    const navigateBack = () => {
      const returnToJobDetails = route.query.returnToJobDetails;
      if (returnToJobDetails) {
        router.push({ name: 'JobDetail', params: { id: returnToJobDetails as string } });
      } else {
        router.push({ name: 'Configurations' });
      }
    };

    return {
      configuration,
      isNewConfiguration,
      saveConfiguration,
      cancel,
    };
  },
});
</script>