<script lang="ts">
	import { TAMIL_FONTS, MIN_FONT_SIZE, MAX_FONT_SIZE, FONT_SIZE_STEP } from '$lib/config/fonts';
	import IconButton from './IconButton.svelte';

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

			<label class="menu-label">Size</label>
			<div class="size-controls">
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
		top: calc(100% + 8px);
		right: 0;
		z-index: 51;
		min-width: 200px;
		padding: 12px;
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		box-shadow: 0 8px 32px rgba(26, 14, 6, 0.12);
	}

	.menu-label {
		display: block;
		font-family: 'Catamaran', sans-serif;
		font-size: 11px;
		font-weight: 600;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 6px;
	}

	.menu-label:not(:first-child) {
		margin-top: 12px;
	}

	.font-select {
		width: 100%;
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 600;
		color: var(--red);
		background: transparent;
		border: 1.5px solid var(--cream-mid);
		border-radius: 8px;
		padding: 0 10px;
		height: 36px;
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

	.size-controls {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.ctrl-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: 8px;
		border: 1.5px solid var(--cream-mid);
		background: transparent;
		color: var(--red);
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
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

	.size-label {
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		font-weight: 600;
		color: var(--ink);
		min-width: 28px;
		text-align: center;
		font-variant-numeric: tabular-nums;
	}
</style>
