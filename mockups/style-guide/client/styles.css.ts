import { globalStyle, style, keyframes } from '@vanilla-extract/css';
import { recipe } from '@vanilla-extract/recipes';
import { vars } from './theme.css';

// Keyframes
const fadeSlideUp = keyframes({
  from: { opacity: 0, transform: 'translateY(16px)' },
  to: { opacity: 1, transform: 'translateY(0)' },
});

const shimmer = keyframes({
  '0%': { transform: 'translateX(-100%)' },
  '100%': { transform: 'translateX(100%)' },
});

// Global styles - Palm Leaf Editorial feel
globalStyle('html', {
  scrollBehavior: 'smooth',
});

globalStyle('body', {
  fontFamily: "'Crimson Pro', Georgia, serif",
  fontSize: '1.125rem',
  lineHeight: 1.7,
  color: vars.foreground,
  background: vars.bg,
  WebkitFontSmoothing: 'antialiased',
});

// Noise texture overlay for organic feel
globalStyle('body::before', {
  content: '""',
  position: 'fixed',
  inset: 0,
  backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E")`,
  opacity: 0.03,
  pointerEvents: 'none',
  zIndex: 9999,
});

// Layout
export const layout = style({
  display: 'grid',
  gridTemplateColumns: '280px 1fr',
  minHeight: '100vh',
});

export const sidebar = style({
  background: vars.surface,
  borderRight: `1px solid ${vars.border}`,
  padding: '2rem 1.5rem',
  position: 'sticky',
  top: 0,
  height: '100vh',
  overflowY: 'auto',
});

export const sidebarTitle = style({
  fontFamily: "'Catamaran', 'Noto Sans Tamil', sans-serif",
  fontSize: '2rem',
  fontWeight: 800,
  color: vars.primary,
  marginBottom: '0.25rem',
  letterSpacing: '-0.02em',
});

export const sidebarSubtitle = style({
  fontSize: '0.875rem',
  color: vars.foreground,
  opacity: 0.6,
  marginBottom: '2.5rem',
  fontFamily: "'Crimson Pro', serif",
  fontStyle: 'italic',
});

export const navTitle = style({
  fontSize: '0.75rem',
  textTransform: 'uppercase',
  letterSpacing: '0.1em',
  color: vars.foreground,
  opacity: 0.5,
  marginBottom: '0.75rem',
  fontFamily: "'Catamaran', sans-serif",
  fontWeight: 600,
});

export const main = style({
  padding: '2.5rem 3rem',
  maxWidth: '1000px',
  animation: `${fadeSlideUp} 0.5s ease-out`,
});

export const section = style({
  marginBottom: '4rem',
});

export const sectionTitle = style({
  fontFamily: "'Catamaran', 'Noto Sans Tamil', sans-serif",
  fontSize: '1.5rem',
  fontWeight: 700,
  marginBottom: '0.5rem',
  paddingBottom: '0.75rem',
  borderBottom: `1px solid ${vars.border}`,
  display: 'flex',
  alignItems: 'center',
  gap: '0.75rem',
});

export const desc = style({
  color: vars.foreground,
  opacity: 0.7,
  marginBottom: '1.5rem',
  fontSize: '1rem',
  lineHeight: 1.6,
});

// Theme list
export const themeList = style({
  listStyle: 'none',
  marginBottom: '2rem',
});

globalStyle(`${themeList} li`, {
  marginBottom: '0.375rem',
});

// Theme row - recipe with active variant
export const themeRow = recipe({
  base: {
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem',
    padding: '0.625rem 0.75rem',
    borderRadius: '8px',
    background: 'transparent',
    border: '1px solid transparent',
    transition: 'all 0.2s cubic-bezier(0.16, 1, 0.3, 1)',
    cursor: 'pointer',
    ':hover': {
      background: vars.bg,
    },
  },
  variants: {
    active: {
      true: {
        background: vars.bg,
        borderColor: vars.primary,
        boxShadow: `0 2px 8px rgba(0, 0, 0, 0.06)`,
      },
      false: {},
    },
  },
  defaultVariants: {
    active: false,
  },
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
  fontFamily: "'Catamaran', sans-serif",
  fontSize: '0.95rem',
  fontWeight: 500,
  color: vars.foreground,
  textAlign: 'left',
});

export const themeDot = style({
  width: '12px',
  height: '12px',
  borderRadius: '50%',
  flexShrink: 0,
  boxShadow: '0 1px 3px rgba(0, 0, 0, 0.15)',
});

export const themeMeta = style({
  display: 'block',
  fontSize: '0.75rem',
  opacity: 0.6,
  fontFamily: "'Crimson Pro', serif",
  fontStyle: 'italic',
});

export const modeToggle = style({
  display: 'flex',
  gap: '2px',
  background: vars.muted,
  borderRadius: '6px',
  padding: '3px',
});

