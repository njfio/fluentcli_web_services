import { Module } from 'vuex';
import { RootState } from '../types';

export interface ChatUIState {
    sidebarOpen: boolean;
    selectedConversationId: string | null;
    gridLayout: 'standard' | 'brickwork';
    gridColumns: number;
}

const chatUIModule: Module<ChatUIState, RootState> = {
    namespaced: true,

    state: {
        sidebarOpen: true,
        selectedConversationId: null,
        gridLayout: 'standard',
        gridColumns: 3
    },

    mutations: {
        SET_SIDEBAR_OPEN(state, isOpen: boolean) {
            state.sidebarOpen = isOpen;
        },
        SET_SELECTED_CONVERSATION(state, id: string | null) {
            state.selectedConversationId = id;
        },
        SET_GRID_LAYOUT(state, layout: 'standard' | 'brickwork') {
            state.gridLayout = layout;
        },
        SET_GRID_COLUMNS(state, columns: number) {
            state.gridColumns = columns;
        }
    },

    actions: {
        toggleSidebar({ commit, state }) {
            commit('SET_SIDEBAR_OPEN', !state.sidebarOpen);
        },
        selectConversation({ commit }, id: string | null) {
            commit('SET_SELECTED_CONVERSATION', id);
        },
        setGridLayout({ commit }, layout: 'standard' | 'brickwork') {
            commit('SET_GRID_LAYOUT', layout);
        },
        setGridColumns({ commit }, columns: number) {
            commit('SET_GRID_COLUMNS', columns);
        }
    }
};

export default chatUIModule;
