import '@tiptap/core';

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    /**
     * Insert a Data Pill with the given value
     */
    insertDataPill: (value: string) => ReturnType;
  }
}

export {};