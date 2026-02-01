<script lang="ts">
	type TokenGroup = {
		label: string;
		tokens: string[];
	};

	const semantic = [
		{ label: 'Text', tokens: ['--color-text', '--color-text-muted', '--color-text-subtle', '--color-text-inverse'] },
		{ label: 'Background', tokens: ['--color-bg', '--color-bg-soft', '--color-bg-pressed', '--color-bg-inverse'] },
		{ label: 'Accent', tokens: ['--color-accent', '--color-accent-strong', '--color-accent-tint', '--color-accent-wash'] },
		{ label: 'Secondary', tokens: ['--color-accent-secondary', '--color-accent-secondary-tint'] },
		{ label: 'Emphasis', tokens: ['--color-highlight'] },
		{ label: 'Feedback', tokens: ['--color-success', '--color-success-tint'] }
	] satisfies TokenGroup[];

	const spaceTokens = [
		'--space-0',
		'--space-0-5',
		'--space-0-75',
		'--space-1',
		'--space-1-5',
		'--space-2',
		'--space-2-5',
		'--space-3',
		'--space-4',
		'--space-4-5',
		'--space-5',
		'--space-6',
		'--space-7',
		'--space-8',
		'--space-9',
		'--space-10',
		'--space-12'
	];

	const radiusTokens = [
		'--radius-0',
		'--radius-0-5',
		'--radius-0-75',
		'--radius-1',
		'--radius-1-5',
		'--radius-2',
		'--radius-2-5',
		'--radius-3',
		'--radius-4',
		'--radius-5',
		'--radius-pill'
	];

	const fontSizeTokens = [
		'--font-size-0',
		'--font-size-0-5',
		'--font-size-0-75',
		'--font-size-1',
		'--font-size-1-5',
		'--font-size-2',
		'--font-size-2-5',
		'--font-size-3',
		'--font-size-3-5',
		'--font-size-4',
		'--font-size-4-5',
		'--font-size-5',
		'--font-size-5-5',
		'--font-size-6',
		'--font-size-6-5',
		'--font-size-7',
		'--font-size-8',
		'--font-size-9',
		'--font-size-10',
		'--font-size-11',
		'--font-size-12'
	];

	const lineHeightTokens = [
		'--line-height-0',
		'--line-height-0-75',
		'--line-height-1',
		'--line-height-1-25',
		'--line-height-1-3',
		'--line-height-1-35',
		'--line-height-1-4',
		'--line-height-1-5',
		'--line-height-1-6',
		'--line-height-1-7',
		'--line-height-1-8',
		'--line-height-2',
		'--line-height-2-2',
		'--line-height-3'
	];

	const componentDimensionTokens = [
		'--size-icon-btn',
		'--size-touch-target'
	];

	const primitiveClasses = [
		{
			name: '.btn-reset',
			description: 'Strips all default button styling. Use as a base for custom button components.',
			usage: 'Apply to buttons that need completely custom styling without browser defaults.'
		},
		{
			name: '.btn-icon',
			description: 'Square icon button with hover/active states. Uses --size-icon-btn for dimensions.',
			usage: 'Use for icon-only buttons like close, menu, or action icons.'
		},
		{
			name: '.interactive-text',
			description: 'Subtle hover highlight for clickable text. Perfect for word-level interactions.',
			usage: 'Apply to clickable text elements like dictionary lookup words.'
		}
	];

	const principles = [
		'Warm manuscript palette with restrained contrast',
		'Legibility-first Tamil typography and generous line-height',
		'Soft surfaces, low-gloss shadows, and calm emphasis',
		'Semantic tokens only in components; palette stays in tokens'
	];
</script>

