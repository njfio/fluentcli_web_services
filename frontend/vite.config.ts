import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path';


export default defineConfig({
  plugins: [vue()],
  server: {
    host: '0.0.0.0',
    port: 1420,
    strictPort: true,
  },
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  }
});