import { render } from 'solid-js/web';
import { createSignal, createEffect, For, Show } from 'solid-js';
import './tokens.css';
import './components.css';
import { themes, thinnai } from '../../tokens.js';

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

const thinnaiList = Object.entries(thinnai).map(([id, t]) => ({
  id,
  name: t.name,
  color: t.light.colors.primary,
  meta: t.emotion.split(',')[0].toLowerCase() // Use first part of emotion as meta
}));

function App() {
  const [currentThinnai, setThinnai] = createSignal('marutham');
  const [currentMode, setMode] = createSignal('dark');
  const [currentView, setView] = createSignal('themes');
  const [headingFont, setHeadingFont] = createSignal('Eczar');
  const [bodyFont, setBodyFont] = createSignal('Mukta Malar');
  
  createEffect(() => {
    const theme = `${currentThinnai()}-${currentMode()}`;
    document.body.dataset.theme = theme;
  });

  const currentTheme = () => themes[`${currentThinnai()}-${currentMode()}`];

  return (
    <div class="layout">
      <aside class="sidebar">
        <h1>Avvai</h1>
        <p class="sidebar-subtitle">Styleguide (SolidJS)</p>

        <p class="nav-title">Navigation</p>
        <ul class="theme-list">
             <li>
                <div 
                  class={`theme-row ${currentView() === 'themes' ? 'active' : ''}`}
                  onClick={() => setView('themes')}
                >
                  <button class="theme-select">Themes & Colors</button>
                </div>
             </li>
             <li>
                <div 
                  class={`theme-row ${currentView() === 'fonts' ? 'active' : ''}`}
                  onClick={() => setView('fonts')}
                >
                  <button class="theme-select">Typography</button>
                </div>
             </li>
        </ul>

        <Show when={currentView() === 'themes'}>
          <p class="nav-title">Themes</p>
          <ul class="theme-list">
            <For each={thinnaiList}>{(t) => (
              <li>
                <div 
                  class={`theme-row ${currentThinnai() === t.id ? 'active' : ''}`}
                  onClick={() => setThinnai(t.id)}
                >
                  <button class="theme-select">
                    <span class="theme-dot" style={{ background: t.color }}></span>
                    <span>
                      {t.name}
                      <span class="theme-meta">{t.meta}</span>
                    </span>
                  </button>
                  <div class="mode-toggle">
                    <button 
                      class={`mode-btn ${currentThinnai() === t.id && currentMode() === 'light' ? 'active' : ''}`}
                      onClick={(e) => { e.stopPropagation(); setThinnai(t.id); setMode('light'); }}
                      title="Light mode"
                    >
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                          <circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
                      </svg>
                    </button>
                    <button 
                      class={`mode-btn ${currentThinnai() === t.id && currentMode() === 'dark' ? 'active' : ''}`}
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

      <main>
        <Show when={currentView() === 'themes'}>
          <div class="theme-info">
              <div class="theme-info-title">
                {currentTheme().meta.name} {currentTheme().meta.mode === 'light' ? 'Light' : 'Dark'} ({currentTheme().meta.tamil})
              </div>
              <div class="theme-info-meta">
                {currentTheme().meta.landscape} — {currentTheme().meta.emotion}
              </div>
          </div>

          <section id="colors">
              <h2>Colors</h2>
              <p class="desc">Core palette tokens for this theme.</p>
              <div class="swatches">
                  <For each={Object.keys(currentTheme().colors)}>{(role) => {
                      const colorData = () => currentTheme().colors[role];
                      
                      return (
                          <div class="swatch">
                              <div class="swatch-color" style={{ 
                                  background: `var(--${role})`,
                                  display: 'flex',
                                  'align-items': 'center',
                                  'justify-content': 'center',
                                  color: colorData().on
                              }}>
                                  <span class="swatch-label">{colorData().value}</span>
                              </div>
                              <div class="swatch-info">
                                  <div class="swatch-name">{role.charAt(0).toUpperCase() + role.slice(1)}</div>
                                  <div class="swatch-hex" style={{ opacity: 0.5 }}>--{role}</div>
                              </div>
                          </div>
                      );
                  }}</For>
              </div>
          </section>

          <section id="components" style={{ "margin-top": "3rem" }}>
            <h2>Components</h2>
            <p class="desc">UI elements using semantic tokens.</p>
            
            <div class="components-grid">
                <div class="component-section">
                    <h3>Buttons</h3>
                    <div class="btn-group">
                        <button class="btn btn-primary">Primary</button>
                        <button class="btn btn-secondary">Secondary</button>
                        <button class="btn btn-outline">Outline</button>
                        <button class="btn btn-ghost">Ghost</button>
                    </div>
                    <div class="btn-group">
                        <button class="btn btn-primary" disabled>Disabled</button>
                        <button class="btn btn-secondary" disabled>Disabled</button>
                    </div>
                </div>

                <div class="component-section">
                    <h3>Badges</h3>
                    <div class="badge-group" style={{ "margin-bottom": "1rem" }}>
                        <span class="badge badge-primary">Primary</span>
                        <span class="badge badge-secondary">Secondary</span>
                        <span class="badge badge-accent">Accent</span>
                        <span class="badge badge-muted">Muted</span>
                    </div>
                    <p style={{ "font-size": "0.9rem", opacity: 0.7 }}>
                        Used for status indicators, tags, and highlights.
                    </p>
                </div>

                <div class="component-section">
                    <h3>Cards</h3>
                    <div class="card">
                        <span class="card-title">Simple Card</span>
                        <p class="card-text">This is a basic card component sitting on the surface layer. It uses the background color for contrast.</p>
                    </div>
                    <div class="card" style={{ "border-left": "4px solid var(--accent)" }}>
                        <span class="card-title">Accent Card</span>
                        <p class="card-text">A card with an accent border to highlight importance.</p>
                    </div>
                </div>

                <div class="component-section">
                    <h3>Inputs</h3>
                    <div class="form-group">
                        <label class="label">Text Input</label>
                        <input type="text" class="input" placeholder="Type something..." />
                    </div>
                    <div class="form-group">
                        <label class="label">Disabled Input</label>
                        <input type="text" class="input" placeholder="Cannot type here" disabled />
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
            </div>
          </section>
        </Show>

        <Show when={currentView() === 'fonts'}>
          <div class="theme-info">
            <div class="theme-info-title">Typography</div>
            <div class="theme-info-meta">Select and preview font combinations</div>
          </div>

          <section id="font-showroom">
            <h2>Font Showroom</h2>
            <p class="desc">Choose different fonts for headings and body text to see how they pair together.</p>

            <div style={{ display: 'grid', 'grid-template-columns': '1fr 1fr', gap: '2rem', 'margin-bottom': '2rem' }}>
              <div>
                <label style={{ display: 'block', 'margin-bottom': '0.5rem', 'font-size': '0.9rem', opacity: 0.7 }}>Heading Font</label>
                <div style={{ display: 'flex', 'flex-wrap': 'wrap', gap: '0.5rem' }}>
                  <For each={fonts}>{(font) => (
                    <button 
                      onClick={() => setHeadingFont(font)}
                      style={{
                        padding: '0.5rem 1rem',
                        background: headingFont() === font ? 'var(--primary)' : 'var(--surface)',
                        color: headingFont() === font ? 'var(--bg)' : 'var(--foreground)',
                        border: '1px solid var(--border)',
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
                        background: bodyFont() === font ? 'var(--primary)' : 'var(--surface)',
                        color: bodyFont() === font ? 'var(--bg)' : 'var(--foreground)',
                        border: '1px solid var(--border)',
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
              background: 'var(--surface)', 
              'border-radius': '8px',
              border: '1px solid var(--border)' 
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
              
              <div style={{ 'margin-top': '2rem', 'padding-top': '2rem', 'border-top': '1px solid var(--border)', opacity: 0.6, 'font-family': 'Space Mono, monospace', 'font-size': '0.8rem' }}>
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