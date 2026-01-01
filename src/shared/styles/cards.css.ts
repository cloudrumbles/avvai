import { style, globalStyle, keyframes } from '@vanilla-extract/css'
import { vars } from './theme.css'

// ═══════════════════════════════════════════════════════════════
// TYPOGRAPHY - Optimized for Tamil readability
// ═══════════════════════════════════════════════════════════════

const fontDisplay = "'Tiro Tamil', 'Noto Serif Tamil', serif"
const fontBody = "'Catamaran', 'Noto Sans Tamil', sans-serif"

// Mobile breakpoint
const mobile = 'screen and (max-width: 768px)'

// ═══════════════════════════════════════════════════════════════
// LESSON CONTAINER
// ═══════════════════════════════════════════════════════════════

export const lesson = style({
  maxWidth: '620px', // Optimal line length for reading
  margin: '0 auto',
  padding: '2.5rem 2rem 5rem',
  fontFamily: fontBody,
  color: vars.foreground,
  '@media': {
    [mobile]: {
      padding: '1.5rem 1.25rem 4rem',
    },
  },
})

export const lessonHeader = style({
  marginBottom: '2.5rem',
  textAlign: 'center',
})

export const lessonChapter = style({
  fontSize: '0.8rem',
  color: vars.secondary,
  textTransform: 'uppercase',
  letterSpacing: '0.12em',
  fontWeight: 600,
  marginBottom: '0.5rem',
})

export const lessonTitle = style({
  fontSize: '2rem',
  fontWeight: 400,
  color: vars.foreground,
  lineHeight: 1.4,
  fontFamily: fontDisplay,
  margin: 0,
})

export const lessonCards = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '2.5rem',
})

// ═══════════════════════════════════════════════════════════════
// SHARED
// ═══════════════════════════════════════════════════════════════

export const cardBase = style({})

export const cardTitle = style({
  fontSize: '0.75rem',
  fontWeight: 600,
  color: vars.secondary,
  marginBottom: '1rem',
  fontFamily: fontBody,
  textTransform: 'uppercase',
  letterSpacing: '0.1em',
})

// ═══════════════════════════════════════════════════════════════
// READING - Comfortable prose
// ═══════════════════════════════════════════════════════════════

export const reading = style({
  fontSize: '1.125rem',
  lineHeight: 2.1,
  color: vars.foreground,
  fontFamily: fontBody,
  fontWeight: 400,
  letterSpacing: '0.01em',
  wordSpacing: '0.05em',
  textAlign: 'justify',
  textJustify: 'inter-word',
  hyphens: 'auto',
  '@media': {
    [mobile]: {
      fontSize: '1.05rem',
      lineHeight: 2.3,
      textAlign: 'left',
      fontWeight: 450,
      wordSpacing: '0.08em',
    },
  },
})

export const readingSource = style({
  marginTop: '1rem',
  fontSize: '0.875rem',
  color: vars.muted,
  fontStyle: 'italic',
})

// ═══════════════════════════════════════════════════════════════
// POETRY - Celebrated verses
// ═══════════════════════════════════════════════════════════════

export const poetry = style({
  padding: '2rem',
  background: vars.surface,
  borderRadius: '12px',
  textAlign: 'center',
  '@media': {
    [mobile]: {
      padding: '1.25rem 1rem',
      borderRadius: '10px',
    },
  },
})

export const poetryLine = style({
  fontSize: '1.25rem',
  lineHeight: 2.2,
  fontFamily: fontDisplay,
  color: vars.foreground,
  fontWeight: 400,
  '@media': {
    [mobile]: {
      fontSize: '1.15rem',
      lineHeight: 2.1,
    },
  },
})

export const poetryTranslation = style({
  marginTop: '1.25rem',
  paddingTop: '1.25rem',
  borderTop: `1px solid ${vars.border}`,
  fontSize: '0.95rem',
  color: vars.secondary,
  fontStyle: 'italic',
  lineHeight: 1.7,
})

