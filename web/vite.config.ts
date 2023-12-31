import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    minify: false,
    target: 'esnext',
  },
  // plugins: [wasmPack(['../wordclock_wasm'])]
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: ['..'],
    },
  },
});
