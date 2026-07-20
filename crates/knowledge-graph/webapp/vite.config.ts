import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import svgr from 'vite-plugin-svgr';

export default defineConfig({
  plugins: [
    react({
      babel: {
        plugins: ['babel-plugin-react-compiler'],
      },
    }),
    // SVG files imported with ?react are transformed into React components.
    // Plain ?url / asset imports remain unaffected.
    svgr(),
  ],
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    sourcemap: true,
    emptyOutDir: true,
    rollupOptions: {
      output: {
        manualChunks: (id: string) => {
          if (id.includes('react-router-dom')) return 'vendor-router';
          if (id.includes('@xyflow/react')) return 'vendor-xyflow';
          if (id.includes('react-markdown') || id.includes('remark-gfm')) return 'vendor-markdown';
          if (id.includes('react-json-view-lite')) return 'vendor-json-view';
          if (id.includes('react-resizable-panels')) return 'vendor-panels';
        },
      },
    },
  },
  server: {
    port: 3000,
    fs: {
      // Allow the dev server to serve files from the parent directory so that
      // import.meta.glob in src/data/helpContent.ts can resolve
      // ../../../resources/help/*.md at dev time.
      // This restriction does not apply to production builds (Rollup resolves
      // glob imports statically at build time with no path restrictions).
      allow: ['..'],
    },
    proxy: {
      // Proxy WebSocket connections to the Spring Boot backend
      '/ws': {
        target: 'ws://localhost:8085',
        ws: true,
        changeOrigin: true,
      },
      // Proxy API endpoints to the Spring Boot backend
      '/info': {
        target: 'http://localhost:8085',
        changeOrigin: true,
      },
      '/health': {
        target: 'http://localhost:8085',
        changeOrigin: true,
      },
      '/env': {
        target: 'http://localhost:8085',
        changeOrigin: true,
      },
      '/api': {
        target: 'http://localhost:8085',
        changeOrigin: true,
      },
    },
  },
});

