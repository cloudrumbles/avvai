/**
 * AVVAI DESIGN TOKENS
 * Single source of truth for all theme colors.
 * Run `node build-tokens.js` to generate CSS.
 *
 * 5 thinnai x 2 modes = 10 themes
 */

// Auto-calculate text color based on background luminance
function getContrastColor(hex, lightColor = '#ffffff', darkColor = '#1a1a1a') {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
    return luminance > 0.5 ? darkColor : lightColor;
}

export const thinnai = {
    // ═══════════════════════════════════════════════════════════════
    // MARUTHAM — Paddy fields, dawn
    // ═══════════════════════════════════════════════════════════════
    marutham: {
        name: 'Marutham',
        tamil: 'மருதம்',
        time: 'Vaikarai (Dawn)',
        emotion: 'New lesson, fresh start',

        light: {
            landscape: 'Paddy fields at dawn',
            colors: {
                bg:        '#f4f6f0',  // morning mist over fields
                surface:   '#e8ebe2',  // pale dawn sky
                primary:   '#c8c864',  // marutham flower yellow
                secondary: '#5c8a42',  // young paddy
                accent:    '#d4a032',  // morning sun on grain
                muted:     '#c8cdc2',  // dew on leaves
                border:    '#a4ab98',  // field edge
                foreground:'#1c2418',  // dark soil
            }
        },
        dark: {
            landscape: 'Paddy fields before first light',
            colors: {
                bg:        '#0c100c',  // rich dark soil
                surface:   '#1a211a',  // shadowed paddy field
                primary:   '#dcdc8c',  // marutham flower glowing
                secondary: '#6a9c4a',  // paddy in moonlight
                accent:    '#d4a83c',  // first gold of dawn
                muted:     '#3a3830',  // wet dark earth
                border:    '#4a4a3c',  // bark in shadow
                foreground:'#e8e4da',  // pre-dawn mist
            }
        }
    },

    // ═══════════════════════════════════════════════════════════════
    // PALAI — Desert, midday
    // ═══════════════════════════════════════════════════════════════
    palai: {
        name: 'Palai',
        tamil: 'பாலை',
        time: 'Nanpakal (Midday)',
        emotion: 'Challenge, struggle, endurance',

        light: {
            landscape: 'Desert at harsh midday',
            colors: {
                bg:        '#fffbf0',  // harsh sunlight
                surface:   '#f0e6d2',  // parched earth
                primary:   '#EFD79B',  // palai flower
                secondary: '#a69b8d',  // dry dead wood
                accent:    '#7ca3c4',  // hazy sky
                muted:     '#d4cfc5',  // dust cloud
                border:    '#a8a093',  // cracked clay
                foreground:'#2a2218',  // burned shadow
            }
        },
        dark: {
            landscape: 'Desert under starless sky',
            colors: {
                bg:        '#12100c',  // cold desert night
                surface:   '#221e18',  // dark dunes
                primary:   '#F5E6D3',  // palai flower light
                secondary: '#9aaa8a',  // scrub in moonlight
                accent:    '#8ab4d4',  // clear night sky
                muted:     '#3a3630',  // cold sand
                border:    '#4a4438',  // dry riverbed
                foreground:'#e8e2d8',  // starlight on sand
            }
        }
    },

    // ═══════════════════════════════════════════════════════════════
    // NEITHAL — Seashore, sunset
    // ═══════════════════════════════════════════════════════════════
    neithal: {
        name: 'Neithal',
        tamil: 'நெய்தல்',
        time: 'Erpadu (Sunset)',
        emotion: 'Review, longing, nostalgia',

        light: {
            landscape: 'Seashore at sunset',
            colors: {
                bg:        '#f0f7fa',  // sea mist
                surface:   '#e0e8e9',  // wet sand
                primary:   '#8d76a0',  // neithal flower extracted
                secondary: '#4a8b9f',  // teal sea
                accent:    '#d48a9a',  // coral pink sky
                muted:     '#c8d6d8',  // sea foam
                border:    '#96a1a9',  // tideline
                foreground:'#1a262c',  // deep ocean
            }
        },
        dark: {
            landscape: 'Seashore after sunset',
            colors: {
                bg:        '#0e1012',  // night sea
                surface:   '#1a1e22',  // wet rocks
                primary:   '#74b9ff',  // neithal flower light
                secondary: '#7a9aa8',  // moonlit waves
                accent:    '#e4a4b4',  // pink afterglow
                muted:     '#2e3234',  // dark sand
                border:    '#4a4e52',  // wet stone
                foreground:'#e6e2dc',  // seafoam white
            }
        }
    },

    // ═══════════════════════════════════════════════════════════════
    // MULLAI — Forest, evening
    // ═══════════════════════════════════════════════════════════════
    mullai: {
        name: 'Mullai',
        tamil: 'முல்லை',
        time: 'Maalai (Evening)',
        emotion: 'Practice, patience, steady rhythm',

        light: {
            landscape: 'Forest in golden hour',
            colors: {
                bg:        '#edf2ea',  // misty forest green
                surface:   '#dde6da',  // pale leaf green
                primary:   '#fdcb6e',  // mullai flower gold
                secondary: '#607d3b',  // leaf green
                accent:    '#e8dcb4',  // jasmine glow
                muted:     '#b8c5b6',  // sage green
                border:    '#8f9e8a',  // mossy border
                foreground:'#1a261a',  // deep forest shade
            }
        },
        dark: {
            landscape: 'Forest at dusk',
            colors: {
                bg:        '#0d140d',  // deep forest night
                surface:   '#1c261c',  // dark canopy
                primary:   '#ffeaa7',  // mullai flower pale gold
                secondary: '#558b2f',  // forest moss
                accent:    '#e8dcb4',  // jasmine in darkness
                muted:     '#2c3e2c',  // deep undergrowth
                border:    '#3e4f3e',  // shadowed leaves
                foreground:'#e4e8e2',  // moonlit mist
            }
        }
    },

    // ═══════════════════════════════════════════════════════════════
    // KURINJI — Mountains, midnight
    // ═══════════════════════════════════════════════════════════════
    kurinji: {
        name: 'Kurinji',
        tamil: 'குறிஞ்சி',
        time: 'Yamam (Midnight)',
        emotion: 'Mastery, union, achievement',

        light: {
            landscape: 'Mountains in soft light',
            colors: {
                bg:        '#f4f4f8',  // mountain mist
                surface:   '#e6e6ee',  // cloud cover
                primary:   '#6c5ce7',  // kurinji flower purple
                secondary: '#7a7a88',  // granite stone
                accent:    '#9898c8',  // lavender fields
                muted:     '#c8c8d4',  // morning fog
                border:    '#a4a4b4',  // rocky outcrop
                foreground:'#1a1c24',  // deep shadow
            }
        },
        dark: {
            landscape: 'Mountains at midnight',
            colors: {
                bg:        '#0a0a10',  // mountain night
                surface:   '#18181e',  // rocky outcrop
                primary:   '#a29bfe',  // kurinji flower light
                secondary: '#8a8a98',  // moonlit stone
                accent:    '#a8b0d8',  // starlit lavender
                muted:     '#2a2834',  // stone shadow
                border:    '#44425a',  // ridge line
                foreground:'#e6e4ea',  // moonlight
            }
        }
    }
};