// Mode button - recipe with active variant
export const modeBtn = recipe({
  base: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: '28px',
    height: '28px',
    border: 'none',
    borderRadius: '4px',
    background: 'transparent',
    color: vars.onMuted,
    cursor: 'pointer',
    transition: 'all 0.15s ease-out',
  },
  variants: {
    active: {
      true: {
        background: vars.surface,
        color: vars.primary,
        boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
      },
      false: {},
    },
  },
  defaultVariants: {
    active: false,
  },
});

// Theme info - editorial card style
export const themeInfo = style({
  background: `linear-gradient(135deg, ${vars.surface}, ${vars.bg})`,
  border: `1px solid ${vars.border}`,
  borderRadius: '16px',
  padding: '1.5rem 2rem',
  marginBottom: '2.5rem',
  position: 'relative',
  overflow: 'hidden',
  '::before': {
    content: '""',
    position: 'absolute',
    top: 0,
    left: 0,
    width: '4px',
    height: '100%',
    background: `linear-gradient(to bottom, ${vars.primary}, ${vars.secondary})`,
    borderRadius: '16px 0 0 16px',
  },
});

export const themeInfoTitle = style({
  fontFamily: "'Catamaran', 'Noto Sans Tamil', sans-serif",
  fontSize: '1.75rem',
  fontWeight: 700,
  marginBottom: '0.25rem',
});

export const themeInfoMeta = style({
  fontSize: '1rem',
  color: vars.secondary,
  fontFamily: "'Crimson Pro', serif",
  fontStyle: 'italic',
});

// Swatches
export const swatches = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(140px, 1fr))',
  gap: '1rem',
});

export const swatch = style({
  borderRadius: '12px',
  overflow: 'hidden',
  border: `1px solid ${vars.border}`,
  transition: 'all 0.25s cubic-bezier(0.16, 1, 0.3, 1)',
  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.04)',
  ':hover': {
    transform: 'translateY(-4px)',
    boxShadow: '0 8px 24px rgba(0, 0, 0, 0.08)',
  },
});

export const swatchColor = style({
  height: '80px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
});

export const swatchLabel = style({
  fontFamily: "'JetBrains Mono', 'Space Mono', monospace",
  fontSize: '0.7rem',
  fontWeight: 500,
  padding: '4px 8px',
  background: 'rgba(255, 255, 255, 0.2)',
  backdropFilter: 'blur(4px)',
  borderRadius: '4px',
  border: '1px solid rgba(255, 255, 255, 0.15)',
  textTransform: 'uppercase',
  letterSpacing: '0.5px',
});

export const swatchInfo = style({
  padding: '0.875rem 1rem',
  background: vars.surface,
});

export const swatchName = style({
  fontFamily: "'Catamaran', sans-serif",
  fontWeight: 600,
  fontSize: '0.9rem',
});

export const swatchHex = style({
  fontFamily: "'JetBrains Mono', monospace",
  fontSize: '0.75rem',
  color: vars.foreground,
  opacity: 0.5,
});

// Components grid
export const componentsGrid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))',
  gap: '1.5rem',
  paddingBottom: '2rem',
});

export const componentSection = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '16px',
  padding: '1.75rem',
  position: 'relative',
  overflow: 'hidden',
  transition: 'all 0.25s cubic-bezier(0.16, 1, 0.3, 1)',
  boxShadow: '0 2px 12px rgba(0, 0, 0, 0.04)',
  '::before': {
    content: '""',
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    height: '4px',
    background: `linear-gradient(90deg, ${vars.primary}, ${vars.accent})`,
  },
  ':hover': {
    transform: 'translateY(-2px)',
    boxShadow: '0 8px 24px rgba(0, 0, 0, 0.08)',
  },
});

globalStyle(`${componentSection} h3`, {
  fontFamily: "'Catamaran', sans-serif",
  marginTop: 0,
  marginBottom: '1.25rem',
  fontSize: '1.1rem',
  fontWeight: 600,
  borderBottom: `1px solid ${vars.border}`,
  paddingBottom: '0.75rem',
});

// Button group
export const btnGroup = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '0.625rem',
  marginBottom: '1rem',
});

// Button - Palm Leaf style
export const btn = recipe({
  base: {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    gap: '0.5rem',
    padding: '0.625rem 1.25rem',
    borderRadius: '8px',
    fontSize: '0.95rem',
    fontWeight: 600,
    cursor: 'pointer',
    border: 'none',
    transition: 'all 0.15s cubic-bezier(0.16, 1, 0.3, 1)',
    fontFamily: "'Catamaran', sans-serif",
    ':active': {
      transform: 'scale(0.98)',
    },
    selectors: {
      '&:disabled': {
        opacity: 0.5,
        cursor: 'not-allowed',
      },
    },
  },
  variants: {
    color: {
      primary: {
        background: vars.primary,
        color: vars.onPrimary,
        boxShadow: '0 2px 8px rgba(85, 139, 47, 0.3)',
        ':hover': {
          boxShadow: '0 4px 16px rgba(85, 139, 47, 0.4)',
        },
      },
      secondary: {
        background: vars.secondary,
        color: vars.onSecondary,
        boxShadow: '0 2px 8px rgba(0, 131, 143, 0.25)',
        ':hover': {
          boxShadow: '0 4px 16px rgba(0, 131, 143, 0.35)',
        },
      },
      outline: {
        background: 'transparent',
        border: `1px solid ${vars.border}`,
        color: vars.foreground,
        selectors: {
          '&:hover:not(:disabled)': {
            borderColor: vars.primary,
            color: vars.primary,
            background: `color-mix(in srgb, ${vars.primary} 8%, transparent)`,
          },
        },
      },
      ghost: {
        background: 'transparent',
        color: vars.foreground,
        selectors: {
          '&:hover:not(:disabled)': {
            background: vars.muted,
          },
        },
      },
    },
  },
  defaultVariants: {
    color: 'primary',
  },
});

