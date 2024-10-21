<template>
    <div class="image-renderer">
        <img v-if="imageUrl" :src="imageUrl" :alt="altText" @load="onImageLoad" @error="onImageError" />
        <div v-if="loading" class="loading-overlay">Loading...</div>
        <div v-if="error" class="error-message">Failed to load image: {{ errorMessage }}</div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUnmounted } from 'vue';
import apiClient from '@/services/apiClient';

export default defineComponent({
    name: 'ImageRenderer',
    props: {
        attachmentId: {
            type: String,
            required: true,
        },
        isDalle: {
            type: Boolean,
            default: false,
        },
        altText: {
            type: String,
            default: 'Generated image',
        },
    },
    setup(props) {
        const imageUrl = ref('');
        const loading = ref(true);
        const error = ref(false);
        const errorMessage = ref('');

        const loadImage = async () => {
            try {
                console.log('Fetching attachment with ID:', props.attachmentId);
                const response = props.isDalle
                    ? await apiClient.getAttachment(props.attachmentId)
                    : await apiClient.getAttachment(props.attachmentId);
                console.log('Attachment response:', response);

                if (props.isDalle && response.data && typeof response.data === 'object' && 'url' in response.data) {
                    imageUrl.value = response.data.url;
                } else if (!props.isDalle && response.headers['content-type'].startsWith('image/')) {
                    const blob = new Blob([response.data], { type: response.headers['content-type'] });
                    imageUrl.value = URL.createObjectURL(blob);
                } else {
                    throw new Error('Unexpected response format');
                }
                console.log('Image URL:', imageUrl.value);
            } catch (err) {
                console.error('Failed to load image:', err);
                error.value = true;
                errorMessage.value = err instanceof Error ? err.message : 'Unknown error';
            } finally {
                loading.value = false;
            }
        };

        const onImageLoad = () => {
            loading.value = false;
            console.log('Image loaded successfully');
        };

        const onImageError = (e: Event) => {
            loading.value = false;
            error.value = true;
            errorMessage.value = 'Image failed to load';
            console.error('Image failed to load:', e);
        };

        onMounted(() => {
            loadImage();
        });

        onUnmounted(() => {
            if (imageUrl.value && !imageUrl.value.startsWith('http')) {
                URL.revokeObjectURL(imageUrl.value);
            }
        });

        return {
            imageUrl,
            loading,
            error,
            errorMessage,
            onImageLoad,
            onImageError,
        };
    },
});
</script>

<style scoped>
.image-renderer {
    position: relative;
}

.loading-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(255, 255, 255, 0.7);
}

.error-message {
    color: red;
    text-align: center;
}
</style>
