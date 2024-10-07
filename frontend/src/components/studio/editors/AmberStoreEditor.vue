<template>
  <div class="amber-store-editor">
    <h2 class="text-2xl font-bold mb-4">{{ isNewAmberStore ? 'Create New Amber Store' : 'Edit Amber Store' }}</h2>
    <form @submit.prevent="saveAmberStore" class="space-y-4">
      <div>
        <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
        <input type="text" id="name" v-model="amberStore.name" required class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
      </div>
      <div>
        <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
        <textarea id="description" v-model="amberStore.description" rows="3" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"></textarea>
      </div>
      <div>
        <label for="data" class="block text-sm font-medium text-gray-700">Data</label>
        <textarea id="data" v-model="amberStore.data" rows="10" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2 font-mono"></textarea>
      </div>
      <div>
        <label for="secure_key_hash" class="block text-sm font-medium text-gray-700">Secure Key Hash</label>
        <input type="text" id="secure_key_hash" v-model="amberStore.secure_key_hash" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
      </div>
      <div class="flex justify-end space-x-2">
        <button type="button" @click="cancel" class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50">
          Cancel
        </button>
        <button type="submit" class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700">
          {{ isNewAmberStore ? 'Create' : 'Save' }}
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
  name: 'AmberStoreEditor',
  setup() {
    const store = useStore();
    const route = useRoute();
    const router = useRouter();

    const amberStore = ref({
      id: '',
      name: '',
      description: '',
      data: '',
      secure_key_hash: '',
    });

    const isNewAmberStore = computed(() => !route.params.id);

    onMounted(async () => {
      if (!isNewAmberStore.value) {
        const amberStoreId = route.params.id as string;
        const fetchedAmberStore = await store.dispatch('studio/fetchAmberStoreById', amberStoreId);
        if (fetchedAmberStore) {
          amberStore.value = { ...fetchedAmberStore };
        }
      }
    });

    const saveAmberStore = async () => {
      try {
        if (isNewAmberStore.value) {
          await store.dispatch('studio/createAmberStore', amberStore.value);
        } else {
          await store.dispatch('studio/updateAmberStore', amberStore.value);
        }
        navigateBack();
      } catch (error) {
        console.error('Error saving Amber Store:', error);
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
        router.push({ name: 'AmberStores' });
      }
    };

    return {
      amberStore,
      isNewAmberStore,
      saveAmberStore,
      cancel,
    };
  },
});
</script>