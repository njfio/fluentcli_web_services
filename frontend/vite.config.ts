import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      'monaco-editor': 'monaco-editor/esm/vs/editor/editor.api.js',
    },
  },
  server: {
    port: 1420,
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
      '/jobs': {
          target: 'http://localhost:8000', // Replace with your backend server's URL and port
          changeOrigin: true,
          secure: false, 
          rewrite: (path) => path.replace(/^\/api/, '')// Set to true if using HTTPS on the backend
        },
      '/pipelines': {
          target: 'http://localhost:8000', // Replace with your backend server's URL and port
          changeOrigin: true,
          secure: false,
                rewrite: (path) => path.replace(/^\/api/, ''), // Set to true if using HTTPS on the backend
        },
    },
  },
});