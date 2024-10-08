<template>
    <div ref="editorContainer" class="monaco-editor"></div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, onMounted, onBeforeUnmount, computed } from 'vue';
import * as monaco from 'monaco-editor';
import { useStore } from 'vuex';

export default defineComponent({
    name: 'MonacoEditor',
    props: {
        modelValue: {
            type: String,
            default: '',
        },
        language: {
            type: String,
            default: 'yaml',
        },
    },
    emits: ['update:modelValue'],
    setup(props, { emit }) {
        const store = useStore();
        const editorContainer = ref<HTMLElement | null>(null);
        let editor: monaco.editor.IStandaloneCodeEditor | null = null;

        const isDarkMode = computed(() => store.state.theme.darkMode);

        const getCurrentTheme = () => isDarkMode.value ? 'vs-dark' : 'vs-light';

        const initMonaco = () => {
            if (editorContainer.value) {
                editor = monaco.editor.create(editorContainer.value, {
                    value: props.modelValue,
                    language: props.language,
                    theme: getCurrentTheme(),
                    automaticLayout: true,
                    minimap: { enabled: false },
                    scrollBeyondLastLine: false,
                });

                editor.onDidChangeModelContent(() => {
                    emit('update:modelValue', editor?.getValue());
                });
            }
        };

        const updateEditorTheme = () => {
            if (editor) {
                monaco.editor.setTheme(getCurrentTheme());
            }
        };

        onMounted(() => {
            initMonaco();
        });

        onBeforeUnmount(() => {
            if (editor) {
                editor.dispose();
            }
        });

        watch(() => props.modelValue, (newValue) => {
            if (editor && newValue !== editor.getValue()) {
                editor.setValue(newValue);
            }
        });

        watch(isDarkMode, () => {
            updateEditorTheme();
        }, { immediate: true });

        return {
            editorContainer,
        };
    },
});
</script>

<style scoped>
.monaco-editor {
    width: 100%;
    height: 100%;
}
</style>