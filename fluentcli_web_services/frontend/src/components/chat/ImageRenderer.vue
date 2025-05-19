<template>
    <div class="image-renderer">
        <img v-if="imageUrl" :src="imageUrl" :alt="altText" @load="onImageLoad" @error="onImageError" />
        <div v-if="loading" class="loading-overlay">Loading...</div>
        <div v-if="error" class="error-message">Failed to load image: {{ errorMessage }}</div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUnmounted, watch } from 'vue';
import apiClient from '../../services/apiClient';

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
                // Clear previous image and reset state
                if (imageUrl.value && !imageUrl.value.startsWith('http')) {
                    URL.revokeObjectURL(imageUrl.value);
                }
                imageUrl.value = '';
                loading.value = true;
                error.value = false;
                errorMessage.value = '';

                console.log('Fetching attachment with ID:', props.attachmentId);
                const response = await apiClient.getAttachment(props.attachmentId);
                console.log('Attachment response received');

                if (props.isDalle && response.data && typeof response.data === 'object' && 'url' in response.data) {
                    imageUrl.value = response.data.url;
                } else if (!props.isDalle && response.headers['content-type'].startsWith('image/')) {
                    const blob = new Blob([response.data], { type: response.headers['content-type'] });
                    imageUrl.value = URL.createObjectURL(blob);
                } else {
                    throw new Error('Unexpected response format');
                }
                console.log('New image URL created:', imageUrl.value);
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

        // Watch for changes in attachmentId
        watch(() => props.attachmentId, (newId, oldId) => {
            console.log('AttachmentId changed from', oldId, 'to', newId);
            if (newId && newId !== oldId) {
                loadImage();
            }
        });

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
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
}

img {
    width: 100%;
    height: auto;
    display: block;
    border-radius: 8px;
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
    border-radius: 8px;
}

.error-message {
    color: #dc2626;
    text-align: center;
    padding: 1rem;
    background-color: #fee2e2;
    border-radius: 8px;
    margin-top: 0.5rem;
}

.dark .loading-overlay {
    background-color: rgba(0, 0, 0, 0.7);
}

.dark .error-message {
    color: #ef4444;
    background-color: #7f1d1d;
}
</style>
