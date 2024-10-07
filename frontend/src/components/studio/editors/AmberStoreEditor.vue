<template>
  <div class="amber-store-editor">
    <h2 class="text-2xl font-bold mb-4">{{ isEditing ? 'Edit' : 'Create' }} Amber Store</h2>
    <form @submit.prevent="saveAmberStore">
      <div class="mb-4">
        <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
        <input
          type="text"
          id="name"
          v-model="amberStore.name"
          required
          class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        />
      </div>
      <div class="mb-4">
        <label for="description" class="block text-sm font-medium text-gray-700">Description</label>
        <textarea
          id="description"
          v-model="amberStore.description"
          rows="3"
          class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        ></textarea>
      </div>
      <div class="mb-4">
        <label for="data" class="block text-sm font-medium text-gray-700">Data</label>
        <textarea
          id="data"
          v-model="amberStore.data"
          rows="10"
          class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        ></textarea>
      </div>
      <div class="flex justify-end">
        <button
          type="button"
          @click="cancel"
          class="mr-2 px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          Cancel
        </button>
        <button
          type="submit"
          class="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          {{ isEditing ? 'Update' : 'Create' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRouter, useRoute } from 'vue-router';
import { AmberStore } from '@/store/modules/studio';

export default defineComponent({
  name: 'AmberStoreEditor',
  setup() {
    const store = useStore();
    const router = useRouter();
    const route = useRoute();

    const amberStore = ref<AmberStore>({
      id: '',
      name: '',
      description: '',
      data: '',
      secure_key_hash: '',
      createdAt: '',
      lastModified: '',
    });

    const isEditing = computed(() => !!route.params.id);

    onMounted(async () => {
      if (isEditing.value) {
        const id = route.params.id as string;
        try {
          const fetchedStore = await store.dispatch('studio/fetchAmberStoreById', id);
          amberStore.value = { ...fetchedStore };
        } catch (error) {
          console.error('Error fetching Amber Store:', error);
          // Handle error (e.g., show error message to user)
        }
      }
    });

    const saveAmberStore = async () => {
      try {
        await store.dispatch('studio/saveAmberStore', amberStore.value);
        router.push({ name: 'AmberStores' });
      } catch (error) {
        console.error('Error saving Amber Store:', error);
        // Handle error (e.g., show error message to user)
      }
    };

    const cancel = () => {
      router.push({ name: 'AmberStores' });
    };

    return {
      amberStore,
      isEditing,
      saveAmberStore,
      cancel,
    };
  },
});
</script>

<style scoped>
.amber-store-editor {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}
</style>