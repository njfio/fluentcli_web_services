<template>
    <div class="image-renderer">
        <img v-if="isValidImageUrl" :src="content" alt="Generated image" class="generated-image" />
        <p v-else class="error-message">Invalid image URL</p>
    </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';

export default defineComponent({
    name: 'ImageRenderer',
    props: {
        content: {
            type: String,
            required: true,
        },
    },
    setup(props) {
        const isValidImageUrl = computed(() => {
            return props.content.startsWith('http') && props.content.match(/\.(jpeg|jpg|gif|png)$/) !== null;
        });

        return {
            isValidImageUrl,
        };
    },
});
</script>

<style scoped>
.image-renderer {
    width: 100%;
    max-width: 512px;
    margin: 0 auto;
}

.generated-image {
    width: 100%;
    height: auto;
    border-radius: 8px;
}

.error-message {
    color: #ef4444;
    font-size: 0.875rem;
}
</style>
