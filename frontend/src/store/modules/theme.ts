import { Module } from 'vuex';
import { RootState } from '@/store/types';

interface ThemeState {
    isDarkMode: boolean;
}

const theme: Module<ThemeState, RootState> = {
    namespaced: true,
    state: {
        isDarkMode: false,
    },
    mutations: {
        toggleDarkMode(state) {
            state.isDarkMode = !state.isDarkMode;
        },
        setDarkMode(state, value: boolean) {
            state.isDarkMode = value;
        },
    },
    actions: {
        toggleDarkMode({ commit }) {
            commit('toggleDarkMode');
        },
        setDarkMode({ commit }, value: boolean) {
            commit('setDarkMode', value);
        },
    },
    getters: {
        isDarkMode: (state) => state.isDarkMode,
    },
};

export default theme;