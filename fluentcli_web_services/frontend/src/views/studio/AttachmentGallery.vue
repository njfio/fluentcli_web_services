<template>
    <div class="p-4">
        <div class="mb-6">
            <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Image Gallery</h1>
            <p class="text-gray-600 dark:text-gray-400">Browse and manage your image attachments</p>
        </div>

        <!-- Gallery Controls -->
        <div class="mb-6 flex flex-wrap gap-4 items-center">
            <!-- Items per page control -->
            <div class="flex items-center gap-2">
                <label class="text-sm text-gray-600 dark:text-gray-400">Items per page:</label>
                <select v-model="itemsPerPage"
                    class="rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-1">
                    <option v-for="n in [12, 24, 36, 48]" :key="n" :value="n">{{ n }}</option>
                </select>
            </div>

            <!-- Columns control -->
            <div class="flex items-center gap-2">
                <label class="text-sm text-gray-600 dark:text-gray-400">Columns:</label>
                <select v-model="columnCount"
                    class="rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-1">
                    <option v-for="n in [2, 3, 4, 5, 6]" :key="n" :value="n">{{ n }}</option>
                </select>
            </div>

            <!-- Page info -->
            <div class="text-sm text-gray-600 dark:text-gray-400 ml-auto">
                Showing {{ paginationInfo.start }}-{{ paginationInfo.end }} of {{ attachments.length }} images
            </div>
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
        <div v-else-if="paginatedAttachments.length > 0" class="grid gap-4"
            :style="{ gridTemplateColumns: `repeat(${columnCount}, minmax(0, 1fr))` }">
            <div v-for="attachment in paginatedAttachments" :key="attachment.id"
                class="relative group bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden hover:shadow-md transition-shadow duration-200">
                <!-- Image Container -->
                <div class="aspect-square w-full overflow-hidden cursor-pointer" @click="openLightbox(attachment)">
                    <ImageRenderer :attachmentId="attachment.id" :isDalle="false" :altText="'Image ' + attachment.id"
                        class="w-full h-full object-cover" />
                </div>

                <!-- Info and Actions -->
                <div class="p-3">
                    <div class="flex justify-between items-center">
                        <div class="text-sm text-gray-600 dark:text-gray-400">
                            {{ formatDate(attachment.created_at) }}
                        </div>
                        <div class="flex gap-2">
                            <!-- View in Chat Link -->
                            <button @click="navigateToChat(attachment.conversation_id)"
                                class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                                    stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                </svg>
                            </button>
                            <!-- Download Button -->
                            <button @click.stop="downloadImage(attachment.id)"
                                class="text-gray-600 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                                    stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                </svg>
                            </button>
                        </div>
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

        <!-- Pagination Controls -->
        <div v-if="attachments.length > 0" class="mt-6 flex justify-center gap-2">
            <button @click="currentPage = 1" :disabled="currentPage === 1"
                class="px-3 py-1 rounded-md bg-gray-100 dark:bg-gray-800 disabled:opacity-50">
                First
            </button>
            <button @click="previousPage" :disabled="currentPage === 1"
                class="px-3 py-1 rounded-md bg-gray-100 dark:bg-gray-800 disabled:opacity-50">
                Previous
            </button>
            <span class="px-3 py-1">Page {{ currentPage }} of {{ totalPages }}</span>
            <button @click="nextPage" :disabled="currentPage === totalPages"
                class="px-3 py-1 rounded-md bg-gray-100 dark:bg-gray-800 disabled:opacity-50">
                Next
            </button>
            <button @click="currentPage = totalPages" :disabled="currentPage === totalPages"
                class="px-3 py-1 rounded-md bg-gray-100 dark:bg-gray-800 disabled:opacity-50">
                Last
            </button>
        </div>

        <!-- Lightbox Modal -->
        <div v-if="lightboxOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-90"
            @click="closeLightbox">
            <!-- Previous button -->
            <button v-if="currentIndex > 0"
                class="fixed left-4 top-1/2 -translate-y-1/2 z-50 p-3 rounded-full bg-black/50 text-white hover:bg-black/70 focus:outline-none"
                @click.stop="previousImage">
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
                    class="absolute top-4 right-4 p-2 rounded-full bg-black/50 text-white hover:bg-black/70 focus:outline-none">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <!-- Action buttons -->
                <div class="absolute bottom-4 right-4 flex gap-2">
                    <!-- View in Chat Link -->
                    <button v-if="currentAttachment" @click.stop="navigateToChat(currentAttachment.conversation_id)"
                        class="p-2 bg-blue-600 text-white rounded-full hover:bg-blue-700 focus:outline-none">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                        </svg>
                    </button>
                    <!-- Download button -->
                    <button @click.stop="downloadImage(currentAttachment?.id)"
                        class="p-2 bg-white text-gray-800 rounded-full hover:bg-gray-100 focus:outline-none">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                        </svg>
                    </button>
                </div>
            </div>

            <!-- Next button -->
            <button v-if="currentIndex < attachments.length - 1"
                class="fixed right-4 top-1/2 -translate-y-1/2 z-50 p-3 rounded-full bg-black/50 text-white hover:bg-black/70 focus:outline-none"
                @click.stop="nextImage">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, computed, ref, watch } from 'vue'
