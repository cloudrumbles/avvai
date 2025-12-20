import { style, globalStyle } from '@vanilla-extract/css';
import { vars } from './theme.css';

// Layout
export const layout = style({
  display: 'grid',
  gridTemplateColumns: '240px 1fr',
  minHeight: '100vh',
});

export const sidebar = style({
  background: vars.surface,
  borderRight: `1px solid ${vars.border}`,
  padding: '1.5rem',
  position: 'sticky',
  top: 0,
  height: '100vh',
  overflowY: 'auto',
});

export const sidebarTitle = style({
  fontFamily: "'Eczar', serif",
  fontSize: '1.5rem',
  marginBottom: '0.25rem',
});

export const sidebarSubtitle = style({
  fontSize: '0.8rem',
  color: vars.foreground,
  opacity: 0.5,
  marginBottom: '2rem',
});

export const navTitle = style({
  fontSize: '0.7rem',
  textTransform: 'uppercase',
  letterSpacing: '1px',
  color: vars.foreground,
  opacity: 0.5,
  marginBottom: '0.5rem',
});

export const main = style({
  padding: '2rem 3rem',
  maxWidth: '900px',
});

export const section = style({
  marginBottom: '4rem',
});

export const sectionTitle = style({
  fontFamily: "'Eczar', serif",
  fontSize: '1.75rem',
  marginBottom: '0.5rem',
  paddingBottom: '0.5rem',
  borderBottom: `1px solid ${vars.border}`,
});

export const desc = style({
  color: vars.foreground,
  opacity: 0.6,
  marginBottom: '1.5rem',
  fontSize: '0.95rem',
});

// Theme list
export const themeList = style({
  listStyle: 'none',
  marginBottom: '2rem',
});

globalStyle(`${themeList} li`, {
  marginBottom: '0.25rem',
});

export const themeRow = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.5rem',
  padding: '0.5rem',
  borderRadius: '6px',
  background: vars.bg,
  border: '1px solid transparent',
  transition: 'border-color 0.15s',
  cursor: 'pointer',
});

export const themeRowActive = style({
  borderColor: vars.primary,
});

export const themeSelect = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.75rem',
  flex: 1,
  padding: '0.25rem',
  background: 'transparent',
  border: 'none',
  cursor: 'pointer',
  fontFamily: 'inherit',
  fontSize: '0.9rem',
  color: vars.foreground,
  textAlign: 'left',
});

export const themeDot = style({
  width: '10px',
  height: '10px',
  borderRadius: '50%',
  flexShrink: 0,
});

export const themeMeta = style({
  display: 'block',
  fontSize: '0.7rem',
  opacity: 0.6,
});

export const modeToggle = style({
  display: 'flex',
  gap: '2px',
  background: vars.muted,
  borderRadius: '4px',
  padding: '2px',
});

export const modeBtn = style({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  width: '26px',
  height: '26px',
  border: 'none',
  borderRadius: '3px',
  background: 'transparent',
  color: vars.onMuted,
  cursor: 'pointer',
  transition: 'background 0.15s, color 0.15s',
});

export const modeBtnActive = style({
  background: vars.surface,
  color: vars.primary,
});

// Theme info
export const themeInfo = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  padding: '1rem 1.25rem',
  marginBottom: '2rem',
});

export const themeInfoTitle = style({
  fontFamily: "'Eczar', serif",
  fontSize: '1.25rem',
});

export const themeInfoMeta = style({
  fontSize: '0.85rem',
  color: vars.secondary,
});

// Swatches
export const swatches = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(4, 1fr)',
  gap: '1rem',
});

export const swatch = style({
  borderRadius: '8px',
  overflow: 'hidden',
  border: `1px solid ${vars.border}`,
});

export const swatchColor = style({
  height: '80px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
});

export const swatchLabel = style({
  fontFamily: "'Space Mono', monospace",
  fontSize: '0.7rem',
  fontWeight: 600,
  padding: '4px 8px',
  background: 'rgba(255, 255, 255, 0.15)',
  backdropFilter: 'blur(4px)',
  borderRadius: '4px',
  border: '1px solid rgba(255, 255, 255, 0.1)',
  textTransform: 'uppercase',
  letterSpacing: '0.5px',
});

