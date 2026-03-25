/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { paraglideVitePlugin } from '@inlang/paraglide-js';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { configDefaults, defineConfig } from 'vitest/config';

export default defineConfig({
    plugins: [
        tailwindcss(),
        sveltekit(),
        paraglideVitePlugin({
            project: './project.inlang',
            outdir: './src/lib/paraglide',
            strategy: ['url', 'cookie', 'baseLocale']
        })
    ],
    test: {
        environment: 'jsdom',
        exclude: [...configDefaults.exclude, 'tests/**'],
        setupFiles: ['testSetup.ts']
    },
    resolve: process.env.VITEST ? { conditions: ['browser'] } : undefined
});
