// Theme data and helpers - separate from CSS to allow function exports

// Contrast color helper
function getContrastColor(hex: string, light = '#ffffff', dark = '#1a1a1a') {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  return luminance > 0.5 ? dark : light;
}

// Palette type
export type Palette = {
  bg: string;
  surface: string;
  primary: string;
  secondary: string;
  accent: string;
  muted: string;
  border: string;
  foreground: string;
};

// Single source of truth for all theme data
export const thinai = {
  marutham: {
    name: 'Marutham',
    tamil: 'மருதம்',
    time: 'Vaikarai (Dawn)',
    emotion: 'New lesson, fresh start',
    lightLandscape: 'Paddy fields at dawn',
    darkLandscape: 'Paddy fields before first light',
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
  palai: {
    name: 'Palai',
    tamil: 'பாலை',
    time: 'Nanpakal (Midday)',
    emotion: 'Challenge, struggle, endurance',
    lightLandscape: 'Desert at harsh midday',
    darkLandscape: 'Desert under starless sky',
    light: {
      bg: '#fffbf0',
      surface: '#f0e6d2',
      primary: '#EFD79B',
      secondary: '#a69b8d',
      accent: '#7ca3c4',
      muted: '#d4cfc5',
      border: '#a8a093',
      foreground: '#2a2218',
    },
    dark: {
      bg: '#12100c',
      surface: '#221e18',
      primary: '#F5E6D3',
      secondary: '#9aaa8a',
      accent: '#8ab4d4',
      muted: '#3a3630',
      border: '#4a4438',
      foreground: '#e8e2d8',
    },
  },
  neithal: {
    name: 'Neithal',
    tamil: 'நெய்தல்',
    time: 'Erpadu (Sunset)',
    emotion: 'Review, longing, nostalgia',
    lightLandscape: 'Seashore at sunset',
    darkLandscape: 'Seashore after sunset',
    light: {
      bg: '#f0f7fa',
      surface: '#e0e8e9',
      primary: '#8d76a0',
      secondary: '#4a8b9f',
      accent: '#d48a9a',
      muted: '#c8d6d8',
      border: '#96a1a9',
      foreground: '#1a262c',
    },
    dark: {
      bg: '#0e1012',
      surface: '#1a1e22',
      primary: '#74b9ff',
      secondary: '#7a9aa8',
      accent: '#e4a4b4',
      muted: '#2e3234',
      border: '#4a4e52',
      foreground: '#e6e2dc',
    },
  },
  mullai: {
    name: 'Mullai',
    tamil: 'முல்லை',
    time: 'Maalai (Evening)',
    emotion: 'Practice, patience, steady rhythm',
    lightLandscape: 'Forest in golden hour',
    darkLandscape: 'Forest at dusk',
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
    lightLandscape: 'Mountains in soft light',
    darkLandscape: 'Mountains at midnight',
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
} as const;

export type ThinaiId = keyof typeof thinai;
export type Mode = 'light' | 'dark';
export type ThemeKey = `${ThinaiId}-${Mode}`;

// Get theme info for UI - derives everything from thinai
export function getThemeInfo(key: ThemeKey) {
  const [id, mode] = key.split('-') as [ThinaiId, Mode];
  const t = thinai[id];
  const palette = t[mode];

  return {
    id,
    mode,
    name: t.name,
    tamil: t.tamil,
    time: t.time,
    emotion: t.emotion,
    landscape: mode === 'light' ? t.lightLandscape : t.darkLandscape,
    palette,
    getContrast: (role: keyof Palette) => getContrastColor(palette[role]),
  };
}

// Simplified list for sidebar/nav
export const thinaiList = (Object.keys(thinai) as ThinaiId[]).map((id) => {
  const t = thinai[id];
  return {
    id,
    name: t.name,
    tamil: t.tamil,
    color: t.light.primary,
    emotion: t.emotion.split(',')[0].trim().toLowerCase(),
  };
});

// Color role descriptions for documentation
export const roleDescriptions: Record<keyof Palette, string> = {
  bg: 'page background',
  surface: 'cards, elevated containers',
  primary: 'main actions (buttons, links)',
  secondary: 'supporting actions',
  accent: 'highlights, alerts, badges',
  muted: 'disabled, subtle elements',
  border: 'dividers, outlines',
  foreground: 'main text on bg/surface',
};

// Tamil fonts available in the showroom
export const tamilFonts = [
  'Anek Tamil',
  'Arima',
  'Baloo Thambi 2',
  'Catamaran',
  'Hind Madurai',
  'Kavivanar',
  'Meera Inimai',
  'Mukta Malar',
  'Noto Sans Tamil',
  'Noto Serif Tamil',
  'Pavanam',
  'Tiro Tamil',
] as const;

export type TamilFont = (typeof tamilFonts)[number];

// Font stacks for default styling
export const fontStacks = {
  tamil: "'Catamaran', 'Noto Sans Tamil', sans-serif",
  body: "'Crimson Pro', Georgia, serif",
  mono: "'JetBrains Mono', monospace",
};
