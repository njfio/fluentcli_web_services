import { Module } from 'vuex'
import { RootState } from '../types'
import apiClient from '@/services/apiClient'

interface Attachment {
    id: string
    message_id: string
    file_type: string
    created_at: string
}

interface AttachmentsState {
    attachments: Attachment[]
    loading: boolean
    error: string | null
}

const attachments: Module<AttachmentsState, RootState> = {
    namespaced: true,

    state: () => ({
        attachments: [],
        loading: false,
        error: null
    }),

    mutations: {
        setAttachments(state, attachments: Attachment[]) {
            state.attachments = attachments
        },
        setLoading(state, loading: boolean) {
            state.loading = loading
        },
        setError(state, error: string | null) {
            state.error = error
        }
    },

    actions: {
        async fetchAttachments({ commit }) {
            commit('setLoading', true)
            commit('setError', null)
            try {
                // First get all conversations
                const conversationsResponse = await apiClient.listConversations()
                const conversations = conversationsResponse.data

                // Then get messages for each conversation
                const allMessages = await Promise.all(
                    conversations.map(async (conv: any) => {
                        const messagesResponse = await apiClient.getMessages(conv.id)
                        return messagesResponse.data
                    })
                )

                // Flatten messages array and collect unique attachment IDs
                const messages = allMessages.flat()
                const attachments = messages
                    .filter((msg: any) => msg.attachment_id)
                    .map((msg: any) => ({
                        id: msg.attachment_id,
                        message_id: msg.id,
                        file_type: 'Image', // We're only showing images in the gallery
                        created_at: msg.created_at
                    }))

                commit('setAttachments', attachments)
            } catch (error) {
                commit('setError', 'Failed to fetch attachments')
                console.error('Error fetching attachments:', error)
            } finally {
                commit('setLoading', false)
            }
        }
    }
}

export default attachments
