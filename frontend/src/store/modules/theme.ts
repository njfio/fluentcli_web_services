import { Module } from 'vuex';
import { RootState } from '@/store/types';

interface ThemeState {
    isDarkMode: boolean;
    isInitialized: boolean;
}

const theme: Module<ThemeState, RootState> = {
    namespaced: true,
    state: {
        isDarkMode: false,
        isInitialized: false
    },
    mutations: {
        toggleDarkMode(state) {
            state.isDarkMode = !state.isDarkMode;
            localStorage.setItem('darkMode', state.isDarkMode.toString());
            console.log('Dark mode toggled:', state.isDarkMode);
        },
        setDarkMode(state, value: boolean) {
            state.isDarkMode = value;
            localStorage.setItem('darkMode', value.toString());
            console.log('Dark mode set to:', value);
        },
        setInitialized(state, value: boolean) {
            state.isInitialized = value;
            console.log('Theme module initialized:', value);
        }
    },
    actions: {
        toggleDarkMode({ commit }) {
            console.log('Toggle dark mode action dispatched');
            commit('toggleDarkMode');
        },
        setDarkMode({ commit }, value: boolean) {
            console.log('Set dark mode action dispatched with value:', value);
            commit('setDarkMode', value);
        },
        initDarkMode({ commit, state }) {
            if (!state.isInitialized) {
                const darkMode = localStorage.getItem('darkMode') === 'true';
                console.log('Initializing dark mode from localStorage:', darkMode);
                commit('setDarkMode', darkMode);
                commit('setInitialized', true);
            } else {
                console.log('Theme module already initialized');
            }
        },
    },
    getters: {
        isDarkMode: (state) => state.isDarkMode,
        isInitialized: (state) => state.isInitialized,
    },
};

export default theme;