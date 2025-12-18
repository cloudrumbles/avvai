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
                primary:   '#9e4a7c',  // jarul flower
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
                primary:   '#c466a8',  // jarul flower glowing
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
                bg:        '#f8f5ef',  // bleached sand
                surface:   '#ebe6db',  // heat haze
                primary:   '#a67842',  // parched earth
                secondary: '#8a9a7a',  // dusty scrubland
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
                primary:   '#c4945c',  // sand in moonlight
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
                bg:        '#faf6f0',  // glowing horizon
                surface:   '#ebe5db',  // warm sand
                primary:   '#d47a4a',  // setting sun
                secondary: '#6a8088',  // distant sea
                accent:    '#d48a9a',  // coral pink sky
                muted:     '#d6d0c8',  // sea foam
                border:    '#a9a196',  // tideline
                foreground:'#2c2620',  // deep ocean
            }
        },
        dark: {
            landscape: 'Seashore after sunset',
            colors: {
                bg:        '#0e1012',  // night sea
                surface:   '#1a1e22',  // wet rocks
                primary:   '#e8946a',  // last glow on horizon
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
                bg:        '#f5f4f0',  // filtered sunlight
                surface:   '#e8e6e0',  // dappled shade
                primary:   '#b85a42',  // evening sun through trees
                secondary: '#7a6a4a',  // deer coat
                accent:    '#e8dcb4',  // jasmine glow
                muted:     '#ccd0c4',  // forest floor
                border:    '#a4a898',  // bark
                foreground:'#2a2c24',  // deep shade
            }
        },
        dark: {
            landscape: 'Forest at dusk',
            colors: {
                bg:        '#0c0e0c',  // forest darkness
                surface:   '#1a1e1a',  // shadowed glade
                primary:   '#c45a3d',  // dying sun through trees
                secondary: '#8a7350',  // deer's hide
                accent:    '#e8dcb4',  // jasmine in darkness
                muted:     '#2e382c',  // undergrowth
                border:    '#4a5444',  // bark shadow
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
                primary:   '#5a68a8',  // kurinji flower
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
                primary:   '#6b7cc4',  // kurinji flower
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
                on: getContrastColor(value)
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
