# Repository Guidelines

## Project Structure & Module Organization
- `src/` holds the SvelteKit app. Routes live under `src/routes/` (e.g., `+page.svelte`, nested route folders like `routes/lessons/`).
- Reusable UI and utilities live in `src/lib/` (e.g., `components/`, `styles/`, `types/`, `server/`).
- Static assets are in `static/` (served as-is at the site root).
- App shell and server hooks live in `src/app.html` and `src/hooks.server.ts`.

## Build, Test, and Development Commands
- `npm run dev` — start the Vite dev server for local development.
- `npm run dev -- --open` — start the dev server and open a browser tab.
- `npm run build` — create a production build.
- `npm run preview` — preview the production build locally.
- `npm run check` — run `svelte-check` with the project `tsconfig`.
- `npm run check:watch` — run type checks in watch mode.

## Coding Style & Naming Conventions
- Language: TypeScript + Svelte 5.
- Indentation follows existing files (tabs in `package.json`, 2 spaces in Svelte/TS is common here—match surrounding code).
- SvelteKit routing conventions: `+page.svelte`, `+layout.svelte`, `+server.ts`, etc.
- Component names are PascalCase; route folders use lowercase (e.g., `routes/lessons/`).
- Prefer colocating route-specific logic under its route folder; shared code goes in `src/lib/`.

## Testing Guidelines
- No automated test framework is configured yet.
- Use `npm run check` to validate types and Svelte diagnostics before PRs.

## Commit & Pull Request Guidelines
- Commit messages in history are short, imperative, and descriptive (e.g., “Add dictionary service…”, “Refactor lesson page…”). Follow that style.
- PRs should include a clear description of changes, any relevant screenshots for UI updates, and links to related issues if applicable.

## Security & Configuration Tips
- Configuration is managed through standard SvelteKit and Vite files (`svelte.config.js`, `vite.config.ts`).
- Keep secrets out of the repo; use environment variables for credentials (e.g., Supabase keys).
