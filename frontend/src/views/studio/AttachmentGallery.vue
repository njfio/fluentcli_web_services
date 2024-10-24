<template>
    <div class="p-4">
        <div class="mb-6">
            <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Image Gallery</h1>
            <p class="text-gray-600 dark:text-gray-400">Browse and manage your image attachments</p>
        </div>

        <!-- Loading state -->
        <div v-if="loading" class="text-center py-12">
            <div class="text-gray-500 dark:text-gray-400">
                Loading images...
            </div>
        </div>

        <!-- Error state -->
        <div v-else-if="error" class="text-center py-12">
            <div class="text-red-500">
                {{ error }}
            </div>
        </div>

        <!-- Gallery Grid -->
        <div v-else-if="attachments.length > 0"
            class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
            <div v-for="attachment in attachments" :key="attachment.id"
                class="relative group bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden hover:shadow-md transition-shadow duration-200 cursor-pointer"
                @click="openLightbox(attachment)">
                <div class="aspect-square w-full overflow-hidden">
                    <ImageRenderer :attachmentId="attachment.id" :isDalle="false" :altText="'Image ' + attachment.id"
                        class="w-full h-full object-cover" />
                </div>

                <!-- Overlay with actions -->
                <div
                    class="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-40 transition-opacity duration-200 flex items-center justify-center opacity-0 group-hover:opacity-100">
                    <div class="flex gap-2">
                        <button @click.stop="downloadImage(attachment.id)"
                            class="bg-white text-gray-800 p-2 rounded-full hover:bg-gray-100 transition-colors duration-200">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                            </svg>
                        </button>
                    </div>
                </div>

                <!-- Image info -->
                <div class="p-3">
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        {{ formatDate(attachment.created_at) }}
                    </div>
                </div>
            </div>
        </div>

        <!-- Empty state -->
        <div v-else class="text-center py-12">
            <div class="text-gray-500 dark:text-gray-400">
                No images found in your gallery
            </div>
        </div>

        <!-- Lightbox Modal -->
        <div v-if="lightboxOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-90"
            @click="closeLightbox">
            <div class="relative max-w-7xl w-full h-full flex items-center justify-center p-4">
                <!-- Previous button -->
                <button v-if="currentIndex > 0" @click.stop="previousImage"
                    class="absolute left-4 p-2 text-white hover:text-gray-300 focus:outline-none">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    </svg>
                </button>

                <!-- Image container -->
                <div class="relative max-h-full max-w-full" @click.stop>
                    <ImageRenderer v-if="currentAttachment" :attachmentId="currentAttachment.id" :isDalle="false"
                        :altText="'Image ' + currentAttachment.id"
                        class="max-h-[90vh] max-w-full object-contain rounded-lg" />

                    <!-- Close button -->
                    <button @click.stop="closeLightbox"
                        class="absolute top-4 right-4 p-2 text-white hover:text-gray-300 focus:outline-none">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>

                    <!-- Download button -->
                    <button @click.stop="downloadImage(currentAttachment?.id)"
                        class="absolute bottom-4 right-4 p-2 bg-white text-gray-800 rounded-full hover:bg-gray-100 focus:outline-none">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                        </svg>
                    </button>
                </div>

                <!-- Next button -->
                <button v-if="currentIndex < attachments.length - 1" @click.stop="nextImage"
                    class="absolute right-4 p-2 text-white hover:text-gray-300 focus:outline-none">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    </svg>
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, computed, ref } from 'vue'
import { useStore } from 'vuex'
import ImageRenderer from '@/components/chat/ImageRenderer.vue'
import apiClient from '@/services/apiClient'

interface Attachment {
    id: string
    message_id: string
    file_type: string
    created_at: string
}

const store = useStore()

const attachments = computed(() => store.state.attachments.attachments as Attachment[])
const loading = computed(() => store.state.attachments.loading)
const error = computed(() => store.state.attachments.error)

// Lightbox state
const lightboxOpen = ref(false)
const currentIndex = ref(0)
const currentAttachment = computed(() =>
    lightboxOpen.value ? attachments.value[currentIndex.value] : null
)

onMounted(() => {
    store.dispatch('attachments/fetchAttachments')
    window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown)
})

function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString()
}

async function downloadImage(attachmentId?: string) {
    if (!attachmentId) return

    try {
        const response = await apiClient.getAttachment(attachmentId)
        const blob = new Blob([response.data], { type: response.headers['content-type'] })
        const url = window.URL.createObjectURL(blob)
        const link = document.createElement('a')
        link.href = url
        link.download = `image-${attachmentId}.${response.headers['content-type'].split('/')[1]}`
        document.body.appendChild(link)
        link.click()
        document.body.removeChild(link)
        window.URL.revokeObjectURL(url)
    } catch (err) {
        console.error('Failed to download image:', err)
    }
}

// Lightbox functions
function openLightbox(attachment: Attachment) {
    currentIndex.value = attachments.value.findIndex((a: Attachment) => a.id === attachment.id)
    lightboxOpen.value = true
}

function closeLightbox() {
    lightboxOpen.value = false
}

function previousImage() {
    if (currentIndex.value > 0) {
        currentIndex.value--
    }
}

function nextImage() {
    if (currentIndex.value < attachments.value.length - 1) {
        currentIndex.value++
    }
}

// Keyboard navigation
function handleKeydown(event: KeyboardEvent) {
    if (!lightboxOpen.value) return

    switch (event.key) {
        case 'ArrowLeft':
            previousImage()
            break
        case 'ArrowRight':
            nextImage()
            break
        case 'Escape':
            closeLightbox()
            break
    }
}
</script>

<style scoped>
.aspect-square {
    aspect-ratio: 1 / 1;
}

/* Fade transition for lightbox */
.lightbox-enter-active,
.lightbox-leave-active {
    transition: opacity 0.3s ease;
}

.lightbox-enter-from,
.lightbox-leave-to {
    opacity: 0;
}
</style>