export const swatchInfo = style({
  padding: '0.75rem',
  background: vars.surface,
});

export const swatchName = style({
  fontWeight: 600,
  fontSize: '0.85rem',
});

export const swatchHex = style({
  fontFamily: "'Space Mono', monospace",
  fontSize: '0.75rem',
  color: vars.foreground,
  opacity: 0.6,
});

// Components grid
export const componentsGrid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))',
  gap: '2rem',
  paddingBottom: '2rem',
});

export const componentSection = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  padding: '1.5rem',
});

globalStyle(`${componentSection} h3`, {
  marginTop: 0,
  marginBottom: '1rem',
  fontSize: '1.1rem',
  borderBottom: `1px solid ${vars.border}`,
  paddingBottom: '0.5rem',
});

// Buttons
export const btnGroup = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '0.5rem',
  marginBottom: '1rem',
});

export const btn = style({
  padding: '0.5rem 1rem',
  borderRadius: '4px',
  fontSize: '0.9rem',
  fontWeight: 500,
  cursor: 'pointer',
  border: '1px solid transparent',
  transition: 'all 0.2s',
  fontFamily: 'inherit',
  selectors: {
    '&:disabled': {
      opacity: 0.5,
      cursor: 'not-allowed',
    },
  },
});

export const btnPrimary = style({
  background: vars.primary,
  color: vars.onPrimary,
});

export const btnSecondary = style({
  background: vars.secondary,
  color: vars.onSecondary,
});

export const btnOutline = style({
  background: 'transparent',
  borderColor: vars.border,
  color: vars.foreground,
  ':hover': {
    borderColor: vars.primary,
    color: vars.primary,
  },
});

export const btnGhost = style({
  background: 'transparent',
  color: vars.foreground,
  ':hover': {
    background: vars.muted,
  },
});

// Badges
export const badgeGroup = style({
  display: 'flex',
  gap: '0.5rem',
});

export const badge = style({
  padding: '0.25rem 0.5rem',
  borderRadius: '999px',
  fontSize: '0.75rem',
  fontWeight: 'bold',
});

export const badgePrimary = style({
  background: vars.primary,
  color: vars.onPrimary,
});

export const badgeSecondary = style({
  background: vars.secondary,
  color: vars.onSecondary,
});

export const badgeAccent = style({
  background: vars.accent,
  color: vars.onAccent,
});

export const badgeMuted = style({
  background: vars.muted,
  color: vars.onMuted,
});

// Cards
export const card = style({
  background: vars.bg,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  padding: '1rem',
  marginBottom: '1rem',
});

export const cardTitle = style({
  fontWeight: 'bold',
  marginBottom: '0.5rem',
  display: 'block',
});

export const cardText = style({
  fontSize: '0.9rem',
  opacity: 0.8,
  lineHeight: 1.4,
});

// Inputs
export const formGroup = style({
  marginBottom: '1rem',
});

export const label = style({
  display: 'block',
  marginBottom: '0.25rem',
  fontSize: '0.85rem',
  fontWeight: 500,
});

export const input = style({
  width: '100%',
  padding: '0.5rem',
  background: vars.bg,
  border: `1px solid ${vars.border}`,
  borderRadius: '4px',
  color: vars.foreground,
  fontFamily: 'inherit',
  ':focus': {
    outline: 'none',
    borderColor: vars.primary,
    boxShadow: `0 0 0 2px ${vars.muted}`,
  },
});

// Tabs
export const tabsRoot = style({});

export const tabsList = style({
  display: 'flex',
  borderBottom: `1px solid ${vars.border}`,
  marginBottom: '1rem',
});

export const tabsTrigger = style({
  padding: '0.5rem 1rem',
  background: 'transparent',
  border: 'none',
  borderBottom: '2px solid transparent',
  cursor: 'pointer',
  fontFamily: 'inherit',
  fontSize: '0.9rem',
  color: vars.foreground,
  opacity: 0.7,
  transition: 'all 0.15s',
  ':hover': {
    opacity: 1,
  },
});

export const tabsTriggerActive = style({
  opacity: 1,
  borderBottomColor: vars.primary,
  color: vars.primary,
});

export const tabsContent = style({
  padding: '0.5rem 0',
});
