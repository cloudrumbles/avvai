<script lang="ts">
	interface Props {
		text: string;
		title?: string;
	}

	let { text, title }: Props = $props();

	const FONTS = [
		{ label: 'Mukta Malar', value: "'Mukta Malar', sans-serif" },
		{ label: 'Catamaran', value: "'Catamaran', sans-serif" },
		{ label: 'Noto Sans Tamil', value: "'Noto Sans Tamil', sans-serif" },
		{ label: 'Noto Serif Tamil', value: "'Noto Serif Tamil', serif" },
		{ label: 'Tiro Tamil', value: "'Tiro Tamil', serif" },
		{ label: 'Hind Madurai', value: "'Hind Madurai', sans-serif" },
		{ label: 'Pavanam', value: "'Pavanam', sans-serif" },
		{ label: 'Baloo Thambi 2', value: "'Baloo Thambi 2', cursive" }
	];

	let fontFamily = $state(FONTS[0].value);
	let fontSize = $state(16);
	let lineHeight = $derived(fontSize * 2);

	const MIN_FONT = 16;
	const MAX_FONT = 32;
	const STEP = 2;

	function increase() {
		if (fontSize < MAX_FONT) fontSize += STEP;
	}

	function decrease() {
		if (fontSize > MIN_FONT) fontSize -= STEP;
	}

	/* ── text parsing ──
	   detect structure: headings, verse couplets, explanations, prose paragraphs.
	   - first group (before first blank line) → heading
	   - lines starting with "பொருள்:" → explanation
	   - single-line group where line > 100 chars → paragraph (prose)
	   - everything else → verse (short couplet lines)
	   blank lines between groups produce an afterGap flag for visual spacing.
	*/
	type Segment =
		| { kind: 'heading'; text: string; afterGap?: boolean }
		| { kind: 'verse'; text: string; afterGap?: boolean }
		| { kind: 'explanation'; text: string; afterGap?: boolean }
		| { kind: 'paragraph'; text: string; afterGap?: boolean };

	let segments: Segment[] = $derived.by(() => {
		const lines = text.split('\n').map((l) => l.trim());

		// collect consecutive non-blank lines into groups
		const groups: string[][] = [];
		let current: string[] = [];
		for (const line of lines) {
			if (line.length === 0) {
				if (current.length > 0) {
					groups.push(current);
					current = [];
				}
			} else {
				current.push(line);
			}
		}
		if (current.length > 0) groups.push(current);

		const result: Segment[] = [];

		for (let g = 0; g < groups.length; g++) {
			const group = groups[g];
			const afterGap = g > 1; // gap after first group (heading) doesn't count

			for (let j = 0; j < group.length; j++) {
				const line = group[j];
				const isFirstInGroup = j === 0;
				const gap = afterGap && isFirstInGroup;

				if (g === 0) {
					// first group → heading
					result.push({ kind: 'heading', text: line, ...(gap ? { afterGap: true } : {}) });
				} else if (line.startsWith('பொருள்:')) {
					result.push({ kind: 'explanation', text: line, ...(gap ? { afterGap: true } : {}) });
				} else if (group.length === 1 && line.length > 100) {
					// single long line → prose paragraph
					result.push({ kind: 'paragraph', text: line, ...(gap ? { afterGap: true } : {}) });
				} else {
					result.push({ kind: 'verse', text: line, ...(gap ? { afterGap: true } : {}) });
				}
			}
		}

		return result;
	});

	/* ── scroll progress ── */
	let scrollProgress = $state(0);
	let textContainer: HTMLElement | undefined = $state(undefined);

	function handleScroll() {
		if (!textContainer) return;
		const { scrollTop, scrollHeight, clientHeight } = textContainer;
		const maxScroll = scrollHeight - clientHeight;
		scrollProgress = maxScroll > 0 ? scrollTop / maxScroll : 0;
	}
</script>

