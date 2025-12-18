/**
 * BUILD TOKENS → CSS
 * Generates CSS custom properties from tokens.js
 *
 * Usage: node build-tokens.js
 * Output: tokens.css
 */

import { themes } from './tokens.js';
import { writeFileSync } from 'fs';

function generateCSS() {
    let css = `/**
 * AVVAI THEME SYSTEM
 * Auto-generated from tokens.js — do not edit directly.
 * Run: node build-tokens.js
 */

`;

    for (const [themeId, theme] of Object.entries(themes)) {
        const { meta, colors } = theme;

        css += `[data-theme="${themeId}"] {
    /* ${meta.name} (${meta.tamil}): ${meta.landscape} */
    /* ${meta.time} — ${meta.emotion} */

`;

        // Background colors
        css += `    /* backgrounds */\n`;
        css += `    --bg: ${colors.bg.value};              /* ${colors.bg.desc} */\n`;
        css += `    --surface: ${colors.surface.value};    /* ${colors.surface.desc} */\n`;
        css += `\n`;

        // Semantic colors
        css += `    /* semantic colors */\n`;
        css += `    --primary: ${colors.primary.value};    /* ${colors.primary.desc} */\n`;
        css += `    --secondary: ${colors.secondary.value}; /* ${colors.secondary.desc} */\n`;
        css += `    --accent: ${colors.accent.value};      /* ${colors.accent.desc} */\n`;
        css += `    --muted: ${colors.muted.value};        /* ${colors.muted.desc} */\n`;
        css += `    --border: ${colors.border.value};      /* ${colors.border.desc} */\n`;
        css += `    --foreground: ${colors.foreground.value}; /* ${colors.foreground.desc} */\n`;
        css += `\n`;

        // Text on colored backgrounds
        css += `    /* text on colored backgrounds */\n`;
        css += `    --on-primary: ${colors.primary.on};\n`;
        css += `    --on-secondary: ${colors.secondary.on};\n`;
        css += `    --on-accent: ${colors.accent.on};\n`;
        css += `    --on-muted: ${colors.muted.on};\n`;

        css += `}\n\n`;
    }

    // Add utility classes
    css += `/* Utility classes — background + text bundled */
.bg-primary { background: var(--primary); color: var(--on-primary); }
.bg-secondary { background: var(--secondary); color: var(--on-secondary); }
.bg-accent { background: var(--accent); color: var(--on-accent); }
.bg-muted { background: var(--muted); color: var(--on-muted); }
.bg-surface { background: var(--surface); color: var(--foreground); }
`;

    return css;
}

// Generate and write
const css = generateCSS();
writeFileSync('tokens.css', css);
console.log('Generated tokens.css');

// Also output a summary
console.log('\nThemes:');
for (const [id, theme] of Object.entries(themes)) {
    console.log(`  ${id}: ${theme.meta.name} (${theme.meta.mode} mode)`);
}