// Badge group
export const badgeGroup = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '0.5rem',
});

// Badge - recipe with color variants
export const badge = recipe({
  base: {
    padding: '0.375rem 0.75rem',
    borderRadius: '999px',
    fontSize: '0.8rem',
    fontWeight: 600,
    fontFamily: "'Catamaran', sans-serif",
    letterSpacing: '0.02em',
  },
  variants: {
    color: {
      primary: {
        background: `color-mix(in srgb, ${vars.primary} 15%, transparent)`,
        color: vars.primary,
      },
      secondary: {
        background: `color-mix(in srgb, ${vars.secondary} 15%, transparent)`,
        color: vars.secondary,
      },
      accent: {
        background: `color-mix(in srgb, ${vars.accent} 12%, transparent)`,
        color: vars.accent,
      },
      muted: {
        background: vars.muted,
        color: vars.onMuted,
      },
    },
  },
  defaultVariants: {
    color: 'primary',
  },
});

// Cards - editorial style with gradient border
export const card = style({
  background: vars.bg,
  border: `1px solid ${vars.border}`,
  borderRadius: '12px',
  padding: '1.25rem',
  marginBottom: '1rem',
  position: 'relative',
  transition: 'all 0.25s cubic-bezier(0.16, 1, 0.3, 1)',
  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.04)',
  ':hover': {
    transform: 'translateY(-2px)',
    boxShadow: '0 6px 20px rgba(0, 0, 0, 0.08)',
  },
});

export const cardAccent = style({
  '::before': {
    content: '""',
    position: 'absolute',
    top: 0,
    left: 0,
    width: '4px',
    height: '100%',
    background: `linear-gradient(to bottom, ${vars.primary}, ${vars.secondary})`,
    borderRadius: '12px 0 0 12px',
  },
});

export const cardTitle = style({
  fontFamily: "'Catamaran', sans-serif",
  fontWeight: 600,
  fontSize: '1.1rem',
  marginBottom: '0.5rem',
  display: 'block',
});

export const cardText = style({
  fontSize: '0.95rem',
  opacity: 0.8,
  lineHeight: 1.6,
});

// Inputs - refined style
export const formGroup = style({
  marginBottom: '1.25rem',
});

export const label = style({
  display: 'block',
  marginBottom: '0.375rem',
  fontSize: '0.9rem',
  fontWeight: 500,
  fontFamily: "'Catamaran', sans-serif",
});

export const input = style({
  width: '100%',
  padding: '0.625rem 0.875rem',
  background: vars.bg,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  color: vars.foreground,
  fontFamily: "'Crimson Pro', serif",
  fontSize: '1rem',
  transition: 'all 0.15s ease-out',
  ':focus': {
    outline: 'none',
    borderColor: vars.primary,
    boxShadow: `0 0 0 3px color-mix(in srgb, ${vars.primary} 15%, transparent)`,
  },
  ':disabled': {
    opacity: 0.5,
    cursor: 'not-allowed',
  },
});

// Tabs - editorial underline style
export const tabsRoot = style({});

export const tabsList = style({
  display: 'flex',
  gap: '0.25rem',
  borderBottom: `1px solid ${vars.border}`,
  marginBottom: '1.25rem',
});

// Tab trigger - recipe with active variant
export const tabsTrigger = recipe({
  base: {
    padding: '0.75rem 1.25rem',
    background: 'transparent',
    border: 'none',
    borderBottom: '2px solid transparent',
    cursor: 'pointer',
    fontFamily: "'Catamaran', sans-serif",
    fontSize: '0.95rem',
    fontWeight: 500,
    color: vars.foreground,
    opacity: 0.6,
    transition: 'all 0.2s cubic-bezier(0.16, 1, 0.3, 1)',
    marginBottom: '-1px',
    selectors: {
      '&:hover': {
        opacity: 1,
        color: vars.primary,
      },
    },
  },
  variants: {
    active: {
      true: {
        opacity: 1,
        borderBottomColor: vars.primary,
        color: vars.primary,
      },
      false: {},
    },
  },
  defaultVariants: {
    active: false,
  },
});

export const tabsContent = style({
  padding: '0.75rem 0',
  animation: `${fadeSlideUp} 0.3s ease-out`,
});
