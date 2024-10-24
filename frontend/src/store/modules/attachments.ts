import { Module } from 'vuex'
import { RootState } from '../types'
import apiClient from '@/services/apiClient'

interface Attachment {
    id: string
    message_id: string
    conversation_id: string
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
        },
        removeAttachment(state, attachmentId: string) {
            state.attachments = state.attachments.filter(att => att.id !== attachmentId)
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
                const messagesPromises = conversations.map((conv: any) => apiClient.getMessages(conv.id))
                const messagesResponses = await Promise.all(messagesPromises)

                // Collect all messages with attachments
                const attachments = messagesResponses
                    .flatMap(response => response.data)
                    .filter((msg: any) => msg.attachment_id)
                    .map((msg: any) => ({
                        id: msg.attachment_id,
                        message_id: msg.id,
                        conversation_id: msg.conversation_id,
                        file_type: 'Image',
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
