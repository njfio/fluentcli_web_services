<template>
  <div v-if="isThemeInitialized" class="amber-store-editor dark:bg-gray-900">
    <h1 class="text-2xl font-bold mb-6 dark:text-white">
      {{ isNewAmberStore ? 'Create New Amber Store' : 'Edit Amber Store' }}
    </h1>
    <div class="flex justify-end mb-6">
      <button type="button" @click="cancel"
        class="bg-white dark:bg-gray-700 py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-900">
        Cancel
      </button>
      <button type="submit" form="amber-store-form"
        class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 dark:bg-primary-700 dark:hover:bg-primary-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 dark:focus:ring-offset-gray-900">
        {{ isNewAmberStore ? 'Create' : 'Update' }}
      </button>
    </div>
    <form id="amber-store-form" @submit.prevent="saveAmberStore" class="space-y-6">
      <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:p-6">
          <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div class="sm:col-span-4">
              <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Name</label>
              <input type="text" id="name" v-model="amberStoreData.name" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white">
            </div>
            <div class="sm:col-span-4">
              <label for="secure_key_hash" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Secure Key
                Hash</label>
              <input type="text" id="secure_key_hash" v-model="amberStoreData.secure_key_hash" required
                class="mt-1 focus:ring-primary-500 focus:border-primary-500 block w-full shadow-sm sm:text-sm border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white">
            </div>
            <div class="sm:col-span-6">
              <label for="data" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Amber Store Data
                (JSON)</label>
              <div class="mt-1 border border-gray-300 dark:border-gray-600 rounded-md overflow-hidden dark:bg-gray-800"
                style="height: calc(100vh - 400px);">
                <MonacoEditor :key="currentTheme" v-model="amberStoreData.data" language="json" :theme="currentTheme"
                  :options="editorOptions" class="h-full" @update:modelValue="onEditorUpdate" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
  <div v-else class="flex justify-center items-center h-screen">
    <p class="text-xl">Loading...</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted, watch } from 'vue';
import { useStore } from 'vuex';
import { useRouter, useRoute } from 'vue-router';
import MonacoEditor from './MonacoEditor.vue';

export default defineComponent({
  name: 'AmberStoreEditor',
  components: {
    MonacoEditor,
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const route = useRoute();

    const amberStoreData = ref({
      id: '',
      name: '',
      data: '',
      secure_key_hash: '',
    });

    const isNewAmberStore = computed(() => route.params.id === 'new');
    const isDarkMode = computed(() => store.getters['theme/isDarkMode']);
    const isThemeInitialized = computed(() => store.getters['theme/isInitialized']);
    const currentTheme = computed(() => isDarkMode.value ? 'vs-dark' : 'vs-light');

    const editorOptions = {
      minimap: { enabled: false },
      lineNumbers: 'on',
      roundedSelection: false,
      scrollBeyondLastLine: false,
      readOnly: false,
    };

    onMounted(async () => {
      if (!isNewAmberStore.value) {
        const id = route.params.id as string;
        console.log('Fetching amber store with ID:', id);
        await store.dispatch('studio/fetchAmberStoreById', id);
        const fetchedAmberStore = store.getters['studio/getCurrentAmberStore'];
        console.log('Fetched amber store:', fetchedAmberStore);
        if (fetchedAmberStore) {
          amberStoreData.value = { ...fetchedAmberStore };
        }
        console.log('Initial amber store data:', amberStoreData.value.data);
      }
    });

    const onEditorUpdate = (value: string) => {
      console.log('Editor update:', value);
      amberStoreData.value.data = value;
    };

    const saveAmberStore = async () => {
      try {
        console.log('Saving amber store:', amberStoreData.value);
        console.log('Amber store data before save:', amberStoreData.value.data);
        if (isNewAmberStore.value) {
          await store.dispatch('studio/createAmberStore', amberStoreData.value);
        } else {
          await store.dispatch('studio/updateAmberStore', amberStoreData.value);
        }
        console.log('Amber store saved successfully');
        router.push({ name: 'AmberStores' });
      } catch (error) {
        console.error('Error saving Amber Store:', error);
        // Handle error (e.g., show an error message to the user)
      }
    };

    const cancel = () => {
      router.push({ name: 'AmberStores' });
    };

    watch(() => amberStoreData.value.data, (newValue) => {
      console.log('Amber store data changed:', newValue);
    });

    return {
      amberStoreData,
      isNewAmberStore,
      currentTheme,
      isThemeInitialized,
      editorOptions,
      saveAmberStore,
      cancel,
      onEditorUpdate,
    };
  },
});
</script>

<style scoped>
.amber-store-editor {
  @apply max-w-7xl mx-auto py-6 sm:px-6 lg:px-8;
}
</style>