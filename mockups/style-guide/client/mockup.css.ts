import { style, globalStyle } from '@vanilla-extract/css';
import { vars } from './theme.css';

// ============================================
// TEXT STYLES
// ============================================

export const textMuted = style({
  color: vars.foreground,
  opacity: 0.6,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 60%)`,
      opacity: 1,
    },
  },
});

export const textPrimary = style({
  color: vars.primary,
});

export const textForeground = style({
  color: vars.foreground,
});

export const fontTamil = style({
  fontFamily: 'var(--font-tamil)',
});

export const fontMono = style({
  fontFamily: 'var(--font-mono)',
});

// ============================================
// LAYOUT
// ============================================

export const mockupContainer = style({
  minHeight: '80vh',
  display: 'flex',
  flexDirection: 'column',
  background: vars.bg,
  borderRadius: '8px',
  overflow: 'hidden',
  border: `1px solid ${vars.border}`,
});

export const flexGrow = style({
  flex: 1,
});

// ============================================
// HEADER
// ============================================

export const header = style({
  background: vars.surface,
  borderBottom: `1px solid ${vars.border}`,
  padding: '1rem 2rem',
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
});

export const logo = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.75rem',
  cursor: 'pointer',
});

export const logoMark = style({
  width: '36px',
  height: '36px',
  background: vars.primary,
  color: vars.onPrimary,
  borderRadius: '8px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  fontSize: '1.25rem',
  fontWeight: 'bold',
  fontFamily: 'var(--font-tamil)',
});

export const logoText = style({
  fontSize: '1.25rem',
  fontWeight: 'bold',
  color: vars.foreground,
  fontFamily: 'var(--font-tamil)',
});

export const nav = style({
  display: 'flex',
  gap: '1.5rem',
});

export const navLink = style({
  color: vars.foreground,
  cursor: 'pointer',
  fontFamily: 'var(--font-tamil)',
  textDecoration: 'none',
  ':hover': {
    color: vars.primary,
  },
});

export const navLinkDisabled = style({
  color: vars.foreground,
  opacity: 0.4,
  cursor: 'default',
  fontFamily: 'var(--font-tamil)',
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 40%)`,
      opacity: 1,
    },
  },
});

// ============================================
// FOOTER
// ============================================

export const footer = style({
  background: vars.surface,
  borderTop: `1px solid ${vars.border}`,
  padding: '2rem',
  textAlign: 'center',
  marginTop: 'auto',
});

// ============================================
// HERO
// ============================================

export const hero = style({
  padding: '4rem 2rem',
  background: vars.bg,
  position: 'relative',
});

export const heroContent = style({
  maxWidth: '600px',
});

export const heroGreeting = style({
  color: vars.primary,
  fontFamily: 'var(--font-tamil)',
  fontSize: '1.5rem',
  marginBottom: '0.5rem',
});

export const heroTitle = style({
  fontSize: '2.5rem',
  fontWeight: 'bold',
  color: vars.foreground,
  marginBottom: '1rem',
  lineHeight: 1.2,
});

export const heroSubtitle = style({
  color: vars.foreground,
  opacity: 0.6,
  fontSize: '1.1rem',
  lineHeight: 1.6,
  marginBottom: '2rem',
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 60%)`,
      opacity: 1,
    },
  },
});

export const buttonGroup = style({
  display: 'flex',
  gap: '1rem',
});

// ============================================
// BUTTONS
// ============================================

export const btnPrimary = style({
  background: vars.primary,
  color: vars.onPrimary,
  border: 'none',
  padding: '0.875rem 1.5rem',
  borderRadius: '8px',
  fontWeight: '600',
  cursor: 'pointer',
  display: 'inline-flex',
  alignItems: 'center',
  gap: '0.5rem',
  fontSize: '1rem',
  transition: 'opacity 0.2s',
  ':hover': {
    opacity: 0.9,
  },
});

export const btnSecondary = style({
  background: 'transparent',
  color: vars.foreground,
  border: `1px solid ${vars.border}`,
  padding: '0.875rem 1.5rem',
  borderRadius: '8px',
  fontWeight: '600',
  cursor: 'pointer',
  display: 'inline-flex',
  alignItems: 'center',
  gap: '0.5rem',
  fontSize: '1rem',
  transition: 'all 0.2s',
  ':hover': {
    borderColor: vars.primary,
    color: vars.primary,
  },
});

export const btnSmall = style({
  padding: '0.5rem 1rem',
  fontSize: '0.875rem',
});

// ============================================
// CARDS
// ============================================

export const card = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '12px',
  padding: '1.5rem',
});

export const cardClickable = style({
  cursor: 'pointer',
  transition: 'border-color 0.2s, box-shadow 0.2s',
  ':hover': {
    borderColor: vars.primary,
    boxShadow: `0 4px 12px rgb(from ${vars.primary} r g b / 10%)`,
  },
});

// Fallback for cardClickable hover
globalStyle(`${cardClickable}:hover`, {
  boxShadow: '0 4px 12px rgba(0,0,0,0.1)',
});

export const lessonCard = style({
  padding: '1.25rem',
});

export const lessonCardLabel = style({
  fontSize: '0.7rem',
  textTransform: 'uppercase',
  letterSpacing: '0.05em',
  color: vars.foreground,
  opacity: 0.5,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 50%)`,
      opacity: 1,
    },
  },
});

