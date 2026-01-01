// Thinai theme data - ported from mockups/style-guide

export type Palette = {
  bg: string
  surface: string
  primary: string
  secondary: string
  accent: string
  muted: string
  border: string
  foreground: string
}

export const thinai = {
  marutham: {
    name: 'Marutham',
    tamil: 'மருதம்',
    time: 'Vaikarai (Dawn)',
    emotion: 'New lesson, fresh start',
    light: {
      bg: '#f0f7f4',
      surface: '#e2f0e9',
      primary: '#558b2f',
      secondary: '#00838f',
      accent: '#ef6c00',
      muted: '#cfd8dc',
      border: '#a5d6a7',
      foreground: '#1b261b',
    },
    dark: {
      bg: '#0e1410',
      surface: '#1a2620',
      primary: '#9ccc65',
      secondary: '#4dd0e1',
      accent: '#ffab91',
      muted: '#37474f',
      border: '#2e4034',
      foreground: '#e0e6e2',
    },
  },
  mullai: {
    name: 'Mullai',
    tamil: 'முல்லை',
    time: 'Maalai (Evening)',
    emotion: 'Practice, patience, steady rhythm',
    light: {
      bg: '#fdfcf5',
      surface: '#f2f0e4',
      primary: '#fb8c00',
      secondary: '#33691e',
      accent: '#8d6e63',
      muted: '#d7ccc8',
      border: '#dcedc8',
      foreground: '#26201a',
    },
    dark: {
      bg: '#14100a',
      surface: '#241d14',
      primary: '#ffb74d',
      secondary: '#689f38',
      accent: '#a1887f',
      muted: '#3e2723',
      border: '#4e342e',
      foreground: '#eaddcf',
    },
  },
  kurinji: {
    name: 'Kurinji',
    tamil: 'குறிஞ்சி',
    time: 'Yamam (Midnight)',
    emotion: 'Mastery, union, achievement',
    light: {
      bg: '#f4f4f8',
      surface: '#e6e6ee',
      primary: '#6c5ce7',
      secondary: '#7a7a88',
      accent: '#9898c8',
      muted: '#c8c8d4',
      border: '#a4a4b4',
      foreground: '#1a1c24',
    },
    dark: {
      bg: '#0a0a10',
      surface: '#18181e',
      primary: '#a29bfe',
      secondary: '#8a8a98',
      accent: '#a8b0d8',
      muted: '#2a2834',
      border: '#44425a',
      foreground: '#e6e4ea',
    },
  },
} as const

export type ThinaiId = keyof typeof thinai
export type Mode = 'light' | 'dark'

// Font stacks
export const fonts = {
  tamil: "'Catamaran', 'Noto Sans Tamil', sans-serif",
  body: "'Crimson Pro', Georgia, serif",
  mono: "'JetBrains Mono', monospace",
}
