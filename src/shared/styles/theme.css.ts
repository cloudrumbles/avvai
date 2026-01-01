import { createTheme, createThemeContract, globalStyle, style } from '@vanilla-extract/css'
import { thinai, fonts, type Palette, type ThinaiId, type Mode } from './theme'

// Theme contract
export const vars = createThemeContract({
  bg: null,
  surface: null,
  primary: null,
  secondary: null,
  accent: null,
  muted: null,
  border: null,
  foreground: null,
})

// Contrast color helper
const getContrast = (hex: string, light = '#ffffff', dark = '#1a1a1a') => {
  const r = parseInt(hex.slice(1, 3), 16)
  const g = parseInt(hex.slice(3, 5), 16)
  const b = parseInt(hex.slice(5, 7), 16)
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
  return luminance > 0.5 ? dark : light
}

// Generate theme class from palette
const makeTheme = (palette: Palette) => createTheme(vars, palette)

// Generate all theme classes
type ThemeKey = `${ThinaiId}-${Mode}`

export const themes: Record<ThemeKey, string> = Object.fromEntries(
  Object.entries(thinai).flatMap(([id, t]) => [
    [`${id}-light`, makeTheme(t.light)],
    [`${id}-dark`, makeTheme(t.dark)],
  ])
) as Record<ThemeKey, string>

// Global styles
globalStyle('*', {
  margin: 0,
  padding: 0,
  boxSizing: 'border-box',
})

globalStyle(':root', {
  vars: {
    '--font-tamil': fonts.tamil,
    '--font-body': fonts.body,
    '--font-mono': fonts.mono,
  },
})

globalStyle('html', {
  fontSize: '16px',
})

globalStyle('body', {
  fontFamily: fonts.body,
  background: vars.bg,
  color: vars.foreground,
  minHeight: '100vh',
  lineHeight: 1.6,
  transition: 'background 0.3s, color 0.3s',
})

// Utility classes
export const tamilText = style({
  fontFamily: fonts.tamil,
})

export const container = style({
  maxWidth: '800px',
  margin: '0 auto',
  padding: '2rem',
})

export const card = style({
  background: vars.surface,
  borderRadius: '12px',
  padding: '1.5rem',
  marginBottom: '1.5rem',
  border: `1px solid ${vars.border}`,
})
