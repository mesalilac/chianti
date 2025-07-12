import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
    plugins: [solidPlugin()],
    server: {
        port: 3242,
        proxy: {
            '/api': {
                target: 'http://localhost:3241',
                changeOrigin: true,
                secure: false,
            },
        },
    },
    build: {
        outDir: '../web-dist',
        target: 'esnext',
    },
});
