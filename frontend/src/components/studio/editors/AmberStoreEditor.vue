<template>
  <div class="amber-store-editor">
    <h1 class="text-2xl font-bold mb-6">{{ isNewAmberStore ? 'Create New Amber Store' : 'Edit Amber Store' }}</h1>
    <div class="flex justify-end mb-6">
      <button type="button" @click="cancel"
        class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
        Cancel
      </button>
      <button type="submit" form="amber-store-form"
        class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
        {{ isNewAmberStore ? 'Create' : 'Update' }}
      </button>
    </div>
    <form id="amber-store-form" @submit.prevent="saveAmberStore" class="space-y-6">
      <div class="bg-white shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
              <input type="text" id="name" v-model="amberStore.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
            </div>
            <div class="sm:col-span-4">
              <label for="secure_key_hash" class="block text-sm font-medium text-gray-700">Secure Key Hash</label>
              <input type="text" id="secure_key_hash" v-model="amberStore.secure_key_hash" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md">
            </div>
            <div class="sm:col-span-6">
              <label for="data" class="block text-sm font-medium text-gray-700">Data (YAML)</label>
              <div class="mt-1 border border-gray-300 rounded-md overflow-hidden" style="height: calc(100vh - 400px);">
                <YamlEditor v-model="amberStore.data" class="h-full" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRouter, useRoute } from 'vue-router';
import YamlEditor from './MonacoEditor.vue';

export default defineComponent({
  name: 'AmberStoreEditor',
  components: {
    YamlEditor,
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const route = useRoute();

    const amberStore = ref({
      id: '',
      name: '',
      data: '',
      secure_key_hash: '',
    });

    const isNewAmberStore = computed(() => route.params.id === 'new');

    onMounted(async () => {
      if (!isNewAmberStore.value) {
        const id = route.params.id as string;
        const fetchedAmberStore = await store.dispatch('studio/fetchAmberStoreById', id);
        amberStore.value = { ...fetchedAmberStore };
      }
    });

    const saveAmberStore = async () => {
      try {
        if (isNewAmberStore.value) {
          await store.dispatch('studio/createAmberStore', amberStore.value);
        } else {
          await store.dispatch('studio/updateAmberStore', amberStore.value);
        }
        router.push({ name: 'AmberStores' });
      } catch (error) {
        console.error('Error saving Amber Store:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const cancel = () => {
      router.push({ name: 'AmberStores' });
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

<style scoped>
.amber-store-editor {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}
</style>