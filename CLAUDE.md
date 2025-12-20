# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Avvai is a Tamil language learning app with a thematic design system based on the Sangam-era Thinai (திணை) landscapes.

## Commands

```bash
# install dependencies
bun install

# run api server (elysia on port 3000)
bun run dev

# run frontend dev server (vite on port 5173)
bun run ui

# build frontend
bun run build

# regenerate CSS tokens from tokens.js
bun run build:tokens
```

## Architecture

**Backend**: Elysia + better-auth + bun:sqlite
- `src/index.ts` - server entry point
- `src/api/index.ts` - API routes under `/api` prefix

**Frontend**: SolidJS + TanStack Router + TanStack Query
- `index.html` - entry HTML
- `src/client/index.tsx` - SolidJS app (styleguide viewer)
- `src/client/tokens.css` - generated CSS variables (do not edit directly)

**Design Tokens**:
- `tokens.js` - source of truth for all theme colors (5 thinai × 2 modes = 10 themes)
- `build-tokens.js` - generates `src/client/tokens.css` from tokens.js
- Themes are applied via `data-theme` attribute on body

## Bun Preferences

Default to Bun instead of Node.js:
- `bun <file>` instead of `node` or `ts-node`
- `bun test` instead of jest/vitest
- `bun install` instead of npm/yarn/pnpm
- Bun automatically loads .env

Prefer Bun APIs:
- `Bun.serve()` for HTTP servers (supports WebSockets, routes)
- `bun:sqlite` for SQLite
- `Bun.file` over `node:fs` readFile/writeFile
