<template>
    <ul class="tree-view">
      <li v-for="item in items" :key="item.name">
        <div @click="toggleItem(item)">
          <span v-if="item.children && item.children.length">
            {{ item.expanded ? '▼' : '▶' }}
          </span>
          <span @click="$emit('item-click', item)">{{ item.name }}</span>
        </div>
        <TreeView
          v-if="item.children && item.children.length && item.expanded"
          :items="item.children"
          @item-click="$emit('item-click', $event)"
        />
      </li>
    </ul>
  </template>
  
  <script lang="ts">
  import { defineComponent, PropType } from 'vue';
  
  interface TreeItem {
    name: string;
    children?: TreeItem[];
    expanded?: boolean;
  }
  
  export default defineComponent({
    name: 'TreeView',
    props: {
      items: {
        type: Array as PropType<TreeItem[]>,
        required: true,
      },
    },
    emits: ['item-click'],
    methods: {
      toggleItem(item: TreeItem) {
        if (item.children && item.children.length) {
          item.expanded = !item.expanded;
        }
      },
    },
  });
  </script>
  
  <style scoped>
  .tree-view {
    list-style-type: none;
    padding-left: 1rem;
  }
  .tree-view li {
    cursor: pointer;
  }
  </style>