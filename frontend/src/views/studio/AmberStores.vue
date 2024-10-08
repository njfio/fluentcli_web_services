<template>
  <div class="amber-stores">
    <h1 class="text-2xl font-bold mb-4">Amber Stores</h1>
    <div class="mb-4 flex justify-between items-center">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search amber stores..."
        class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
      />
      <button
        @click="createNewAmberStore"
        class="bg-primary-500 text-white px-4 py-2 rounded-lg hover:bg-primary-600 transition-colors duration-200"
      >
        Create New Amber Store
      </button>
    </div>
    <div class="overflow-x-auto shadow-md rounded-lg">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-primary-600">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/4">Name</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/4">Description</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Created At</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Last Modified</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-white uppercase tracking-wider w-1/6">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="amberStore in filteredAmberStores" :key="amberStore.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              <router-link :to="{ name: 'AmberStoreEditor', params: { id: amberStore.id } }" class="text-primary-600 hover:text-primary-900">
                <span :title="amberStore.name" class="truncate block max-w-xs">{{ amberStore.name }}</span>
              </router-link>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <span :title="amberStore.description" class="truncate block max-w-xs">{{ amberStore.description }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ formatDate(amberStore.createdAt) }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ formatDate(amberStore.lastModified) }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
              <router-link :to="{ name: 'AmberStoreEditor', params: { id: amberStore.id } }" class="text-primary-600 hover:text-primary-900 mr-2">Edit</router-link>
              <button @click="deleteAmberStore(amberStore.id)" class="text-red-600 hover:text-red-900">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import { formatDate } from '@/utils/dateFormatter';

export default defineComponent({
  name: 'AmberStores',
  setup() {
    const store = useStore();
    const router = useRouter();
    const searchQuery = ref('');

    const amberStores = computed(() => store.getters['studio/getAmberStores']);
    const filteredAmberStores = computed(() => {
      return amberStores.value.filter((amberStore: any) =>
        amberStore.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        amberStore.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    });

    onMounted(() => {
      store.dispatch('studio/fetchAmberStores');
    });

    const createNewAmberStore = () => {
      router.push({ name: 'AmberStoreEditor', params: { id: 'new' } });
    };

    const deleteAmberStore = async (id: string) => {
      if (confirm('Are you sure you want to delete this Amber Store?')) {
        try {
          await store.dispatch('studio/deleteAmberStore', id);
          // Refresh the Amber Stores list after deletion
          await store.dispatch('studio/fetchAmberStores');
        } catch (error) {
          console.error('Error deleting Amber Store:', error);
          // Handle error (e.g., show an error message to the user)
        }
      }
    };

    return {
      searchQuery,
      filteredAmberStores,
      createNewAmberStore,
      deleteAmberStore,
      formatDate,
    };
  },
});
</script>

<style scoped>
.amber-stores {
  @apply p-6;
}

.truncate {
  @apply overflow-hidden text-ellipsis;
}
</style>