import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import webExtension, { readJsonFile } from 'vite-plugin-web-extension';

const target: string = process.env.TARGET_BROWSER || 'firefox';

function generateManifest() {
    const manifest = readJsonFile('src/manifest.json');
    const pkg = readJsonFile('package.json');
    return {
        name: pkg.name,
        description: pkg.description,
        version: pkg.version,
        ...manifest,
    };
}

export default defineConfig({
    plugins: [
        solidPlugin(),
        webExtension({
            manifest: generateManifest,
            watchFilePaths: ['package.json', 'manifest.json'],
            browser: target,
            disableAutoLaunch: true,
        }),
    ],
    build: {
        outDir: target === 'firefox' ? 'firefox-dist' : 'chrome-dist',
        watch: {
            include: 'src/**/*',
        },
    },
});
