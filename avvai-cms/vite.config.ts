import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
	plugins: [sveltekit()],
	resolve: {
		alias: [
			// Package entry points
			{
				find: 'avvai-frontend/components/sections',
				replacement: path.resolve(__dirname, '../avvai-frontend/src/lib/components/sections/index.ts')
			},
			{
				find: 'avvai-frontend/styles/tokens',
				replacement: path.resolve(__dirname, '../avvai-frontend/src/lib/styles/tokens.css')
			},
			{
				find: 'avvai-frontend/types/lesson',
				replacement: path.resolve(__dirname, '../avvai-frontend/src/lib/types/lesson.ts')
			},
			// Resolve $lib imports from frontend components to frontend's src/lib
			{
				find: /^\$lib\/(.+)$/,
				replacement: path.resolve(__dirname, '../avvai-frontend/src/lib/$1')
			}
		]
	}
});
