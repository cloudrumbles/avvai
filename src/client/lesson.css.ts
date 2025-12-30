import { style, keyframes, globalStyle, createVar } from '@vanilla-extract/css';
import { vars } from './theme.css';

// Custom variables for lesson
const sidebarWidth = createVar();

// Animations
const fadeSlideIn = keyframes({
  '0%': { opacity: 0, transform: 'translateY(8px)' },
  '100%': { opacity: 1, transform: 'translateY(0)' },
});

const shimmer = keyframes({
  '0%': { backgroundPosition: '-200% 0' },
  '100%': { backgroundPosition: '200% 0' },
});

// Layout
export const lessonLayout = style({
  vars: { [sidebarWidth]: '300px' },
  display: 'grid',
  gridTemplateColumns: `${sidebarWidth} 1fr`,
  minHeight: '100vh',
  '@media': {
    '(max-width: 900px)': {
      gridTemplateColumns: '1fr',
    },
  },
});

// Sidebar
export const sidebar = style({
  background: vars.surface,
  borderRight: `1px solid ${vars.border}`,
  position: 'sticky',
  top: 0,
  height: '100vh',
  overflowY: 'auto',
  display: 'flex',
  flexDirection: 'column',
  '@media': {
    '(max-width: 900px)': {
      display: 'none',
    },
  },
});

export const sidebarHeader = style({
  padding: '1.5rem',
  borderBottom: `1px solid ${vars.border}`,
});

export const courseId = style({
  fontFamily: "'JetBrains Mono', 'IBM Plex Mono', monospace",
  fontSize: '0.65rem',
  letterSpacing: '0.5px',
  color: vars.primary,
  background: `${vars.primary}15`,
  padding: '0.2rem 0.6rem',
  borderRadius: '4px',
  display: 'inline-block',
  marginBottom: '0.6rem',
});

export const courseTitle = style({
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontSize: '1.1rem',
  fontWeight: 600,
  lineHeight: 1.4,
  marginBottom: '0.15rem',
});

export const courseSubtitle = style({
  fontFamily: "'Literata', 'Source Serif 4', Georgia, serif",
  fontSize: '0.85rem',
  color: vars.foreground,
  opacity: 0.6,
  fontStyle: 'italic',
});

export const navSections = style({
  flex: 1,
  padding: '1rem 0',
  overflowY: 'auto',
});

export const navGroupLabel = style({
  fontFamily: "'JetBrains Mono', 'IBM Plex Mono', monospace",
  fontSize: '0.6rem',
  textTransform: 'uppercase',
  letterSpacing: '1.5px',
  color: vars.foreground,
  opacity: 0.5,
  padding: '1rem 1.5rem 0.5rem',
});

export const navItem = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.5rem',
  padding: '0.65rem 1.5rem',
  color: vars.foreground,
  textDecoration: 'none',
  fontSize: '0.9rem',
  borderLeft: '2px solid transparent',
  cursor: 'pointer',
  transition: 'all 0.15s ease',
  ':hover': {
    background: `${vars.primary}08`,
  },
});

export const navItemActive = style({
  background: `${vars.primary}12`,
  borderLeftColor: vars.primary,
  color: vars.primary,
});

export const navItemCompleted = style({
  color: vars.foreground,
  opacity: 0.6,
});

export const navItemNum = style({
  fontFamily: "'JetBrains Mono', 'IBM Plex Mono', monospace",
  fontSize: '0.75rem',
  color: vars.foreground,
  opacity: 0.5,
  minWidth: '1.5rem',
});

export const checkMark = style({
  color: vars.primary,
  marginRight: '0.25rem',
  fontSize: '0.8rem',
});

// Main content area
export const lessonMain = style({
  padding: '0',
  maxWidth: '100%',
  animation: `${fadeSlideIn} 0.4s ease-out`,
  display: 'flex',
  flexDirection: 'column',
  position: 'relative',
});

export const pageContainer = style({
  padding: '2.5rem 4rem 2rem',
  maxWidth: '900px',
  margin: '0 auto',
  width: '100%',
  '@media': {
    '(max-width: 1100px)': {
      padding: '2rem 2.5rem',
    },
    '(max-width: 600px)': {
      padding: '1.5rem 1.25rem',
    },
  },
});

// Lesson header
export const lessonHeader = style({
  marginBottom: '2.5rem',
  paddingBottom: '1.75rem',
  borderBottom: `1px solid ${vars.border}`,
});

