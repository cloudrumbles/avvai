import { createTheme, createThemeContract, globalStyle } from '@vanilla-extract/css';
import { fontStacks, thinai, type Palette, type ThemeKey } from './themeData';

// Theme contract - flat structure for simplicity
export const vars = createThemeContract({
  bg: null,
  surface: null,
  primary: null,
  secondary: null,
  accent: null,
  muted: null,
  border: null,
  foreground: null,
  onPrimary: null,
  onSecondary: null,
  onAccent: null,
  onMuted: null,
});

// Contrast color helper (inline to avoid function export issues)
function getContrastColor(hex: string, light = '#ffffff', dark = '#1a1a1a') {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  return luminance > 0.5 ? dark : light;
}

// Create theme from palette
function makeTheme(palette: Palette) {
  return createTheme(vars, {
    ...palette,
    onPrimary: getContrastColor(palette.primary),
    onSecondary: getContrastColor(palette.secondary),
    onAccent: getContrastColor(palette.accent),
    onMuted: getContrastColor(palette.muted),
  });
}

// Generate all 10 theme classes
export const themes: Record<ThemeKey, string> = Object.fromEntries(
  Object.entries(thinai).flatMap(([id, t]) => [
    [`${id}-light`, makeTheme(t.light)],
    [`${id}-dark`, makeTheme(t.dark)],
  ])
) as Record<ThemeKey, string>;

// Global styles
globalStyle('*', {
  margin: 0,
  padding: 0,
  boxSizing: 'border-box',
});

globalStyle(':root', {
  vars: {
    '--font-tamil': fontStacks.tamil,
    '--font-body': fontStacks.body,
    '--font-mono': fontStacks.mono,
  },
});

globalStyle('body', {
  fontFamily: fontStacks.body,
  background: vars.bg,
  color: vars.foreground,
  minHeight: '100vh',
  transition: 'background 0.3s, color 0.3s',
});