<article class="reader" role="document" aria-label={title ?? 'Tamil reader'}>
	<!-- progress bar -->
	<div class="progress-track" aria-hidden="true">
		<div class="progress-fill" style="width: {scrollProgress * 100}%;"></div>
	</div>

	<!-- title -->
	{#if title}
		<header class="reader-header">
			<h2 class="reader-title">{title}</h2>
		</header>
	{/if}

	<!-- reading surface -->
	<div
		class="reading-surface"
		bind:this={textContainer}
		onscroll={handleScroll}
		style="font-family: {fontFamily}; font-size: {fontSize}px; line-height: {lineHeight}px;"
	>
		<div class="reading-content">
			{#each segments as seg, i (i)}
				{#if seg.kind === 'heading'}
					<p class="seg heading" class:after-gap={seg.afterGap}>{seg.text}</p>
				{:else if seg.kind === 'verse'}
					<p class="seg verse" class:after-gap={seg.afterGap}>{seg.text}</p>
				{:else if seg.kind === 'paragraph'}
					<p class="seg paragraph" class:after-gap={seg.afterGap}>{seg.text}</p>
				{:else}
					<p class="seg explanation" class:after-gap={seg.afterGap}>{seg.text}</p>
				{/if}
			{/each}

			<!-- end mark -->
			<div class="end-mark" aria-hidden="true">&#xB83;</div>
		</div>
	</div>

	<!-- controls — sticky bottom bar -->
	<div class="controls" role="toolbar" aria-label="Reading controls">
		<select class="font-select" bind:value={fontFamily} aria-label="Choose font">
			{#each FONTS as font}
				<option value={font.value}>{font.label}</option>
			{/each}
		</select>

		<button
			class="ctrl-btn"
			onclick={decrease}
			disabled={fontSize <= MIN_FONT}
			aria-label="Decrease font size"
		>
			<span class="ctrl-label">அ</span>
			<svg class="ctrl-icon" width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
				<line x1="2" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
			</svg>
		</button>

		<span class="size-label" aria-live="polite" aria-atomic="true">
			{fontSize}
		</span>

		<button
			class="ctrl-btn"
			onclick={increase}
			disabled={fontSize >= MAX_FONT}
			aria-label="Increase font size"
		>
			<span class="ctrl-label">அ</span>
			<svg class="ctrl-icon" width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
				<line x1="2" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
				<line x1="7" y1="2" x2="7" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
			</svg>
		</button>
	</div>
</article>

<style>
	/* ========================================
	   DESIGN TOKENS
	   ======================================== */
	.reader {
		--ink: #1a0e06;
		--ink-soft: #3a2a1a;
		--cream: #faf3e6;
		--cream-mid: #f0e4cc;
		--red: #8b1a1a;
		--red-deep: #5c0e0e;
		--red-faint: rgba(139, 26, 26, 0.08);
		--gold: #c5941a;
		--gold-dim: rgba(197, 148, 26, 0.3);
		--stone: #6b5a48;

		font-family: 'Catamaran', sans-serif;
		background: var(--cream);
		max-width: 640px;
		width: 100%;
		margin: 0 auto;
		position: relative;
		display: flex;
		flex-direction: column;
		height: 100dvh;
	}

	/* ========================================
	   PROGRESS BAR
	   ======================================== */
	.progress-track {
		position: sticky;
		top: 0;
		z-index: 10;
		height: 3px;
		background: var(--cream-mid);
		flex-shrink: 0;
	}

	.progress-fill {
		height: 100%;
		background: var(--gold);
		transition: width 0.12s ease-out;
		will-change: width;
	}

	/* ========================================
	   HEADER
	   ======================================== */
	.reader-header {
		padding: 24px 24px 20px;
		border-bottom: 1px solid var(--cream-mid);
		flex-shrink: 0;
	}

	.reader-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 17px;
		font-weight: 700;
		color: var(--red-deep);
		margin: 0;
		line-height: 1.5;
		text-align: center;
	}

	/* ========================================
	   READING SURFACE
	   ======================================== */
	.reading-surface {
		flex: 1;
		padding: 24px 24px 32px;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		overscroll-behavior: contain;
		color: var(--ink);
		text-align: left;
		transition: font-size 0.2s ease, line-height 0.2s ease;
		scrollbar-width: none;
	}

	.reading-content {
		width: 100%;
	}

	.reading-surface::-webkit-scrollbar {
		display: none;
	}

	@media (min-width: 600px) {
		.reading-surface {
			padding: 32px 36px 40px;
		}
	}

	/* ========================================
	   TEXT SEGMENTS — visual hierarchy
	   ======================================== */
	.seg {
		margin: 0;
		overflow-wrap: break-word;
	}

	/* section heading */
	.heading {
		font-family: 'Catamaran', sans-serif;
		font-size: 0.75em;
		font-weight: 700;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin-bottom: 1.8em;
		padding-bottom: 0.8em;
		border-bottom: 1px solid var(--cream-mid);
		text-align: center;
	}

	/* verse couplet lines — the star of the show */
	.verse {
		font-weight: 500;
		color: var(--ink);
		margin-bottom: 0.3em;
		padding-left: 0.8em;
		border-left: 2px solid var(--gold-dim);
	}

	/* when a verse is followed by an explanation, add space after the last verse line.
	   we detect this by adding extra bottom margin to verse lines that precede explanations.
	   since CSS can't look ahead, we add space before explanations instead. */
	.explanation {
		font-size: 0.88em;
		color: var(--ink-soft);
		line-height: inherit;
		margin-top: 0.6em;
		margin-bottom: 2em;
		padding-left: 0.8em;
	}

	/* last explanation doesn't need bottom margin */
	.explanation:last-of-type {
		margin-bottom: 1em;
	}

	/* prose paragraph — no gold border, proper spacing */
	.paragraph {
		margin-bottom: 1.2em;
	}

	/* gap between blank-line-separated groups */
	.after-gap {
		margin-top: 1.6em;
	}

	/* first segment after heading doesn't need gap */
	.heading + .after-gap {
		margin-top: 0;
	}

	/* ========================================
	   END MARK
	   ======================================== */
	.end-mark {
		text-align: center;
		font-size: 20px;
		color: var(--gold-dim);
		padding: 16px 0 0;
		user-select: none;
	}

	/* ========================================
	   CONTROLS
	   ======================================== */
	.controls {
		position: sticky;
		bottom: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 8px 24px;
		padding-bottom: max(8px, env(safe-area-inset-bottom));
		background: var(--cream);
		border-top: 1px solid var(--cream-mid);
		flex-shrink: 0;
		box-shadow: 0 -4px 20px rgba(26, 14, 6, 0.05);
	}

	.ctrl-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2px;
		min-width: 56px;
		height: 48px;
		padding: 0 12px;
		border-radius: 12px;
		border: 1.5px solid var(--cream-mid);
		background: transparent;
		color: var(--red);
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.ctrl-btn:hover:not(:disabled) {
		background: var(--red-faint);
		border-color: var(--red);
	}

	.ctrl-btn:active:not(:disabled) {
		background: rgba(139, 26, 26, 0.14);
		transform: scale(0.96);
	}

	.ctrl-btn:disabled {
		opacity: 0.2;
		cursor: not-allowed;
	}

	.ctrl-label {
		font-family: 'Catamaran', sans-serif;
		font-size: 18px;
		font-weight: 600;
		line-height: 1;
	}

	.ctrl-icon {
		flex-shrink: 0;
	}

	.size-label {
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 600;
		color: var(--stone);
		min-width: 32px;
		text-align: center;
		font-variant-numeric: tabular-nums;
		user-select: none;
	}

	.font-select {
		flex: 1;
		min-width: 0;
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 600;
		color: var(--red);
		background: transparent;
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		padding: 0 10px;
		height: 48px;
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
		outline: none;
	}

	.font-select:hover {
		background: var(--red-faint);
		border-color: var(--red);
	}

	.font-select:focus-visible {
		border-color: var(--red);
	}

	/* ========================================
	   RESPONSIVE
	   ======================================== */
	@media (min-width: 600px) {
		.reader {
			min-height: auto;
			border-radius: 4px;
			border: 1.5px solid var(--cream-mid);
			box-shadow: 0 8px 32px rgba(26, 14, 6, 0.06);
		}

		.reader-header {
			padding: 28px 36px 24px;
		}

		.reader-title {
			font-size: 19px;
		}

		.reading-surface {
			max-height: 78vh;
		}

		.controls {
			position: static;
			box-shadow: none;
			padding: 12px 36px;
		}
	}
</style>
