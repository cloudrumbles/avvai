import { render } from 'solid-js/web';
import { createSignal, createEffect, For, Show } from 'solid-js';
import { themes, themeData, thinnaiMeta, vars, type ThemeKey } from './theme.css';
import * as s from './styles.css';

const fonts = [
  'Arima',
  'Baloo Thambi 2',
  'Catamaran',
  'Eczar',
  'Hind Madurai',
  'Kavivanar',
  'Mukta Malar',
  'Noto Sans Tamil'
];

const sampleText = {
  heading: 'தமிழ் எழுத்துரு காட்சியகம்',
  body: 'அறம் செய்ய விரும்பு, ஆறுவது சினம், இயல்வது கரவேல், ஈவது விலக்கேல், உடையது விளம்பேல், ஊக்கமது கைவிடேல், எண் எழுத்து இகழேல், ஏற்பது இகழ்ச்சி, ஐயம் இட்டு உண், ஒப்புரவு ஒழுகு, ஓதுவது ஒழியேல், ஔவியம் பேசேல்.'
};

function SimpleTabs() {
  const [activeTab, setActiveTab] = createSignal('account');

  return (
    <div class={s.tabsRoot}>
      <div class={s.tabsList}>
        <button
          class={`${s.tabsTrigger} ${activeTab() === 'account' ? s.tabsTriggerActive : ''}`}
          onClick={() => setActiveTab('account')}
        >Account</button>
        <button
          class={`${s.tabsTrigger} ${activeTab() === 'password' ? s.tabsTriggerActive : ''}`}
          onClick={() => setActiveTab('password')}
        >Password</button>
        <button
          class={`${s.tabsTrigger} ${activeTab() === 'settings' ? s.tabsTriggerActive : ''}`}
          onClick={() => setActiveTab('settings')}
        >Settings</button>
      </div>
      <Show when={activeTab() === 'account'}>
        <div class={s.tabsContent}>
          <p class={s.cardText}>Make changes to your account here. Click save when you're done.</p>
          <div class={s.btnGroup} style={{ "margin-top": "1rem" }}>
            <button class={`${s.btn} ${s.btnPrimary}`}>Save changes</button>
          </div>
        </div>
      </Show>
      <Show when={activeTab() === 'password'}>
        <div class={s.tabsContent}>
          <p class={s.cardText}>Change your password here. After saving, you'll be logged out.</p>
          <div class={s.formGroup} style={{ "margin-top": "1rem" }}>
            <label class={s.label}>Current password</label>
            <input type="password" class={s.input} />
          </div>
          <div class={s.formGroup}>
            <label class={s.label}>New password</label>
            <input type="password" class={s.input} />
          </div>
          <button class={`${s.btn} ${s.btnSecondary}`}>Update password</button>
        </div>
      </Show>
      <Show when={activeTab() === 'settings'}>
        <div class={s.tabsContent}>
          <p class={s.cardText}>Manage your settings and preferences.</p>
        </div>
      </Show>
    </div>
  );
}

