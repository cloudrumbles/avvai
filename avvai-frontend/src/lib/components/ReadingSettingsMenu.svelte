<script lang="ts">
	import { TAMIL_FONTS, MIN_FONT_SIZE, MAX_FONT_SIZE, FONT_SIZE_STEP } from '$lib/config/fonts';
	import IconButton from '$lib/components/IconButton.svelte';

	interface Props {
		fontFamily: string;
		fontSize: number;
		onfontchange: (value: string) => void;
		onsizechange: (value: number) => void;
	}

	let { fontFamily, fontSize, onfontchange, onsizechange }: Props = $props();

	let open = $state(false);

	function increase() {
		if (fontSize < MAX_FONT_SIZE) {
			onsizechange(fontSize + FONT_SIZE_STEP);
		}
	}

	function decrease() {
		if (fontSize > MIN_FONT_SIZE) {
			onsizechange(fontSize - FONT_SIZE_STEP);
		}
	}
</script>

<div class="menu-container">
	<IconButton label="Reading settings" expanded={open} onclick={() => open = !open}>
		<svg width="20" height="20" viewBox="0 0 20 20" fill="none" aria-hidden="true">
			<circle cx="10" cy="4" r="1.5" fill="currentColor"/>
			<circle cx="10" cy="10" r="1.5" fill="currentColor"/>
			<circle cx="10" cy="16" r="1.5" fill="currentColor"/>
		</svg>
	</IconButton>

	{#if open}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div class="menu-backdrop" onclick={() => open = false}></div>
		<div class="menu-dropdown" role="menu">
			<label class="menu-label" for="font-select">Font</label>
			<select
				id="font-select"
				class="font-select"
				value={fontFamily}
				onchange={(e) => onfontchange(e.currentTarget.value)}
			>
				{#each TAMIL_FONTS as font (font.value)}
					<option value={font.value}>{font.label}</option>
				{/each}
			</select>

			<label class="menu-label" id="size-label">Size</label>
			<div class="size-controls" role="group" aria-labelledby="size-label">
				<button
					class="ctrl-btn"
					onclick={decrease}
					disabled={fontSize <= MIN_FONT_SIZE}
					aria-label="Decrease font size"
				>
					<svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
						<line x1="2" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
					</svg>
				</button>

				<span class="size-label" aria-live="polite">{fontSize}</span>

				<button
					class="ctrl-btn"
					onclick={increase}
					disabled={fontSize >= MAX_FONT_SIZE}
					aria-label="Increase font size"
				>
					<svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
						<line x1="2" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
						<line x1="7" y1="2" x2="7" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
					</svg>
				</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.menu-container {
		position: relative;
	}

	.menu-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
	}

	.menu-dropdown {
		position: absolute;
		top: calc(100% + var(--space-2));
		right: 0;
		z-index: 51;
		min-width: 200px;
		padding: var(--space-3);
		background: var(--color-bg);
		border: var(--border-strong);
		border-radius: var(--radius-3);
		box-shadow: var(--shadow-2);
	}

	.menu-label {
		display: block;
		font-family: var(--font-sans);
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-wide);
		margin-bottom: var(--space-2);
	}

	.menu-label:not(:first-child) {
		margin-top: var(--space-3);
	}

	.font-select {
		width: 100%;
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-accent);
		background: transparent;
		border: var(--border-strong);
		border-radius: var(--radius-2);
		padding: 0 var(--space-3);
		height: var(--size-icon-btn);
		cursor: pointer;
		transition: background var(--duration-fast) var(--ease-standard),
			border-color var(--duration-fast) var(--ease-standard),
			color var(--duration-fast) var(--ease-standard);
		-webkit-tap-highlight-color: transparent;
		outline: none;
	}

	.font-select:hover {
		background: var(--color-accent-tint);
		border-color: var(--color-accent);
	}

	.font-select:focus-visible {
		border-color: var(--color-accent);
	}

	.size-controls {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.ctrl-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--size-icon-btn);
		height: var(--size-icon-btn);
		border-radius: var(--radius-2);
		border: var(--border-strong);
		background: transparent;
		color: var(--color-accent);
		cursor: pointer;
		transition: transform var(--duration-fast) var(--ease-standard),
			background var(--duration-fast) var(--ease-standard),
			border-color var(--duration-fast) var(--ease-standard),
			color var(--duration-fast) var(--ease-standard);
		-webkit-tap-highlight-color: transparent;
	}

	.ctrl-btn:hover:not(:disabled) {
		background: var(--color-accent-tint);
		border-color: var(--color-accent);
	}

	.ctrl-btn:active:not(:disabled) {
		background: var(--overlay-red-soft);
		transform: scale(0.96);
	}

	.ctrl-btn:disabled {
		opacity: 0.2;
		cursor: not-allowed;
	}

	.size-label {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-text);
		min-width: 28px;
		text-align: center;
		font-variant-numeric: tabular-nums;
	}
</style>