// Flatten into themes object for backwards compatibility
export const roleDescriptions = {
    bg: 'page background',
    surface: 'cards, elevated containers',
    primary: 'main actions (buttons, links)',
    secondary: 'supporting actions',
    accent: 'highlights, alerts, badges',
    muted: 'disabled, subtle elements',
    border: 'dividers, outlines',
    foreground: 'main text on bg/surface'
};

export const themes = {};
for (const [id, t] of Object.entries(thinnai)) {
    for (const mode of ['light', 'dark']) {
        const themeId = `${id}-${mode}`;
        const modeData = t[mode];

        themes[themeId] = {
            meta: {
                name: t.name,
                tamil: t.tamil,
                landscape: modeData.landscape,
                time: t.time,
                emotion: t.emotion,
                mode: mode
            },
            colors: {}
        };

        // Add colors with auto-calculated contrast colors
        for (const [role, value] of Object.entries(modeData.colors)) {
            themes[themeId].colors[role] = {
                value: value,
                on: getContrastColor(value),
                desc: roleDescriptions[role] || ''
            };
        }
    }
}

/**
 * Color role reference:
 *
 * bg         -> page background
 * surface    -> cards, elevated containers
 * primary    -> main actions (buttons, links)
 * secondary  -> supporting actions
 * accent     -> highlights, alerts, badges
 * muted      -> disabled, subtle elements
 * border     -> dividers, outlines
 * foreground -> main text on bg/surface
 */
