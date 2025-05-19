// src/extensions/DataPill.ts

import { Node, mergeAttributes } from '@tiptap/core'

export interface DataPillOptions {
  HTMLAttributes: Record<string, any>
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    dataPill: {
      insertDataPill: (value: string) => ReturnType
    }
  }
}

export const DataPill = Node.create<DataPillOptions>({
  name: 'dataPill',

  group: 'inline',

  inline: true,

  atom: true,

  addOptions() {
    return {
      HTMLAttributes: {},
    }
  },

  addAttributes() {
    return {
      value: {
        default: null,
      },
    }
  },

  parseHTML() {
    return [
      {
        tag: 'span[data-type="data-pill"]',
      },
    ]
  },

  renderHTML({ node, HTMLAttributes }) {
    return ['span', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes, { 'data-type': 'data-pill' }), node.attrs.value]
  },

  addCommands() {
    return {
      insertDataPill: (value: string) => ({ chain }) => {
        return chain()
          .insertContent({
            type: this.name,
            attrs: { value },
          })
          .run()
      },
    }
  },
})