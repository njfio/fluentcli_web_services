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
            default: 'markdown',
        },
        theme: {
            type: String,
            default: 'vs-dark',
        },
        options: {
            type: Object,
            default: () => ({}),
        },
    },
    emits: ['update:modelValue', 'keydown', 'send-message'],
    setup(props, { emit }) {
        const editorContainer = ref<HTMLElement | null>(null);
        let editor: monaco.editor.IStandaloneCodeEditor | null = null;
        const currentTheme = ref(props.theme);

        const initMonaco = () => {
            if (editorContainer.value) {
                if (editor) {
                    editor.dispose();
                }
                editor = monaco.editor.create(editorContainer.value, {
                    value: props.modelValue,
                    language: props.language,
                    theme: currentTheme.value,
                    automaticLayout: true,
                    minimap: { enabled: false },
                    scrollBeyondLastLine: false,
                    lineNumbers: 'off',
                    glyphMargin: false,
                    folding: false,
                    lineDecorationsWidth: 0,
                    lineNumbersMinChars: 0,
                    wordWrap: 'on',
                    wrappingStrategy: 'advanced',
                    fontSize: 14,
                    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
                    cursorBlinking: 'smooth',
                    cursorSmoothCaretAnimation: 'on',
                    smoothScrolling: true,
                    contextmenu: false,
                    quickSuggestions: false,
                    suggestOnTriggerCharacters: false,
                    acceptSuggestionOnEnter: 'off',
                    tabCompletion: 'off',
                    wordBasedSuggestions: 'off',
                    parameterHints: { enabled: false },
                    links: false,
                    renderWhitespace: 'none',
                    overviewRulerLanes: 0,
                    hideCursorInOverviewRuler: true,
                    scrollbar: {
                        vertical: 'hidden',
                        horizontal: 'hidden'
                    },
                    renderLineHighlight: 'none',
                    fixedOverflowWidgets: true,
                    ...props.options,
                });

                editor.onDidChangeModelContent(() => {
                    emit('update:modelValue', editor?.getValue());
                });

                editor.onKeyDown((e) => {
                    if ((e.ctrlKey || e.metaKey) && e.keyCode === monaco.KeyCode.Enter) {
                        e.preventDefault();
                        emit('send-message');
                    } else {
                        emit('keydown', e);
                    }
                });
            }
        };


        const handleNewline = () => {
            if (editor) {
                const selection = editor.getSelection();
                if (selection) {
                    const position = selection.getPosition();
                    editor.executeEdits('', [{
                        range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column),
                        text: '\n',
                        forceMoveMarkers: true
                    }]);
                    editor.setPosition({ lineNumber: position.lineNumber + 1, column: 1 });
                }
            }
        };

        onMounted(() => {
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
            currentTheme.value = newTheme;
            if (editor) {
                monaco.editor.setTheme(newTheme);
            }
        });

        watch(() => props.options, (newOptions) => {
            if (editor) {
                editor.updateOptions(newOptions);
            }
        }, { deep: true });

        return {
            editorContainer,
            currentTheme,
            handleNewline,
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