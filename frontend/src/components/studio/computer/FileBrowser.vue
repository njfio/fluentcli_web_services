<template>
    <div class="flex h-full bg-gray-900">
        <!-- File list section -->
        <div class="w-1/2 border-r border-gray-700">
            <div class="p-4">
                <!-- Current path display -->
                <div class="flex items-center mb-4 text-gray-300">
                    <button @click="navigateUp" class="mr-2 hover:text-blue-500" :disabled="isLoading">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd"
                                d="M9.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L7.414 9H15a1 1 0 110 2H7.414l2.293 2.293a1 1 0 010 1.414z"
                                clip-rule="evenodd" />
                        </svg>
                    </button>
                    <span class="truncate">{{ currentPath || '/' }}</span>
                </div>

                <!-- Loading indicator -->
                <div v-if="isLoading" class="flex items-center justify-center py-4">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                </div>

                <!-- Error message -->
                <div v-else-if="error" class="text-red-500 p-4 bg-red-900 bg-opacity-25 rounded">
                    {{ error }}
                </div>

                <!-- File list -->
                <div v-else class="space-y-2 max-h-[calc(100vh-12rem)] overflow-y-auto">
                    <div v-for="item in fileList" :key="item.path" @click="handleItemClick(item)"
                        class="flex items-center p-2 rounded hover:bg-gray-800 cursor-pointer text-gray-300">
                        <!-- Directory icon -->
                        <svg v-if="item.is_dir" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-yellow-500"
                            viewBox="0 0 20 20" fill="currentColor">
                            <path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" />
                        </svg>
                        <!-- File icon -->
                        <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-500"
                            viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd"
                                d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z"
                                clip-rule="evenodd" />
                        </svg>
                        <span class="truncate">{{ item.name }}</span>
                    </div>
                </div>
            </div>
        </div>

        <!-- Preview section -->
        <div class="w-1/2 p-4">
            <div v-if="selectedFile" class="h-full">
                <!-- Preview header -->
                <div class="mb-4 text-gray-300 font-medium">
                    {{ selectedFile.name }}
                </div>

                <!-- Preview content -->
                <div class="bg-gray-800 rounded p-4 h-[calc(100%-3rem)] overflow-auto">
                    <!-- Loading indicator -->
                    <div v-if="isLoadingContent" class="flex items-center justify-center py-4">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                    </div>

                    <!-- Error message -->
                    <div v-else-if="contentError" class="text-red-500">
                        {{ contentError }}
                    </div>

                    <!-- Content preview -->
                    <template v-else>
                        <!-- Text preview -->
                        <pre v-if="isTextFile"
                            class="text-gray-300 font-mono text-sm whitespace-pre-wrap">{{ fileContent }}</pre>

                        <!-- Image preview -->
                        <img v-else-if="isImageFile" :src="fileContent" class="max-w-full h-auto" alt="Preview">

                        <!-- Binary file message -->
                        <div v-else class="text-gray-400">
                            Binary file - preview not available
                        </div>
                    </template>
                </div>
            </div>
            <div v-else class="h-full flex items-center justify-center text-gray-500">
                Select a file to preview
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface FileItem {
    name: string
    path: string
    is_dir: boolean
    size: number
    modified: string
}

const currentPath = ref('/')
const fileList = ref<FileItem[]>([])
const selectedFile = ref<FileItem | undefined>(undefined)
const fileContent = ref<string | undefined>(undefined)
const isLoading = ref(false)
const isLoadingContent = ref(false)
const error = ref<string | null>(null)
const contentError = ref<string | null>(null)

// API base URL
const API_BASE = '/filesystem'

// Computed properties for file type detection
const isTextFile = computed(() => {
    if (!selectedFile.value) return false
    const textExtensions = ['.txt', '.md', '.json', '.yaml', '.yml', '.js', '.ts', '.html', '.css', '.rs', '.toml']
    return textExtensions.some(ext => selectedFile.value!.name.toLowerCase().endsWith(ext))
})

const isImageFile = computed(() => {
    if (!selectedFile.value) return false
    const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.svg', '.webp']
    return imageExtensions.some(ext => selectedFile.value!.name.toLowerCase().endsWith(ext))
})

// Navigation functions
const navigateUp = async () => {
    if (currentPath.value === '/') return
    const newPath = currentPath.value.split('/').slice(0, -1).join('/') || '/'
    await loadDirectory(newPath)
}

const handleItemClick = async (item: FileItem) => {
    console.log('Clicked item:', item)
    if (item.is_dir) {
        const newPath = item.path.startsWith('/') ? item.path : `${currentPath.value}/${item.path}`
        console.log('Navigating to:', newPath)
        await loadDirectory(newPath)
    } else {
        selectedFile.value = item
        await loadFileContent(item)
    }
}

// API interaction functions
const loadDirectory = async (path: string) => {
    isLoading.value = true
    error.value = null
    try {
        console.log('Loading directory:', path)
        const response = await fetch(`${API_BASE}?path=${encodeURIComponent(path)}`)
        if (!response.ok) {
            const errorText = await response.text()
            console.error('Server response:', errorText)
            throw new Error(errorText || 'Failed to load directory')
        }
        const data = await response.json()
        console.log('Directory contents:', data)
        fileList.value = data
        currentPath.value = path
        selectedFile.value = undefined
        fileContent.value = undefined
    } catch (err) {
        console.error('Error loading directory:', err)
        error.value = err instanceof Error ? err.message : 'Failed to load directory'
        fileList.value = []
    } finally {
        isLoading.value = false
    }
}

const loadFileContent = async (file: FileItem) => {
    isLoadingContent.value = true
    contentError.value = null
    try {
        console.log('Loading file content:', file.path)
        const response = await fetch(`${API_BASE}/content?path=${encodeURIComponent(file.path)}`)
        if (!response.ok) {
            const errorText = await response.text()
            console.error('Server response:', errorText)
            throw new Error(errorText || 'Failed to load file content')
        }

        if (isImageFile.value) {
            const blob = await response.blob()
            fileContent.value = URL.createObjectURL(blob)
        } else {
            fileContent.value = await response.text()
        }
    } catch (err) {
        console.error('Error loading file content:', err)
        contentError.value = err instanceof Error ? err.message : 'Failed to load file content'
        fileContent.value = undefined
    } finally {
        isLoadingContent.value = false
    }
}

// Initial load
onMounted(() => {
    loadDirectory('/')
})
</script>