export const lessonCardTitle = style({
  fontSize: '1.1rem',
  fontWeight: '600',
  color: vars.foreground,
  margin: '0.25rem 0',
});

export const lessonCardTamil = style({
  fontFamily: 'var(--font-tamil)',
  fontSize: '1.25rem',
  color: vars.primary,
  marginBottom: '0.75rem',
});

export const lessonCardMeta = style({
  display: 'flex',
  gap: '1rem',
  fontSize: '0.75rem',
  color: vars.foreground,
  opacity: 0.5,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 50%)`,
      opacity: 1,
    },
  },
});

export const metaItem = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.25rem',
});

// ============================================
// SECTIONS
// ============================================

export const section = style({
  padding: '2rem',
});

export const sectionNoPadTop = style({
  padding: '0 2rem 2rem',
});

export const sectionTitle = style({
  fontSize: '1.25rem',
  fontWeight: '600',
  color: vars.foreground,
  marginBottom: '1rem',
});

export const grid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(220px, 1fr))',
  gap: '1rem',
});

export const gridNarrow = style({
  gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
  gap: '0.75rem',
});

// ============================================
// PROGRESS BAR
// ============================================

export const progressCard = style({
  maxWidth: '400px',
});

export const progressHeader = style({
  display: 'flex',
  justifyContent: 'space-between',
  marginBottom: '0.5rem',
  fontSize: '0.875rem',
  color: vars.foreground,
  opacity: 0.6,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 60%)`,
      opacity: 1,
    },
  },
});

export const progressTrack = style({
  height: '6px',
  background: vars.border,
  borderRadius: '3px',
  overflow: 'hidden',
});

export const progressFill = style({
  height: '100%',
  background: vars.primary,
  transition: 'width 0.3s',
});

// ============================================
// THINAI SECTION
// ============================================

export const thinaiCard = style({
  borderRadius: '12px',
  padding: '2rem',
});

export const thinaiTitle = style({
  fontSize: '1.5rem',
  color: vars.foreground,
  marginBottom: '1rem',
});

export const thinaiList = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '0.75rem',
});

export const thinaiItem = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.5rem',
});

export const thinaiDot = style({
  width: '10px',
  height: '10px',
  borderRadius: '50%',
});

export const thinaiName = style({
  fontFamily: 'var(--font-tamil)',
  color: vars.foreground,
});