export const lessonMeta = style({
  display: 'flex',
  alignItems: 'center',
  gap: '1.25rem',
  marginBottom: '1.25rem',
});

export const lessonBadge = style({
  fontFamily: "'JetBrains Mono', monospace",
  fontSize: '0.75rem',
  fontWeight: 600,
  background: vars.primary,
  color: vars.onPrimary,
  padding: '0.35rem 0.85rem',
  borderRadius: '6px',
  letterSpacing: '0.5px',
});

export const pageIndicator = style({
  fontFamily: "'JetBrains Mono', monospace",
  fontSize: '0.75rem',
  opacity: 0.6,
});

export const progressBar = style({
  width: '100%',
  height: '4px',
  background: vars.border,
  position: 'sticky',
  top: 0,
  zIndex: 10,
});

export const progressFill = style({
  height: '100%',
  background: `linear-gradient(90deg, ${vars.primary}, ${vars.secondary})`,
  transition: 'width 0.3s ease',
  position: 'relative',
  selectors: {
    '&::after': {
      content: '""',
      position: 'absolute',
      inset: 0,
      background: `linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent)`,
      backgroundSize: '200% 100%',
      animation: `${shimmer} 2s ease infinite`,
    },
  },
});

export const lessonTitle = style({
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontSize: '2rem',
  fontWeight: 600,
  lineHeight: 1.35,
  marginBottom: '1rem',
  letterSpacing: '-0.01em',
});

export const pageSummary = style({
  fontFamily: "'Literata', 'Source Serif 4', Georgia, serif",
  fontSize: '1.1rem',
  lineHeight: 1.6,
  color: vars.foreground,
  opacity: 0.8,
  fontStyle: 'italic',
  maxWidth: '65ch',
});

// Sections
export const section = style({
  marginBottom: '2rem',
});

export const sectionCard = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '12px',
  overflow: 'hidden',
  transition: 'border-color 0.2s ease, box-shadow 0.2s ease',
  ':hover': {
    borderColor: `${vars.primary}40`,
  },
});

export const sectionHeader = style({
  display: 'flex',
  alignItems: 'center',
  gap: '1rem',
  padding: '1rem 1.5rem',
  cursor: 'pointer',
  background: `${vars.bg}80`,
  borderBottom: `1px solid transparent`,
  transition: 'background 0.15s ease',
  ':hover': {
    background: `${vars.primary}05`,
  },
});

export const sectionType = style({
  fontFamily: "'JetBrains Mono', monospace",
  fontSize: '0.65rem',
  textTransform: 'uppercase',
  letterSpacing: '0.5px',
  padding: '0.25rem 0.6rem',
  borderRadius: '4px',
  fontWeight: 600,
  flexShrink: 0,
});

export const sectionTypeContent = style({
  color: vars.secondary,
  background: `${vars.secondary}15`,
});

export const sectionTypeQuiz = style({
  color: vars.accent,
  background: `${vars.accent}15`,
});

export const sectionTypeVocab = style({
  color: vars.primary,
  background: `${vars.primary}15`,
});

export const sectionTitle = style({
  flex: 1,
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontSize: '1.1rem',
  fontWeight: 600,
  color: vars.foreground,
});

export const sectionToggle = style({
  color: vars.foreground,
  opacity: 0.4,
  fontSize: '0.75rem',
  transition: 'transform 0.25s ease',
});

export const sectionToggleOpen = style({
  transform: 'rotate(180deg)',
});

export const sectionBody = style({
  maxHeight: 0,
  opacity: 0,
  overflow: 'hidden',
  transition: 'max-height 0.35s ease, opacity 0.25s ease',
});

export const sectionBodyOpen = style({
  maxHeight: '4000px',
  opacity: 1,
  padding: '1.5rem',
  borderTop: `1px solid ${vars.border}`,
});

export const tamilBlock = style({
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontSize: '1.1rem',
  lineHeight: 2.2, // increased leading for better readability of Tamil
  textAlign: 'justify',
  color: vars.foreground,
  marginBottom: '1rem',
  ':last-child': {
    marginBottom: 0,
  },
});

export const term = style({
  color: vars.primary,
  fontWeight: 600,
  cursor: 'help',
  borderBottom: `1px dashed ${vars.primary}40`,
});

// Vocab Grid
export const vocabGrid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(240px, 1fr))',
  gap: '1rem',
  marginTop: '1rem',
});

