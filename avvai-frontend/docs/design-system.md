# Avvai Frontend Design System

This design system codifies the Avvai UI language: warm manuscript tones, calm hierarchy, and tactile, ink-like emphasis. It is implemented as CSS tokens plus reusable UI patterns.

## 1) Design Principles
- Warm, analog palette rooted in palm-leaf manuscripts.
- Clear hierarchy with restrained emphasis (gold for highlights, maroon for action).
- Legibility-first Tamil typography with generous line-height.
- Soft, rounded surfaces and gentle shadows.

## 2) Tokens
Tokens live in `src/lib/styles/tokens.css`. Use semantic tokens in components; reserve raw palette tokens for the token file and themes.

### Core Colors (palette only — do not use directly in components)
- `--ink`, `--ink-soft`, `--stone`
- `--cream`, `--cream-mid`, `--cream-dark`
- `--red`, `--red-deep`, `--red-faint`
- `--gold`
- `--green`, `--green-faint`
- `--terracotta`, `--terracotta-faint`
- `--dark`

### Semantic Colors (required in components)
- Text: `--color-text`, `--color-text-muted`, `--color-text-subtle`, `--color-text-inverse`
- Backgrounds: `--color-bg`, `--color-bg-soft`, `--color-bg-pressed`, `--color-bg-inverse`
- Action: `--color-accent`, `--color-accent-strong`, `--color-accent-tint`, `--color-accent-wash`
- Secondary accent: `--color-accent-secondary`, `--color-accent-secondary-tint`
- Emphasis: `--color-highlight`
- Feedback: `--color-success`, `--color-success-tint`

### Typography
- Families: `--font-sans` (Catamaran), `--font-serif` (Cormorant Garamond + Tamil fallbacks), `--font-display` (Eczar + Tamil fallbacks)
- Sizes: `--font-size-0` through `--font-size-12` (includes micro and display sizes)
- Line-height: `--line-height-0` through `--line-height-2-2`, plus `--line-height-3` and the semantic alias `--line-height-body` for long-form body text
- Letter spacing: `--letter-tight`, `--letter-normal`, `--letter-wide`, plus `--letter-2/3/4/5/8/10/18`

### Spacing
`--space-0` (1px) → `--space-12` (80px) with mid‑steps (`--space-0-5`, `--space-1-5`, etc.)

### Radius
`--radius-0` → `--radius-5`, plus `--radius-pill`

### Borders
`--border-1`, `--border-strong`

### Component Dimensions
`--size-icon-btn`, `--size-touch-target`

### Shadows
Core: `--shadow-1`, `--shadow-2`, `--shadow-3`, `--shadow-inset`
Specialized: `--shadow-soft`, `--shadow-warm`, `--shadow-red`, `--shadow-red-deep`, `--shadow-deep`, `--shadow-elevated`, `--shadow-accent-secondary`, `--shadow-text-inverse`, `--shadow-drawer-*`

### Overlays
`--overlay-*` tokens for modals, scrims, and surface tints.

### Motion
`--duration-fast`, `--duration-medium`, `--duration-slow`, `--ease-standard`

## 3) Component Guidance
Use these patterns to keep UI consistent:

- **Buttons**: rounded (`--radius-2`), maroon as primary, use `--color-accent-tint` for hover backgrounds.
- **Cards / Panels**: `--color-bg`, `--border-1`, `--shadow-1` or `--shadow-2`.
- **Inputs**: `--border-strong`, `--radius-2`, focus on `--color-accent`.
- **Badges / Pills**: `--radius-pill`, small caps, `--letter-wide`.

## 4) When to Extend
Introduce new tokens only if a value appears in 3+ components or becomes a semantic need (e.g. “warning”, “info”).
Palette tokens should not be used directly in component styles.

## 5) Implementation Notes
- Base styles are set in `src/routes/+layout.svelte`.
- Tokens are global, so use them in component styles directly.
- Marketing or experimental pages can override locally, but should be refactored into tokens if reused.

## 6) Demo
See `src/routes/design-system/+page.svelte` for a living example of tokens and patterns.