export const thinaiNameEn = style({
  fontSize: '0.875rem',
  color: vars.foreground,
  opacity: 0.5,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 50%)`,
      opacity: 1,
    },
  },
});

// ============================================
// LESSON PAGE
// ============================================

export const lessonContainer = style({
  flex: 1,
  padding: '2rem',
  maxWidth: '700px',
  margin: '0 auto',
  width: '100%',
});

export const backLink = style({
  display: 'inline-flex',
  alignItems: 'center',
  gap: '0.5rem',
  color: vars.secondary,
  cursor: 'pointer',
  marginBottom: '1.5rem',
  fontSize: '0.875rem',
  textDecoration: 'none',
  ':hover': {
    textDecoration: 'underline',
  },
});

export const lessonHeader = style({
  marginBottom: '2rem',
});

export const badgeGroup = style({
  display: 'flex',
  gap: '0.5rem',
  marginBottom: '0.75rem',
});

export const badge = style({
  padding: '0.25rem 0.75rem',
  borderRadius: '999px',
  fontSize: '0.75rem',
  fontWeight: '600',
});

export const badgePrimary = style({
  background: vars.primary,
  color: vars.onPrimary,
});

export const badgeSecondary = style({
  background: vars.secondary,
  color: vars.onSecondary,
});

export const lessonTitle = style({
  fontFamily: 'var(--font-tamil)',
  fontSize: '2.5rem',
  color: vars.primary,
  marginBottom: '0.25rem',
});

export const lessonSubtitle = style({
  fontSize: '1.5rem',
  fontWeight: '400',
  color: vars.foreground,
});

// ============================================
// VOCAB CARDS
// ============================================

export const vocabCard = style({
  padding: '1rem',
});

export const vocabTamil = style({
  fontFamily: 'var(--font-tamil)',
  fontSize: '1.5rem',
  color: vars.primary,
});

export const vocabTranslit = style({
  fontSize: '0.8rem',
  fontFamily: 'var(--font-mono)',
  color: vars.foreground,
  opacity: 0.5,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 50%)`,
      opacity: 1,
    },
  },
});

export const vocabMeaning = style({
  fontSize: '0.875rem',
  color: vars.foreground,
});

// ============================================
// GRAMMAR
// ============================================

export const grammarBox = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  padding: '1.25rem',
});

export const grammarTitle = style({
  fontSize: '1rem',
  fontWeight: '600',
  color: vars.foreground,
  marginBottom: '0.75rem',
});

export const grammarText = style({
  color: vars.foreground,
  opacity: 0.6,
  lineHeight: 1.6,
  marginBottom: '1rem',
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 60%)`,
      opacity: 1,
    },
  },
});

export const grammarExample = style({
  background: vars.bg,
  padding: '0.75rem',
  borderRadius: '6px',
  marginBottom: '0.5rem',
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
  flexWrap: 'wrap',
  gap: '0.5rem',
});

export const grammarTamil = style({
  fontFamily: 'var(--font-tamil)',
  fontSize: '1.25rem',
  color: vars.foreground,
});

export const grammarLabel = style({
  fontSize: '0.875rem',
  color: vars.foreground,
  opacity: 0.5,
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 50%)`,
      opacity: 1,
    },
  },
});

// ============================================
// QUIZ
// ============================================

export const quizBox = style({
  background: vars.surface,
  border: `1px solid ${vars.border}`,
  borderRadius: '8px',
  padding: '1.5rem',
  textAlign: 'center',
});

export const quizPrompt = style({
  color: vars.foreground,
  opacity: 0.6,
  marginBottom: '0.5rem',
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 60%)`,
      opacity: 1,
    },
  },
});

export const quizWord = style({
  fontFamily: 'var(--font-tamil)',
  fontSize: '2.5rem',
  color: vars.primary,
  marginBottom: '1.5rem',
});

export const quizOptions = style({
  display: 'grid',
  gridTemplateColumns: '1fr 1fr',
  gap: '0.75rem',
});

export const quizOption = style({
  background: vars.bg,
  border: `1px solid ${vars.border}`,
  borderRadius: '6px',
  padding: '0.75rem',
  color: vars.foreground,
  cursor: 'pointer',
  fontSize: '1rem',
  transition: 'all 0.2s',
  ':hover': {
    borderColor: vars.primary,
    background: vars.surface,
  },
});

export const quizResult = style({
  textAlign: 'center',
});

export const quizEmoji = style({
  fontSize: '3rem',
  marginBottom: '0.5rem',
});

export const quizResultTitle = style({
  fontSize: '1.25rem',
  fontWeight: '600',
  color: vars.foreground,
});

export const quizResultText = style({
  color: vars.foreground,
  opacity: 0.6,
  marginBottom: '1rem',
  '@supports': {
    '(color: rgb(from white r g b))': {
      color: `rgb(from ${vars.foreground} r g b / 60%)`,
      opacity: 1,
    },
  },
});

// ============================================
// NAV BUTTONS
// ============================================

export const navButtons = style({
  display: 'flex',
  justifyContent: 'space-between',
  marginTop: '2rem',
});