function App() {
  const [currentThinnai, setThinnai] = createSignal('marutham');
  const [currentMode, setMode] = createSignal<'light' | 'dark'>('dark');
  const [currentView, setView] = createSignal('themes');
  const [headingFont, setHeadingFont] = createSignal('Eczar');
  const [bodyFont, setBodyFont] = createSignal('Mukta Malar');

  const themeKey = () => `${currentThinnai()}-${currentMode()}` as ThemeKey;

  createEffect(() => {
    // Apply theme class to body
    const key = themeKey();
    document.body.className = themes[key];
  });

  const currentTheme = () => themeData[themeKey()];

  return (
    <div class={s.layout}>
      <aside class={s.sidebar}>
        <h1 class={s.sidebarTitle}>Avvai</h1>
        <p class={s.sidebarSubtitle}>Styleguide (SolidJS)</p>

        <p class={s.navTitle}>Navigation</p>
        <ul class={s.themeList}>
          <li>
            <div
              class={`${s.themeRow} ${currentView() === 'themes' ? s.themeRowActive : ''}`}
              onClick={() => setView('themes')}
            >
              <button class={s.themeSelect}>Themes & Colors</button>
            </div>
          </li>
          <li>
            <div
              class={`${s.themeRow} ${currentView() === 'fonts' ? s.themeRowActive : ''}`}
              onClick={() => setView('fonts')}
            >
              <button class={s.themeSelect}>Typography</button>
            </div>
          </li>
        </ul>

        <Show when={currentView() === 'themes'}>
          <p class={s.navTitle}>Themes</p>
          <ul class={s.themeList}>
            <For each={thinnaiMeta}>{(t) => (
              <li>
                <div
                  class={`${s.themeRow} ${currentThinnai() === t.id ? s.themeRowActive : ''}`}
                  onClick={() => setThinnai(t.id)}
                >
                  <button class={s.themeSelect}>
                    <span class={s.themeDot} style={{ background: t.color }}></span>
                    <span>
                      {t.name}
                      <span class={s.themeMeta}>{t.meta}</span>
                    </span>
                  </button>
                  <div class={s.modeToggle}>
                    <button
                      class={`${s.modeBtn} ${currentThinnai() === t.id && currentMode() === 'light' ? s.modeBtnActive : ''}`}
                      onClick={(e) => { e.stopPropagation(); setThinnai(t.id); setMode('light'); }}
                      title="Light mode"
                    >
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
                      </svg>
                    </button>
                    <button
                      class={`${s.modeBtn} ${currentThinnai() === t.id && currentMode() === 'dark' ? s.modeBtnActive : ''}`}
                      onClick={(e) => { e.stopPropagation(); setThinnai(t.id); setMode('dark'); }}
                      title="Dark mode"
                    >
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
                      </svg>
                    </button>
                  </div>
                </div>
              </li>
            )}</For>
          </ul>
        </Show>
      </aside>

      <main class={s.main}>
        <Show when={currentView() === 'themes'}>
          <div class={s.themeInfo}>
            <div class={s.themeInfoTitle}>
              {currentTheme().meta.name} {currentTheme().meta.mode === 'light' ? 'Light' : 'Dark'} ({currentTheme().meta.tamil})
            </div>
            <div class={s.themeInfoMeta}>
              {currentTheme().meta.landscape} — {currentTheme().meta.emotion}
            </div>
          </div>

          <section class={s.section} id="colors">
            <h2 class={s.sectionTitle}>Colors</h2>
            <p class={s.desc}>Core palette tokens for this theme.</p>
            <div class={s.swatches}>
              <For each={Object.keys(currentTheme().colors)}>{(role) => {
                const colorData = () => currentTheme().colors[role];

                return (
                  <div class={s.swatch}>
                    <div class={s.swatchColor} style={{
                      background: colorData().value,
                      color: colorData().on
                    }}>
                      <span class={s.swatchLabel}>{colorData().value}</span>
                    </div>
                    <div class={s.swatchInfo}>
                      <div class={s.swatchName}>{role.charAt(0).toUpperCase() + role.slice(1)}</div>
                      <div class={s.swatchHex} style={{ opacity: 0.5 }}>--{role}</div>
                    </div>
                  </div>
                );
              }}</For>
            </div>
          </section>

          <section class={s.section} id="components" style={{ "margin-top": "3rem" }}>
            <h2 class={s.sectionTitle}>Components</h2>
            <p class={s.desc}>UI elements using semantic tokens.</p>

            <div class={s.componentsGrid}>
              <div class={s.componentSection}>
                <h3>Buttons</h3>
                <div class={s.btnGroup}>
                  <button class={`${s.btn} ${s.btnPrimary}`}>Primary</button>
                  <button class={`${s.btn} ${s.btnSecondary}`}>Secondary</button>
                  <button class={`${s.btn} ${s.btnOutline}`}>Outline</button>
                  <button class={`${s.btn} ${s.btnGhost}`}>Ghost</button>
                </div>
                <div class={s.btnGroup}>
                  <button class={`${s.btn} ${s.btnPrimary}`} disabled>Disabled</button>
                  <button class={`${s.btn} ${s.btnSecondary}`} disabled>Disabled</button>
                </div>
              </div>

              <div class={s.componentSection}>
                <h3>Badges</h3>
                <div class={s.badgeGroup} style={{ "margin-bottom": "1rem" }}>
                  <span class={`${s.badge} ${s.badgePrimary}`}>Primary</span>
                  <span class={`${s.badge} ${s.badgeSecondary}`}>Secondary</span>
                  <span class={`${s.badge} ${s.badgeAccent}`}>Accent</span>
                  <span class={`${s.badge} ${s.badgeMuted}`}>Muted</span>
                </div>
                <p style={{ "font-size": "0.9rem", opacity: 0.7 }}>
                  Used for status indicators, tags, and highlights.
                </p>
              </div>

              <div class={s.componentSection}>
                <h3>Cards</h3>
                <div class={s.card}>
                  <span class={s.cardTitle}>Simple Card</span>
                  <p class={s.cardText}>This is a basic card component sitting on the surface layer. It uses the background color for contrast.</p>
                </div>
                <div class={s.card} style={{ "border-left": `4px solid ${vars.accent}` }}>
                  <span class={s.cardTitle}>Accent Card</span>
                  <p class={s.cardText}>A card with an accent border to highlight importance.</p>
                </div>
              </div>

              <div class={s.componentSection}>
                <h3>Inputs</h3>
                <div class={s.formGroup}>
                  <label class={s.label}>Text Input</label>
                  <input type="text" class={s.input} placeholder="Type something..." />
                </div>
                <div class={s.formGroup}>
                  <label class={s.label}>Disabled Input</label>
                  <input type="text" class={s.input} placeholder="Cannot type here" disabled />
                </div>
                <div style={{ display: "flex", gap: "1rem", "align-items": "center" }}>
                  <label style={{ display: "flex", "align-items": "center", gap: "0.5rem", "font-size": "0.9rem" }}>
                    <input type="checkbox" checked /> Checkbox
                  </label>
                  <label style={{ display: "flex", "align-items": "center", gap: "0.5rem", "font-size": "0.9rem" }}>
                    <input type="radio" name="radio" checked /> Radio
                  </label>
                </div>
              </div>

              <div class={s.componentSection}>
                <h3>Tabs</h3>
                <SimpleTabs />
              </div>
            </div>
          </section>
        </Show>

        <Show when={currentView() === 'fonts'}>
          <div class={s.themeInfo}>
            <div class={s.themeInfoTitle}>Typography</div>
            <div class={s.themeInfoMeta}>Select and preview font combinations</div>
          </div>

          <section class={s.section} id="font-showroom">
            <h2 class={s.sectionTitle}>Font Showroom</h2>
            <p class={s.desc}>Choose different fonts for headings and body text to see how they pair together.</p>

            <div style={{ display: 'grid', 'grid-template-columns': '1fr 1fr', gap: '2rem', 'margin-bottom': '2rem' }}>
              <div>
                <label style={{ display: 'block', 'margin-bottom': '0.5rem', 'font-size': '0.9rem', opacity: 0.7 }}>Heading Font</label>
                <div style={{ display: 'flex', 'flex-wrap': 'wrap', gap: '0.5rem' }}>
                  <For each={fonts}>{(font) => (
                    <button
                      onClick={() => setHeadingFont(font)}
                      style={{
                        padding: '0.5rem 1rem',
                        background: headingFont() === font ? vars.primary : vars.surface,
                        color: headingFont() === font ? vars.onPrimary : vars.foreground,
                        border: `1px solid ${vars.border}`,
                        'border-radius': '4px',
                        cursor: 'pointer',
                        'font-family': `'${font}'`,
                        transition: 'all 0.2s'
                      }}
                    >
                      {font}
                    </button>
                  )}</For>
                </div>
              </div>

              <div>
                <label style={{ display: 'block', 'margin-bottom': '0.5rem', 'font-size': '0.9rem', opacity: 0.7 }}>Body Font</label>
                <div style={{ display: 'flex', 'flex-wrap': 'wrap', gap: '0.5rem' }}>
                  <For each={fonts}>{(font) => (
                    <button
                      onClick={() => setBodyFont(font)}
                      style={{
                        padding: '0.5rem 1rem',
                        background: bodyFont() === font ? vars.primary : vars.surface,
                        color: bodyFont() === font ? vars.onPrimary : vars.foreground,
                        border: `1px solid ${vars.border}`,
                        'border-radius': '4px',
                        cursor: 'pointer',
                        'font-family': `'${font}'`,
                        transition: 'all 0.2s'
                      }}
                    >
                      {font}
                    </button>
                  )}</For>
                </div>
              </div>
            </div>

            <div style={{
              padding: '3rem',
              background: vars.surface,
              'border-radius': '8px',
              border: `1px solid ${vars.border}`
            }}>
              <h1 style={{
                'font-family': `'${headingFont()}'`,
                'margin-bottom': '1.5rem',
                'font-size': '2.5rem',
                'line-height': 1.2
              }}>
                {sampleText.heading}
              </h1>
              <p style={{
                'font-family': `'${bodyFont()}'`,
                'font-size': '1.1rem',
                'line-height': 1.6,
                opacity: 0.9,
                'white-space': 'pre-line'
              }}>
                {sampleText.body}
              </p>

              <div style={{
                'margin-top': '2rem',
                'padding-top': '2rem',
                'border-top': `1px solid ${vars.border}`,
                opacity: 0.6,
                'font-family': 'Space Mono, monospace',
                'font-size': '0.8rem'
              }}>
                <div>Heading: {headingFont()}</div>
                <div>Body: {bodyFont()}</div>
              </div>
            </div>
          </section>
        </Show>
      </main>
    </div>
  );
}

render(() => <App />, document.getElementById('root') as HTMLElement);