export const poetrySource = style({
  marginTop: '1rem',
  fontSize: '0.8rem',
  color: vars.muted,
  letterSpacing: '0.02em',
})

// ═══════════════════════════════════════════════════════════════
// VOCAB - Quick reference
// ═══════════════════════════════════════════════════════════════

export const vocab = style({
  padding: '1.25rem 1.5rem',
  background: vars.surface,
  borderRadius: '10px',
})

export const vocabTitle = style({
  fontSize: '0.7rem',
  fontWeight: 600,
  color: vars.muted,
  textTransform: 'uppercase',
  letterSpacing: '0.1em',
  marginBottom: '1rem',
  fontFamily: fontBody,
})

export const vocabList = style({
  listStyle: 'none',
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
  gap: '0.75rem 2.5rem',
})

export const vocabItem = style({
  display: 'flex',
  alignItems: 'baseline',
  gap: '0.5rem',
  fontSize: '1rem',
  lineHeight: 1.5,
})

export const vocabTerm = style({
  fontWeight: 600,
  color: vars.primary,
  fontFamily: fontDisplay,
  fontSize: '1.05rem',
})

export const vocabMeaning = style({
  color: vars.foreground,
  fontFamily: fontBody,
  '::before': {
    content: '"—"',
    marginRight: '0.375rem',
    color: vars.muted,
  },
})

// ═══════════════════════════════════════════════════════════════
// DIALOGUE - Conversational flow
// ═══════════════════════════════════════════════════════════════

export const dialogue = style({
  padding: '1.25rem 1.5rem',
  background: vars.surface,
  borderRadius: '10px',
  '@media': {
    [mobile]: {
      padding: '1rem',
      borderRadius: '8px',
    },
  },
})

export const dialogueContext = style({
  fontSize: '0.9rem',
  color: vars.secondary,
  marginBottom: '1rem',
  fontStyle: 'italic',
  lineHeight: 1.6,
  '@media': {
    [mobile]: {
      fontSize: '0.85rem',
      padding: '0.75rem',
      background: vars.bg,
      borderRadius: '6px',
      marginBottom: '1rem',
    },
  },
})

export const dialogueLines = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '0.75rem',
  '@media': {
    [mobile]: {
      gap: '1rem',
    },
  },
})

export const dialogueLine = style({
  display: 'flex',
  gap: '0.625rem',
  alignItems: 'baseline',
  fontSize: '1.05rem',
  lineHeight: 1.7,
  '@media': {
    [mobile]: {
      flexDirection: 'column',
      gap: '0.25rem',
      fontSize: '1rem',
      lineHeight: 1.8,
    },
  },
})

export const dialogueSpeaker = style({
  fontWeight: 600,
  color: vars.primary,
  fontFamily: fontDisplay,
  flexShrink: 0,
  '::after': {
    content: '":"',
  },
  '@media': {
    [mobile]: {
      fontSize: '0.9rem',
      '::after': {
        content: 'none',
      },
    },
  },
})

export const dialogueText = style({
  color: vars.foreground,
  fontFamily: fontBody,
  '@media': {
    [mobile]: {
      paddingLeft: '0.75rem',
      borderLeft: `2px solid ${vars.border}`,
    },
  },
})

// ═══════════════════════════════════════════════════════════════
// EXERCISES - Clear interactive sections
// ═══════════════════════════════════════════════════════════════

const exerciseBase = style({
  padding: '1.5rem',
  background: vars.bg,
  border: `1.5px solid ${vars.border}`,
  borderRadius: '10px',
})

// ═══════════════════════════════════════════════════════════════
// MCQ - Clear options
// ═══════════════════════════════════════════════════════════════

export const mcq = style([exerciseBase, {}])

export const mcqQuestion = style({
  fontSize: '1.1rem',
  marginBottom: '1.25rem',
  lineHeight: 1.7,
  color: vars.foreground,
  fontFamily: fontBody,
  fontWeight: 500,
})

