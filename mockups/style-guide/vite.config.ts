import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';

export default defineConfig({
  plugins: [solidPlugin(), vanillaExtractPlugin()],
  root: import.meta.dirname,
  server: {
    port: 5174,
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
  },
});