<div class="ds-page">
	<header class="hero">
		<div class="hero-copy">
			<p class="hero-kicker">Design System</p>
			<h1>Avvai UI, unified.</h1>
			<p class="hero-body">
				A semantic token system and component language inspired by palm-leaf manuscripts.
				Use tokens to keep the UI consistent and theme-ready.
			</p>
			<div class="hero-meta">
				<span>Semantic-first</span>
				<span>16 base colors</span>
				<span>Tokenized scale</span>
			</div>
		</div>
		<div class="hero-card">
			<p class="card-label">Primary actions</p>
			<h2>Maroon for intent, terracotta for warmth.</h2>
			<div class="card-actions">
				<button class="btn btn-primary">Primary</button>
				<button class="btn btn-secondary">Secondary</button>
				<button class="btn btn-ghost">Ghost</button>
			</div>
		</div>
	</header>

	<section class="section">
		<div class="section-head">
			<h2>Principles</h2>
			<p>Design values that guide every component and surface.</p>
		</div>
		<div class="principles">
			{#each principles as principle (principle)}
				<div class="principle">
					<span class="dot"></span>
					<p>{principle}</p>
				</div>
			{/each}
		</div>
	</section>

	<section class="section split">
		<div>
			<div class="section-head">
				<h2>Semantic tokens</h2>
				<p>Components must use semantic tokens, not raw palette values.</p>
			</div>
			<div class="semantic-grid">
				{#each semantic as group (group.label)}
					<div class="semantic-card">
						<h3>{group.label}</h3>
						<div class="token-list">
							{#each group.tokens as token (token)}
								<span>{token}</span>
							{/each}
						</div>
					</div>
				{/each}
			</div>
		</div>
		<div class="surface-stack">
			<div class="surface panel">
				<p class="surface-title">Surface</p>
				<p>Use <span>--color-bg</span> and <span>--border-strong</span>.</p>
			</div>
			<div class="surface panel soft">
				<p class="surface-title">Soft surface</p>
				<p>Use <span>--color-bg-soft</span> and subtle shadow.</p>
			</div>
			<div class="surface panel accent">
				<p class="surface-title">Accent surface</p>
				<p>Use <span>--color-accent-secondary-tint</span>.</p>
			</div>
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Spacing scale</h2>
			<p>A comprehensive spacing scale from 1px to 80px. Visual boxes show relative sizes.</p>
		</div>
		<div class="scale-visual-card">
			<div class="scale-tokens-list">
				{#each spaceTokens as token (token)}
					<div class="scale-token-row">
						<span class="token-name">{token}</span>
						<div class="scale-box-container">
							<div class="scale-box space-box" style="width: var({token}); height: var({token});"></div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Radius scale</h2>
			<p>Border radius tokens from subtle to pill-shaped.</p>
		</div>
		<div class="scale-visual-card">
			<div class="radius-preview-grid">
				{#each radiusTokens as token (token)}
					<div class="radius-preview-item">
						<div class="radius-box" style="border-radius: var({token});"></div>
						<span class="token-name-small">{token.replace('--radius-', '')}</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Typography scale</h2>
			<p>Font sizes from 0.5625rem to 4rem with half-step increments.</p>
		</div>
		<div class="scale-visual-card">
			<div class="type-scale-list">
				{#each fontSizeTokens as token (token)}
					<div class="type-scale-row">
						<span class="token-name">{token}</span>
						<span class="type-sample" style="font-size: var({token});">Tamil தமிழ்</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Line height scale</h2>
			<p>Line heights from tight (1) to loose (2.2) for various text contexts.</p>
		</div>
		<div class="scale-visual-card">
			<div class="line-height-list">
				{#each lineHeightTokens as token (token)}
					<div class="line-height-row">
						<span class="token-name">{token}</span>
						<div class="line-height-sample" style="line-height: var({token});">
							<span>Line one</span>
							<span>Line two</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Component dimensions</h2>
			<p>Standard sizes for interactive elements and touch targets.</p>
		</div>
		<div class="scale-visual-card">
			<div class="dimension-preview-grid">
				{#each componentDimensionTokens as token (token)}
					<div class="dimension-preview-item">
						<div class="dimension-box" style="width: var({token}); height: var({token});">
							<span class="dimension-label">{token.replace('--size-', '')}</span>
						</div>
						<span class="token-name-small">{token}</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Primitive classes</h2>
			<p>Reusable utility classes for common interactive patterns.</p>
		</div>
		<div class="primitives-grid">
			{#each primitiveClasses as prim (prim.name)}
				<div class="primitive-card">
					<h3>{prim.name}</h3>
					<p class="primitive-desc">{prim.description}</p>
					<p class="primitive-usage"><strong>Usage:</strong> {prim.usage}</p>
					<div class="primitive-demo">
						{#if prim.name === '.btn-reset'}
							<button class="btn-reset demo-btn-reset">Reset button</button>
						{:else if prim.name === '.btn-icon'}
							<button class="btn-icon" aria-label="Example icon button">
								<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M12 5v14M5 12h14"/>
								</svg>
							</button>
						{:else if prim.name === '.interactive-text'}
							<span class="interactive-text demo-interactive">Clickable word</span>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</section>

	<section class="section">
		<div class="section-head">
			<h2>Components</h2>
			<p>Reference patterns built on tokens only.</p>
		</div>
		<div class="component-grid">
			<div class="component-card">
				<h3>Buttons</h3>
				<div class="component-row">
					<button class="btn btn-primary">Primary</button>
					<button class="btn btn-secondary">Secondary</button>
					<button class="btn btn-ghost">Ghost</button>
				</div>
			</div>
			<div class="component-card">
				<h3>Inputs</h3>
				<div class="component-row">
					<input class="input" placeholder="Search Tamil" />
					<select class="input">
						<option>Catamaran</option>
						<option>Tiro Tamil</option>
					</select>
				</div>
			</div>
			<div class="component-card">
				<h3>Cards</h3>
				<div class="card-stack">
					<div class="card">
						<p class="card-title">Lesson Card</p>
						<p class="card-body">A warm surface for guidance and reflection.</p>
						<span class="badge">New</span>
					</div>
					<div class="card accent">
						<p class="card-title">Practice Card</p>
						<p class="card-body">Secondary accent for momentum.</p>
						<span class="badge alt">Accent</span>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.ds-page {
		min-height: 100vh;
		background: radial-gradient(circle at top, var(--overlay-gold-08), transparent 55%),
			linear-gradient(180deg, var(--color-bg) 0%, var(--color-bg-soft) 55%, var(--color-bg-pressed) 100%);
		color: var(--color-text);
		padding: var(--space-7) var(--space-5) var(--space-9);
		font-family: var(--font-sans);
		position: relative;
		overflow: hidden;
	}

	.ds-page::before {
		content: '';
		position: fixed;
		inset: 0;
		pointer-events: none;
		background-image: repeating-linear-gradient(
			0deg,
			var(--overlay-ink-08) 0px,
			var(--overlay-ink-08) 1px,
			transparent 1px,
			transparent var(--space-2-5)
		);
		opacity: 0.04;
	}

	.hero {
		display: grid;
		grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr);
		gap: var(--space-7);
		align-items: center;
		margin-bottom: var(--space-8);
	}

	.hero-kicker {
		text-transform: uppercase;
		letter-spacing: var(--letter-10);
		font-size: var(--font-size-1);
		color: var(--color-text-subtle);
		margin: 0 0 var(--space-2);
	}

	.hero h1 {
		font-family: var(--font-display);
		font-size: clamp(var(--font-size-9), 4vw, var(--font-size-11));
		line-height: var(--line-height-1);
		margin: 0 0 var(--space-3);
	}

	.hero-body {
		font-size: var(--font-size-3);
		line-height: var(--line-height-1-6);
		max-width: 42ch;
		margin: 0;
	}

	.hero-meta {
		display: flex;
		gap: var(--space-2);
		margin-top: var(--space-4);
		color: var(--color-text-subtle);
		font-size: var(--font-size-1-5);
	}

	.hero-meta span {
		padding: var(--space-0-5) var(--space-2);
		border-radius: var(--radius-pill);
		border: 1px solid var(--color-bg-soft);
		background: var(--color-bg);
	}

	.hero-card {
		background: var(--color-bg);
		border: var(--border-strong);
		border-radius: var(--radius-4);
		box-shadow: var(--shadow-2);
		padding: var(--space-6);
	}

	.card-label {
		text-transform: uppercase;
		letter-spacing: var(--letter-8);
		font-size: var(--font-size-1);
		color: var(--color-text-subtle);
		margin: 0 0 var(--space-2);
	}

	.hero-card h2 {
		font-family: var(--font-display);
		font-size: var(--font-size-5);
		margin: 0 0 var(--space-4);
	}

	.card-actions {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	.section {
		margin-bottom: var(--space-8);
	}

	.section-head h2 {
		font-family: var(--font-display);
		font-size: var(--font-size-5);
		margin: 0 0 var(--space-2);
	}

	.section-head p {
		margin: 0 0 var(--space-4);
		color: var(--color-text-muted);
	}

	.principles {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: var(--space-3);
	}

	.principle {
		display: flex;
		gap: var(--space-2);
		align-items: flex-start;
		padding: var(--space-3);
		border-radius: var(--radius-3);
		background: var(--color-bg);
		border: var(--border-1);
		box-shadow: var(--shadow-1);
	}

	.principle .dot {
		width: var(--space-1);
		height: var(--space-1);
		margin-top: var(--space-0-5);
		border-radius: var(--radius-pill);
		background: var(--color-accent-secondary);
	}

	.split {
		display: grid;
		grid-template-columns: minmax(0, 1.4fr) minmax(0, 0.9fr);
		gap: var(--space-7);
		align-items: start;
	}

	.semantic-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: var(--space-3);
	}

	.semantic-card {
		border: var(--border-1);
		border-radius: var(--radius-3);
		padding: var(--space-4);
		background: var(--color-bg);
		box-shadow: var(--shadow-1);
	}

	.semantic-card h3 {
		font-family: var(--font-display);
		margin: 0 0 var(--space-3);
	}

	.token-list {
		display: grid;
		gap: var(--space-1);
		font-size: var(--font-size-1-5);
		color: var(--color-text-muted);
	}

	.surface-stack {
		display: grid;
		gap: var(--space-3);
	}

	.surface {
		padding: var(--space-4);
		border-radius: var(--radius-3);
		border: var(--border-1);
		background: var(--color-bg);
		box-shadow: var(--shadow-1);
	}

	.surface.soft {
		background: var(--color-bg-soft);
	}

	.surface.accent {
		background: var(--color-accent-secondary-tint);
		border-color: var(--color-accent-secondary);
	}

	.surface-title {
		font-family: var(--font-display);
		margin: 0 0 var(--space-2);
	}

	.surface span {
		color: var(--color-accent);
	}

	/* ── Scale Visual Card ── */
	.scale-visual-card {
		border: var(--border-1);
		border-radius: var(--radius-3);
		padding: var(--space-4);
		background: var(--color-bg);
		box-shadow: var(--shadow-1);
	}

	/* ── Spacing Scale ── */
	.scale-tokens-list {
		display: grid;
		gap: var(--space-2);
	}

	.scale-token-row {
		display: grid;
		grid-template-columns: 140px 1fr;
		align-items: center;
		gap: var(--space-3);
	}

	.token-name {
		font-size: var(--font-size-1-5);
		color: var(--color-text-muted);
		font-family: monospace;
	}

	.scale-box-container {
		display: flex;
		align-items: center;
	}

	.scale-box {
		background: var(--color-accent-secondary);
		border-radius: var(--radius-0-5);
		min-width: 1px;
		min-height: 1px;
	}

	/* ── Radius Scale ── */
	.radius-preview-grid {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-4);
	}

	.radius-preview-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
	}

	.radius-box {
		width: 48px;
		height: 48px;
		background: var(--color-accent-secondary);
		border: 2px solid var(--color-accent-secondary);
	}

	.token-name-small {
		font-size: var(--font-size-1);
		color: var(--color-text-subtle);
		font-family: monospace;
	}

	/* ── Typography Scale ── */
	.type-scale-list {
		display: grid;
		gap: var(--space-3);
	}

	.type-scale-row {
		display: grid;
		grid-template-columns: 160px 1fr;
		align-items: baseline;
		gap: var(--space-4);
	}

	.type-sample {
		font-family: var(--font-serif);
		color: var(--color-text);
		line-height: var(--line-height-1-4);
	}

	/* ── Line Height Scale ── */
	.line-height-list {
		display: grid;
		gap: var(--space-4);
	}

	.line-height-row {
		display: grid;
		grid-template-columns: 180px 1fr;
		align-items: start;
		gap: var(--space-4);
	}

	.line-height-sample {
		display: flex;
		flex-direction: column;
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		background: var(--color-bg-soft);
		padding: var(--space-2);
		border-radius: var(--radius-2);
	}

	/* ── Component Dimensions ── */
	.dimension-preview-grid {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-6);
		align-items: flex-end;
	}

	.dimension-preview-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
	}

	.dimension-box {
		background: var(--color-accent-tint);
		border: 2px dashed var(--color-accent);
		border-radius: var(--radius-2);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.dimension-label {
		font-size: var(--font-size-0-75);
		color: var(--color-accent);
		font-weight: 600;
		text-align: center;
	}

	/* ── Primitives Grid ── */
	.primitives-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: var(--space-4);
	}

	.primitive-card {
		border: var(--border-1);
		border-radius: var(--radius-3);
		padding: var(--space-4);
		background: var(--color-bg);
		box-shadow: var(--shadow-1);
	}

	.primitive-card h3 {
		font-family: monospace;
		font-size: var(--font-size-3);
		color: var(--color-accent);
		margin: 0 0 var(--space-2);
	}

	.primitive-desc {
		margin: 0 0 var(--space-2);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
	}

	.primitive-usage {
		margin: 0 0 var(--space-3);
		font-size: var(--font-size-1-5);
		color: var(--color-text-subtle);
	}

	.primitive-demo {
		padding: var(--space-3);
		background: var(--color-bg-soft);
		border-radius: var(--radius-2);
		display: flex;
		justify-content: center;
	}

	.demo-btn-reset {
		font-size: var(--font-size-2);
	}

	.demo-interactive {
		padding: var(--space-0-5) var(--space-1);
		font-size: var(--font-size-3);
	}

	/* ── Components ── */
	.component-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
		gap: var(--space-4);
	}

	.component-card {
		border: var(--border-1);
		border-radius: var(--radius-3);
		padding: var(--space-4);
		background: var(--color-bg);
		box-shadow: var(--shadow-1);
	}

	.component-card h3 {
		font-family: var(--font-display);
		margin: 0 0 var(--space-3);
	}

	.component-row {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}

	.card-stack {
		display: grid;
		gap: var(--space-3);
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--font-size-4);
		margin: 0 0 var(--space-2);
	}

	.card-body {
		margin: 0 0 var(--space-3);
		color: var(--color-text-muted);
	}

	@media (max-width: 900px) {
		.hero,
		.split {
			grid-template-columns: 1fr;
		}

		.scale-token-row,
		.type-scale-row,
		.line-height-row {
			grid-template-columns: 1fr;
			gap: var(--space-1);
		}

		.token-name {
			font-size: var(--font-size-1);
		}
	}

	@media (max-width: 600px) {
		.ds-page {
			padding: var(--space-6) var(--space-4) var(--space-8);
		}

		.hero-meta {
			flex-wrap: wrap;
		}
	}
</style>