export const mcqOptions = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '0.625rem',
})

export const mcqOption = style({
  display: 'flex',
  alignItems: 'center',
  gap: '0.875rem',
  padding: '0.875rem 1.125rem',
  border: `1.5px solid ${vars.border}`,
  borderRadius: '8px',
  background: vars.surface,
  cursor: 'pointer',
  transition: 'all 0.15s ease',
  fontSize: '1rem',
  fontFamily: fontBody,
  color: vars.foreground,
  textAlign: 'left',
  ':hover': {
    borderColor: vars.primary,
    background: vars.bg,
  },
})

globalStyle(`${mcqOption}[data-answered]`, {
  cursor: 'default',
})

globalStyle(`${mcqOption}[data-answered]:hover`, {
  background: vars.surface,
  borderColor: vars.border,
})

export const mcqCorrect = style({
  borderColor: `${vars.primary} !important`,
  background: `${vars.surface} !important`,
})

globalStyle(`${mcqCorrect}::before`, {
  content: '"✓"',
  color: vars.primary,
  fontWeight: 700,
})

export const mcqIncorrect = style({
  borderColor: `${vars.muted} !important`,
  color: `${vars.muted} !important`,
  opacity: 0.6,
})

export const mcqLabel = style({
  fontSize: '0.9rem',
  fontWeight: 600,
  color: vars.secondary,
  fontFamily: fontBody,
  minWidth: '1.5rem',
})

// ═══════════════════════════════════════════════════════════════
// FILL-IN-BLANK - Sentence completion
// ═══════════════════════════════════════════════════════════════

export const fillBlank = style([exerciseBase, {}])

export const fillBlankSentence = style({
  fontSize: '1.1rem',
  lineHeight: 2.2,
  marginBottom: '1.25rem',
  color: vars.foreground,
  fontFamily: fontBody,
})

export const fillBlankBlank = style({
  display: 'inline-block',
  minWidth: '110px',
  borderBottom: `2px dashed ${vars.primary}`,
  padding: '0.125rem 0.625rem',
  margin: '0 0.25rem',
  textAlign: 'center',
  verticalAlign: 'baseline',
})

export const fillBlankFilled = style({
  color: vars.primary,
  fontWeight: 600,
  fontFamily: fontDisplay,
  borderBottomStyle: 'solid',
})

export const fillBlankWordBank = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '0.625rem',
  paddingTop: '1rem',
  borderTop: `1px solid ${vars.border}`,
})

export const fillBlankWord = style({
  padding: '0.625rem 1.125rem',
  border: `1.5px solid ${vars.border}`,
  borderRadius: '8px',
  background: vars.surface,
  cursor: 'pointer',
  transition: 'all 0.15s ease',
  fontSize: '0.95rem',
  fontFamily: fontBody,
  color: vars.foreground,
  ':hover': {
    borderColor: vars.primary,
    background: vars.bg,
  },
})

export const fillBlankUsed = style({
  opacity: 0.35,
  cursor: 'default',
  ':hover': {
    borderColor: vars.border,
    background: vars.surface,
  },
})

const pulse = keyframes({
  '0%, 100%': { opacity: 1 },
  '50%': { opacity: 0.5 },
})

export const fillBlankCorrect = style({
  background: `${vars.surface} !important`,
  borderColor: `${vars.primary} !important`,
})

export const fillBlankIncorrect = style({
  borderColor: `${vars.accent} !important`,
  animation: `${pulse} 0.3s ease`,
})

// ═══════════════════════════════════════════════════════════════
// SHORT ANSWER - Reflection prompt
// ═══════════════════════════════════════════════════════════════

export const shortAnswer = style([exerciseBase, {}])

export const shortAnswerQuestion = style({
  fontSize: '1.1rem',
  lineHeight: 1.7,
  color: vars.foreground,
  fontFamily: fontBody,
})

export const shortAnswerNote = style({
  display: 'none',
})