import { useStore } from 'vuex'
import { useRouter } from 'vue-router'
import { useChatLogic } from '../../components/chat/ChatLogic'
import ImageRenderer from '../../components/chat/ImageRenderer.vue'
import apiClient from '../../services/apiClient'

interface Attachment {
    id: string
    message_id: string
    conversation_id: string
    file_type: string
    created_at: string
}

interface Conversation {
    id: string
    title: string
    mode: string
    createdAt: string
    updatedAt: string
}

const store = useStore()
const router = useRouter()
const { loadMessages } = useChatLogic()

// Pagination and layout state
const currentPage = ref(1)
const itemsPerPage = ref(24)
const columnCount = ref(4)

const attachments = computed(() => store.state.attachments.attachments as Attachment[])
const loading = computed(() => store.state.attachments.loading)
const error = computed(() => store.state.attachments.error)

// Pagination computed properties
const totalPages = computed(() => Math.ceil(attachments.value.length / itemsPerPage.value))
const paginatedAttachments = computed(() => {
    const start = (currentPage.value - 1) * itemsPerPage.value
    const end = start + itemsPerPage.value
    return attachments.value.slice(start, end)
})

const paginationInfo = computed(() => {
    const start = (currentPage.value - 1) * itemsPerPage.value + 1
    const end = Math.min(start + itemsPerPage.value - 1, attachments.value.length)
    return { start, end }
})

// Lightbox state
const lightboxOpen = ref(false)
const currentIndex = ref(0)
const currentAttachment = computed(() =>
    lightboxOpen.value ? attachments.value[currentIndex.value] : null
)

// Watch for changes that should reset pagination
watch([itemsPerPage], () => {
    currentPage.value = 1
})

// Pagination methods
function nextPage() {
    if (currentPage.value < totalPages.value) {
        currentPage.value++
    }
}

function previousPage() {
    if (currentPage.value > 1) {
        currentPage.value--
    }
}

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

function openLightbox(attachment: Attachment) {
    const index = attachments.value.findIndex((a: Attachment) => a.id === attachment.id)
    if (index !== -1) {
        currentIndex.value = index
        lightboxOpen.value = true
    }
}

function closeLightbox() {
    lightboxOpen.value = false
}

function previousImage(event?: Event) {
    event?.stopPropagation()
    if (currentIndex.value > 0) {
        currentIndex.value--
    }
}

function nextImage(event?: Event) {
    event?.stopPropagation()
    if (currentIndex.value < attachments.value.length - 1) {
        currentIndex.value++
    }
}

async function navigateToChat(conversationId: string) {
    try {
        // First ensure conversations are loaded
        if (store.state.chat.conversations.length === 0) {
            await store.dispatch('chat/getConversations')
        }

        // Get the conversation first
        const response = await apiClient.getConversation(conversationId)
        const conversation = response.data

        // Add it to the conversations list if it's not there
        if (!store.state.chat.conversations.find((c: Conversation) => c.id === conversation.id)) {
            store.commit('chat/addConversation', conversation)
        }

        // Set it as current conversation
        store.commit('chat/setCurrentConversation', conversation)

        // Load messages using ChatLogic's loadMessages
        await loadMessages(conversationId)

        // Then navigate to the chat view
        router.push({
            name: 'Chat',
            params: { conversationId }
        })
    } catch (err) {
        console.error('Error navigating to chat:', err)
    }
}

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
