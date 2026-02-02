# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Avvai is a Tamil language learning app built with SvelteKit 5 that teaches Tamil through classical Sangam-era literature. The frontend is a SvelteKit app using Svelte 5 runes syntax, with a Rust backend accessed via API proxy routes.

## Commands

```bash
bun run dev          # Start dev server
bun run build        # Production build
bun run preview      # Preview production build
bun run check        # Run svelte type checking
bun run check:watch  # Type checking in watch mode

oxlint .             # Lint
oxfmt .              # Format
```

The backend must be running at `http://localhost:3001` (or set `BACKEND_URL` env var).

## Architecture

### Routing Structure

- `/` - Marketing landing page explaining "thinai" (five landscapes of Tamil literature)
- `/home` - User dashboard with lesson/flashcard links
- `/lesson` - Main learning interface with `LessonList` → `LessonReader` flow
- `/flashcards` - Spaced repetition practice
- `/fonts` - Font gallery

### API Proxy Pattern

All API routes in `src/routes/api/` proxy to the Rust backend:
- `/api/lessons` → `GET /lesson/list`
- `/api/lesson/[id]` → `GET /lesson/get?id={id}`
- `/api/dictionary` → `GET /dictionary/lookup?word={word}`

### Core Interaction Pattern

The central learning interaction is **click-to-lookup**:
1. `ClickableText` splits text into clickable word tokens
2. Click triggers `showDictionary()` from global store
3. `DictionaryPopup` (rendered in root layout) shows definition

### Component Organization

```
src/lib/components/
├── lesson/           # LessonList, LessonReader
├── sections/         # ProseSection, PoetrySection, VocabularySection, ExercisesSection
├── ClickableText.svelte     # Word tokenizer for dictionary lookup
├── DictionaryPopup.svelte   # Global floating definition popup
├── Drawer.svelte            # Reusable slide-out panel
├── TableOfContents.svelte   # Section navigation with progress
└── ReadingSettingsMenu.svelte
```

### State Management

- **Local state**: Svelte 5 runes (`$state()`, `$derived()`)
- **Global store**: `stores/dictionary.svelte.ts` for dictionary popup state using getter pattern

### Type System

`src/lib/types/lesson.ts` mirrors the Rust backend exactly. Content uses discriminated unions:
- `ContentSection` = prose | poetry | vocabulary | exercises | media
- `ExerciseContent` = multiple_choice | fill_in_blank | short_answer | long_answer

### Styling

- CSS custom properties in `lib/styles/tokens.css` (palm-leaf manuscript inspired palette)
- Component-scoped `<style>` blocks, no CSS framework
- Mobile-first with 768px breakpoint
- Tamil fonts: configurable (Mukta Malar, Tiro Tamil, Noto, etc.)
- English fonts: Catamaran (UI), Cormorant Garamond (landing)

## Out of Scope

- **Accessibility (a11y)**: ARIA attributes, screen reader support, and other accessibility features are not a priority for this project.
