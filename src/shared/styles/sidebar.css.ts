import { style, globalStyle } from '@vanilla-extract/css'
import { vars } from './theme.css'

const fontBody = "'Catamaran', 'Noto Sans Tamil', sans-serif"
const fontDisplay = "'Tiro Tamil', 'Noto Serif Tamil', serif"

// Mobile breakpoint
const mobile = 'screen and (max-width: 768px)'

// ═══════════════════════════════════════════════════════════════
// MOBILE HEADER - Only visible on mobile
// ═══════════════════════════════════════════════════════════════

export const mobileHeader = style({
  display: 'none',
  '@media': {
    [mobile]: {
      display: 'flex',
      alignItems: 'center',
      gap: '0.75rem',
      padding: '0.75rem 1rem',
      paddingTop: 'max(0.75rem, env(safe-area-inset-top))',
      background: vars.surface,
      borderBottom: `1px solid ${vars.border}`,
      position: 'sticky',
      top: 0,
      zIndex: 50,
      boxShadow: '0 1px 3px rgba(0, 0, 0, 0.08)',
    },
  },
})

export const menuButton = style({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  width: '40px',
  height: '40px',
  border: 'none',
  background: 'transparent',
  cursor: 'pointer',
  borderRadius: '8px',
  color: vars.foreground,
  ':hover': {
    background: vars.bg,
  },
})

export const mobileTitle = style({
  flex: 1,
  fontSize: '1rem',
  fontWeight: 500,
  color: vars.foreground,
  fontFamily: fontDisplay,
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  whiteSpace: 'nowrap',
})

// ═══════════════════════════════════════════════════════════════
// SIDEBAR - Hidden on mobile, visible on desktop
// ═══════════════════════════════════════════════════════════════

export const sidebar = style({
  width: '220px',
  height: '100vh',
  position: 'fixed',
  left: 0,
  top: 0,
  background: vars.surface,
  borderRight: `1px solid ${vars.border}`,
  display: 'flex',
  flexDirection: 'column',
  overflowY: 'auto',
  overflowX: 'hidden',
  zIndex: 100,
  fontSize: '0.875rem',
  '@media': {
    [mobile]: {
      display: 'none',
    },
  },
})

// Mobile drawer (when open)
export const sidebarOpen = style({
  '@media': {
    [mobile]: {
      display: 'flex',
      width: '280px',
      boxShadow: '4px 0 24px rgba(0, 0, 0, 0.15)',
    },
  },
})

// Overlay behind drawer
export const overlay = style({
  display: 'none',
  '@media': {
    [mobile]: {
      display: 'block',
      position: 'fixed',
      inset: 0,
      background: 'rgba(0, 0, 0, 0.4)',
      zIndex: 99,
    },
  },
})

export const sidebarHeader = style({
  padding: '1rem',
  borderBottom: `1px solid ${vars.border}`,
  background: vars.bg,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
})

export const courseTitle = style({
  fontSize: '0.8rem',
  fontWeight: 600,
  color: vars.foreground,
  fontFamily: fontBody,
  margin: 0,
  lineHeight: 1.3,
})

export const closeButton = style({
  display: 'none',
  '@media': {
    [mobile]: {
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      width: '32px',
      height: '32px',
      border: 'none',
      background: 'transparent',
      cursor: 'pointer',
      borderRadius: '6px',
      color: vars.muted,
      ':hover': {
        background: vars.surface,
        color: vars.foreground,
      },
    },
  },
})

export const sidebarNav = style({
  flex: 1,
  padding: '0.5rem 0',
})

export const chapter = style({
  marginBottom: '0.25rem',
})

export const chapterTitle = style({
  fontSize: '0.65rem',
  fontWeight: 700,
  color: vars.muted,
  textTransform: 'uppercase',
  letterSpacing: '0.08em',
  padding: '0.5rem 1rem 0.25rem',
  margin: 0,
  fontFamily: fontBody,
})

export const sectionList = style({
  listStyle: 'none',
  margin: 0,
  padding: 0,
})

export const sectionItem = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.5rem',
  width: '100%',
  padding: '0.5rem 1rem',
  border: 'none',
  background: 'transparent',
  cursor: 'pointer',
  textAlign: 'left',
  transition: 'background 0.1s ease',
  fontFamily: fontDisplay,
  fontSize: '0.9rem',
  color: vars.foreground,
  lineHeight: 1.4,
  ':hover': {
    background: vars.bg,
  },
  '@media': {
    [mobile]: {
      padding: '0.75rem 1.25rem',
      fontSize: '1rem',
    },
  },
})

export const sectionActive = style({
  background: vars.bg,
  fontWeight: 600,
  color: vars.primary,
  borderLeft: `2px solid ${vars.primary}`,
  paddingLeft: 'calc(1rem - 2px)',
  '@media': {
    [mobile]: {
      paddingLeft: 'calc(1.25rem - 2px)',
    },
  },
})

export const sectionNumber = style({
  fontSize: '0.7rem',
  fontWeight: 600,
  color: vars.secondary,
  fontFamily: fontBody,
  minWidth: '1.25rem',
  '@media': {
    [mobile]: {
      fontSize: '0.8rem',
      minWidth: '1.5rem',
    },
  },
})

export const sectionTitle = style({
  flex: 1,
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  whiteSpace: 'nowrap',
})

// ═══════════════════════════════════════════════════════════════
// MAIN CONTENT - Full width on mobile
// ═══════════════════════════════════════════════════════════════

export const mainContent = style({
  marginLeft: '220px',
  minHeight: '100vh',
  '@media': {
    [mobile]: {
      marginLeft: 0,
      paddingBottom: '70px', // Space for bottom nav
    },
  },
})

// ═══════════════════════════════════════════════════════════════
// BOTTOM NAV - Mobile only, app-like navigation
// ═══════════════════════════════════════════════════════════════

export const bottomNav = style({
  display: 'none',
  '@media': {
    [mobile]: {
      display: 'flex',
      position: 'fixed',
      bottom: 0,
      left: 0,
      right: 0,
      background: vars.surface,
      borderTop: `1px solid ${vars.border}`,
      padding: '0.5rem 1rem',
      paddingBottom: 'max(0.5rem, env(safe-area-inset-bottom))',
      zIndex: 50,
      gap: '0.5rem',
      boxShadow: '0 -1px 3px rgba(0, 0, 0, 0.08)',
    },
  },
})

export const navButton = style({
  flex: 1,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  gap: '0.5rem',
  padding: '0.75rem',
  border: 'none',
  borderRadius: '8px',
  background: 'transparent',
  color: vars.foreground,
  fontFamily: fontBody,
  fontSize: '0.875rem',
  fontWeight: 500,
  cursor: 'pointer',
  transition: 'all 0.15s ease',
  ':hover': {
    background: vars.bg,
  },
  ':active': {
    transform: 'scale(0.98)',
  },
})

export const navButtonPrimary = style({
  background: vars.primary,
  color: '#fff',
  ':hover': {
    background: vars.primary,
    opacity: 0.9,
  },
})

export const navButtonDisabled = style({
  opacity: 0.4,
  cursor: 'default',
  ':hover': {
    background: 'transparent',
  },
  ':active': {
    transform: 'none',
  },
})

export const progressBar = style({
  display: 'none',
  '@media': {
    [mobile]: {
      display: 'block',
      position: 'fixed',
      top: 0,
      left: 0,
      right: 0,
      height: '3px',
      background: vars.border,
      zIndex: 51,
    },
  },
})

export const progressFill = style({
  height: '100%',
  background: vars.primary,
  transition: 'width 0.3s ease',
})
