<template>
    <div ref="editorContainer" class="monaco-editor"></div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, onMounted, onBeforeUnmount, nextTick } from 'vue';
import * as monaco from 'monaco-editor';

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
        theme: {
            type: String,
            default: 'vs-light',
        },
    },
    emits: ['update:modelValue'],
    setup(props, { emit }) {
        const editorContainer = ref<HTMLElement | null>(null);
        let editor: monaco.editor.IStandaloneCodeEditor | null = null;
        const currentTheme = ref(props.theme);

        console.log('MonacoEditor setup, initial theme:', props.theme);

        const initMonaco = () => {
            if (editorContainer.value) {
                console.log('Initializing Monaco editor with theme:', currentTheme.value);
                if (editor) {
                    console.log('Disposing existing editor');
                    editor.dispose();
                }
                editor = monaco.editor.create(editorContainer.value, {
                    value: props.modelValue,
                    language: props.language,
                    theme: currentTheme.value,
                    automaticLayout: true,
                    minimap: { enabled: false },
                    scrollBeyondLastLine: false,
                });

                editor.onDidChangeModelContent(() => {
                    emit('update:modelValue', editor?.getValue());
                });

                console.log('Monaco editor initialized with theme:', currentTheme.value);
            }
        };

        onMounted(() => {
            console.log('MonacoEditor mounted, current theme:', currentTheme.value);
            nextTick(() => {
                initMonaco();
            });
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

        watch(() => props.theme, (newTheme) => {
            console.log('Theme prop changed in MonacoEditor:', newTheme);
            currentTheme.value = newTheme;
            nextTick(() => {
                if (editor) {
                    console.log('Updating Monaco editor theme to:', newTheme);
                    monaco.editor.setTheme(newTheme);
                } else {
                    console.warn('Editor not initialized, reinitializing with new theme');
                    initMonaco();
                }
            });
        }, { immediate: true });

        return {
            editorContainer,
            currentTheme,
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