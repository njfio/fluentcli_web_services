<template>
    <div class="create-job">
        <h2 class="text-2xl font-bold mb-4">Create New Job</h2>
        <JobEditor :job="null" :dockerFiles="dockerFiles" :configurations="configurations" :pipelines="pipelines"
            :amberStores="amberStores" @save="handleSave" @cancel="handleCancel" />
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, computed } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import JobEditor from '@/components/studio/editors/JobEditor.vue';

export default defineComponent({
    name: 'CreateJob',
    components: {
        JobEditor,
    },
    setup() {
        const store = useStore();
        const router = useRouter();

        onMounted(() => {
            store.dispatch('studio/fetchDockerFiles');
            store.dispatch('studio/fetchConfigurations');
            store.dispatch('studio/fetchPipelines');
            store.dispatch('studio/fetchAmberStores');
        });

        const dockerFiles = computed(() => store.getters['studio/getDockerFiles']);
        const configurations = computed(() => store.getters['studio/getConfigurations']);
        const pipelines = computed(() => store.getters['studio/getPipelines']);
        const amberStores = computed(() => store.getters['studio/getAmberStores']);

        const handleSave = async (job: any) => {
            try {
                await store.dispatch('studio/createJob', job);
                router.push({ name: 'Jobs' });
            } catch (error) {
                console.error('Error creating job:', error);
                // Handle error (e.g., show an error message to the user)
            }
        };

        const handleCancel = () => {
            router.push({ name: 'Jobs' });
        };

        return {
            dockerFiles,
            configurations,
            pipelines,
            amberStores,
            handleSave,
            handleCancel,
        };
    },
});
</script>

<style scoped>
.create-job {
    padding: 20px;
}
</style>