export const vocabCard = style({
  background: vars.bg,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  padding: '1rem',
  display: 'flex',
  flexDirection: 'column',
  gap: '0.5rem',
  transition: 'transform 0.2s, box-shadow 0.2s',
  ':hover': {
    transform: 'translateY(-2px)',
    boxShadow: `0 4px 12px ${vars.primary}15`,
    borderColor: vars.primary,
  },
});

export const vocabTamil = style({
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontSize: '1.25rem',
  fontWeight: 600,
  color: vars.foreground,
});

export const vocabMeta = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '0.25rem',
});

export const vocabEnglish = style({
  fontFamily: "'Literata', serif",
  fontSize: '0.95rem',
  color: vars.foreground,
  opacity: 0.9,
});

export const vocabPhonetic = style({
  fontFamily: "'JetBrains Mono', monospace",
  fontSize: '0.8rem',
  color: vars.foreground,
  opacity: 0.6,
});

export const vocabExtra = style({
  fontSize: '0.8rem',
  marginTop: '0.25rem',
  paddingTop: '0.5rem',
  borderTop: `1px solid ${vars.border}`,
  opacity: 0.7,
  fontStyle: 'italic',
});

// Inline Vocab List (for text sections)
export const inlineVocabList = style({
  marginTop: '1.5rem',
  background: `${vars.bg}`,
  borderRadius: '8px',
  border: `1px solid ${vars.border}`,
  padding: '0.5rem 0',
});

export const inlineVocabItem = style({
  padding: '0.75rem 1.25rem',
  borderBottom: `1px solid ${vars.border}`,
  display: 'flex',
  alignItems: 'baseline',
  gap: '0.75rem',
  ':last-child': {
    borderBottom: 'none',
  },
});

export const inlineVocabTamil = style({
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontWeight: 600,
  fontSize: '1rem',
  color: vars.primary,
});

export const inlineVocabEnglish = style({
  fontSize: '0.95rem',
  opacity: 0.8,
});

// Navigation
export const pageNav = style({
  display: 'flex',
  justifyContent: 'space-between',
  marginTop: '4rem',
  padding: '2rem 4rem',
  borderTop: `1px solid ${vars.border}`,
  gap: '1rem',
  '@media': {
    '(max-width: 600px)': {
      padding: '2rem 1.5rem',
      flexDirection: 'column',
    },
  },
});

export const pageNavBtn = style({
  display: 'flex',
  alignItems: 'center',
  gap: '1rem',
  padding: '1rem 1.5rem',
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '10px',
  color: vars.foreground,
  textDecoration: 'none',
  cursor: 'pointer',
  transition: 'all 0.2s ease',
  minWidth: '200px',
  ':hover': {
    borderColor: vars.primary,
    background: `${vars.primary}08`,
    transform: 'translateY(-2px)',
  },
  ':disabled': {
    opacity: 0.5,
    cursor: 'not-allowed',
    transform: 'none',
  },
});

export const pageNavBtnNext = style({
  background: vars.primary,
  borderColor: vars.primary,
  color: vars.onPrimary,
  marginLeft: 'auto',
  ':hover': {
    background: vars.primary,
    transform: 'translateY(-2px)',
    boxShadow: `0 8px 24px ${vars.primary}30`,
  },
});

export const pageNavContent = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '0.25rem',
  textAlign: 'left',
});

export const pageNavLabel = style({
  fontFamily: "'JetBrains Mono', monospace",
  fontSize: '0.7rem',
  textTransform: 'uppercase',
  letterSpacing: '0.5px',
  opacity: 0.8,
});

export const pageNavTitle = style({
  fontFamily: "'Noto Sans Tamil', sans-serif",
  fontSize: '1rem',
  fontWeight: 600,
});

export const pageNavArrow = style({
  fontSize: '1.25rem',
  opacity: 0.8,
});

// Kolam pattern overlay
export const kolam = style({
  position: 'fixed',
  inset: 0,
  pointerEvents: 'none',
  opacity: 0.03,
  background: `radial-gradient(circle, ${vars.primary} 1px, transparent 1px)`,
  backgroundSize: '24px 24px',
  zIndex: -1,
});

// Custom scrollbar
globalStyle(`${sidebar}::-webkit-scrollbar`, {
  width: '6px',
});

globalStyle(`${sidebar}::-webkit-scrollbar-track`, {
  background: vars.bg,
});

globalStyle(`${sidebar}::-webkit-scrollbar-thumb`, {
  background: vars.border,
  borderRadius: '3px',
});

globalStyle(`${sidebar}::-webkit-scrollbar-thumb:hover`, {
  background: vars.muted,
});
