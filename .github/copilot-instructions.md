# Copilot instructions for Avvai

## Big picture
- Monorepo with three apps: Rust backend in [avvai-backend](avvai-backend), SvelteKit 5 frontend in [avvai-frontend](avvai-frontend), and a minimal SvelteKit CMS in [avvai-cms](avvai-cms).
- Backend serves JSON lesson data from [avvai-backend/data/lessons](avvai-backend/data/lessons) via Axum routes defined in [avvai-backend/src/routes](avvai-backend/src/routes) and wired in [avvai-backend/src/main.rs](avvai-backend/src/main.rs).
- Frontend never calls the backend directly; it uses SvelteKit API proxy routes under [avvai-frontend/src/routes/api](avvai-frontend/src/routes/api) with `BACKEND_URL` (default `http://localhost:3001`). Example: [avvai-frontend/src/routes/api/lesson/[id]/+server.ts](avvai-frontend/src/routes/api/lesson/%5Bid%5D/+server.ts) proxies to `/lesson/get`.

## Data model conventions
- Lesson JSON shape is source-of-truth in Rust types in [avvai-backend/src/routes/lesson.rs](avvai-backend/src/routes/lesson.rs).
- Frontend mirrors these types in [avvai-frontend/src/lib/types/lesson.ts](avvai-frontend/src/lib/types/lesson.ts). Keep these in sync when adding fields or new `ContentSection`/`ExerciseContent` variants.
- Backend progress tracking is **in-memory** only in [avvai-backend/src/routes/progress.rs](avvai-backend/src/routes/progress.rs); don’t assume persistence.

## Frontend patterns
- Svelte 5 runes are used for state (`$state`, `$derived`), including global state in [avvai-frontend/src/lib/stores/dictionary.svelte.ts](avvai-frontend/src/lib/stores/dictionary.svelte.ts).
- Click-to-lookup flow: `ClickableText` in [avvai-frontend/src/lib/components/ClickableText.svelte](avvai-frontend/src/lib/components/ClickableText.svelte) calls `showDictionary()`; `DictionaryPopup` in [avvai-frontend/src/lib/components/DictionaryPopup.svelte](avvai-frontend/src/lib/components/DictionaryPopup.svelte) renders the global popup.
- Dictionary lookups go through the frontend service in [avvai-frontend/src/lib/services/dictionary.ts](avvai-frontend/src/lib/services/dictionary.ts), which calls `/api/dictionary`.

## Developer workflows
- Frontend/CMS: run Vite/SvelteKit scripts from [avvai-frontend/package.json](avvai-frontend/package.json) or [avvai-cms/package.json](avvai-cms/package.json) (`dev`, `build`, `preview`, `check`).
- Backend: Axum server listens on port 3001 (see [avvai-backend/src/main.rs](avvai-backend/src/main.rs)); keep it running when working on frontend API proxies.
- Content tooling: [avvai-backend/src/bin/extract_image.rs](avvai-backend/src/bin/extract_image.rs) is a PDF image extraction utility with hard-coded local paths.

## Integration points
- API proxy mappings (frontend → backend):
  - [avvai-frontend/src/routes/api/lessons/+server.ts](avvai-frontend/src/routes/api/lessons/+server.ts) → `/lesson/list`
  - [avvai-frontend/src/routes/api/lesson/[id]/+server.ts](avvai-frontend/src/routes/api/lesson/%5Bid%5D/+server.ts) → `/lesson/get?id=...`
  - [avvai-frontend/src/routes/api/dictionary/+server.ts](avvai-frontend/src/routes/api/dictionary/+server.ts) → `/dictionary/lookup?word=...`
  - [avvai-frontend/src/routes/api/progress/+server.ts](avvai-frontend/src/routes/api/progress/+server.ts) → `/progress/get` and `/progress/update`
