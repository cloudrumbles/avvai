<script lang="ts">
	import type { Flashcard, ReviewResponse, FlashcardSettings } from '$lib/types/flashcard';

	let cards = $state<Flashcard[]>([]);
	let currentIndex = $state(0);
	let flipped = $state(false);
	let completed = $state<string[]>([]);
	let loading = $state(true);
	let errorMessage = $state<string | null>(null);
	let desiredRetention = $state(0.9);
	let settingsSaving = $state(false);
	let settingsError = $state<string | null>(null);

	let currentCard = $derived(cards[currentIndex]);
	let progress = $derived(cards.length ? Math.round((completed.length / cards.length) * 100) : 0);

	async function loadCards() {
		loading = true;
		errorMessage = null;
		try {
			const res = await fetch('/api/flashcards?limit=30');
			if (!res.ok) {
				throw new Error('Failed to load flashcards');
			}
			cards = await res.json();
			currentIndex = 0;
			completed = [];
			flipped = false;
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Something went wrong';
		} finally {
			loading = false;
		}
	}

	async function loadSettings() {
		settingsError = null;
		try {
			const res = await fetch('/api/flashcards/settings');
			if (!res.ok) {
				throw new Error('Failed to load settings');
			}
			const data: FlashcardSettings = await res.json();
			desiredRetention = data.desired_retention;
		} catch (err) {
			settingsError = err instanceof Error ? err.message : 'Something went wrong';
		}
	}

	async function saveSettings() {
		settingsSaving = true;
		settingsError = null;
		try {
			const res = await fetch('/api/flashcards/settings', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ desiredRetention })
			});
			if (!res.ok) {
				throw new Error('Failed to update settings');
			}
			await loadCards();
		} catch (err) {
			settingsError = err instanceof Error ? err.message : 'Something went wrong';
		} finally {
			settingsSaving = false;
		}
	}

	function flip() {
		if (!currentCard) return;
		flipped = !flipped;
	}

	async function review(rating: number) {
		if (!currentCard) return;
		try {
			const res = await fetch('/api/flashcards', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ cardId: currentCard.id, rating })
			});
			if (!res.ok) {
				throw new Error('Failed to save review');
			}
			const data: ReviewResponse = await res.json();
			if (!data.success) {
				throw new Error('Review update failed');
			}
			if (!completed.includes(currentCard.id)) {
				completed = [...completed, currentCard.id];
			}
			flipped = false;
			if (currentIndex < cards.length - 1) {
				currentIndex++;
			} else {
				currentIndex = 0;
			}
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Something went wrong';
		}
	}

	function reset() {
		loadCards();
	}

	$effect(() => {
		loadCards();
		loadSettings();
	});
</script>

<div class="flashcards">
	<header class="header">
		<a href="/home" class="btn-icon" aria-label="Back to home">
			<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
				<path d="M12 4L6 10L12 16" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
			</svg>
		</a>
		<h1>Review</h1>
		<span class="progress-text">{completed.length}/{cards.length}</span>
	</header>

	<div class="progress-bar">
		<div class="progress-fill" style="width: {progress}%"></div>
	</div>

	<div class="settings">
		<div class="settings-label">
			<span>Desired retention</span>
			<span class="tooltip" aria-label="Retention controls how likely you should remember a card when it is shown again. Higher values mean shorter intervals and more frequent reviews.">?
				<span class="tooltip-text">
					Retention controls how likely you should remember a card when it is shown again. Higher values mean shorter intervals and more frequent reviews.
				</span>
			</span>
		</div>
		<div class="settings-controls">
			<input
				id="retention"
				type="range"
				min="0.7"
				max="0.99"
				step="0.01"
				bind:value={desiredRetention}
			/>
			<span class="settings-value">{desiredRetention.toFixed(2)}</span>
			<button class="btn btn-secondary" disabled={settingsSaving} onclick={saveSettings}>
				{settingsSaving ? 'Saving…' : 'Save'}
			</button>
		</div>
		{#if settingsError}
			<p class="settings-error">{settingsError}</p>
		{/if}
	</div>

	{#if loading}
		<div class="loading">Loading flashcards…</div>
	{:else if errorMessage}
		<div class="error">
			<p>{errorMessage}</p>
			<button class="btn btn-secondary" onclick={reset}>Try again</button>
		</div>
	{:else if !currentCard}
		<div class="empty">
			<p>No flashcards due right now.</p>
			<button class="btn btn-secondary" onclick={reset}>Refresh</button>
		</div>
	{:else}
		<div class="card-container">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<div class="card" class:flipped onclick={flip}>
				<div class="card-face card-front">
					<p class="card-text">{currentCard.front}</p>
					<span class="tap-hint">Tap to reveal</span>
				</div>
				<div class="card-face card-back">
					<p class="card-text">{currentCard.back}</p>
				</div>
			</div>
		</div>

		{#if flipped}
			<div class="actions">
				<button class="action-btn again" onclick={() => review(1)}>
					<span class="action-icon">✗</span>
					<span>Again</span>
				</button>
				<button class="action-btn hard" onclick={() => review(2)}>
					<span class="action-icon">!</span>
					<span>Hard</span>
				</button>
				<button class="action-btn good" onclick={() => review(3)}>
					<span class="action-icon">✓</span>
					<span>Good</span>
				</button>
				<button class="action-btn easy" onclick={() => review(4)}>
					<span class="action-icon">★</span>
					<span>Easy</span>
				</button>
			</div>
		{/if}

		{#if completed.length === cards.length && cards.length > 0}
			<div class="complete-message">
				<p>🎉 All cards reviewed!</p>
				<button class="btn btn-primary" onclick={reset}>Review again</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.flashcards {
		min-height: 100dvh;
		background: var(--color-bg);
		font-family: var(--font-sans);
		display: flex;
		flex-direction: column;
	}

	.header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
	}

	.header h1 {
		flex: 1;
		font-size: var(--font-size-3-5);
		font-weight: 600;
		color: var(--color-accent-strong);
		margin: 0;
	}

	.progress-text {
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-text-subtle);
	}

	.progress-bar {
		height: 4px;
		background: var(--color-bg-soft);
	}

	.progress-fill {
		height: 100%;
		background: var(--color-accent);
		transition: width var(--duration-medium) var(--ease-standard);
	}

	.settings {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		border-bottom: var(--border-1);
	}

	.settings-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--font-size-1-5);
		font-weight: 600;
		color: var(--color-text-subtle);
	}

	.settings-controls {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.settings-controls input[type='range'] {
		flex: 1;
		height: 4px;
		background: var(--color-bg-soft);
		border-radius: var(--radius-pill);
		appearance: none;
		cursor: pointer;
	}

	.settings-controls input[type='range']::-webkit-slider-thumb {
		appearance: none;
		width: 16px;
		height: 16px;
		background: var(--color-accent);
		border-radius: var(--radius-pill);
		border: 2px solid var(--color-bg);
		box-shadow: var(--shadow-1);
		cursor: pointer;
		transition: transform var(--duration-fast) var(--ease-standard);
	}

	.settings-controls input[type='range']::-webkit-slider-thumb:hover {
		transform: scale(1.1);
	}

	.settings-controls input[type='range']::-moz-range-thumb {
		width: 16px;
		height: 16px;
		background: var(--color-accent);
		border-radius: var(--radius-pill);
		border: 2px solid var(--color-bg);
		box-shadow: var(--shadow-1);
		cursor: pointer;
	}

	.settings-value {
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-text);
		min-width: 36px;
		text-align: right;
	}

	.settings-error {
		font-size: var(--font-size-1-5);
		color: var(--color-accent);
		margin: 0;
	}

	.tooltip {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		border-radius: var(--radius-pill);
		background: var(--color-bg-soft);
		color: var(--color-text-subtle);
		font-size: var(--font-size-0-75);
		font-weight: 700;
		cursor: help;
	}

	.tooltip-text {
		position: absolute;
		left: 50%;
		bottom: 140%;
		transform: translateX(-50%);
		width: 220px;
		background: var(--color-bg-inverse);
		color: var(--color-text-inverse);
		padding: var(--space-2-5) var(--space-3);
		border-radius: var(--radius-2);
		font-size: var(--font-size-1);
		font-weight: 400;
		line-height: var(--line-height-1-4);
		opacity: 0;
		pointer-events: none;
		transition: opacity var(--duration-fast) var(--ease-standard);
		box-shadow: var(--shadow-2);
		z-index: 2;
	}

	.tooltip:hover .tooltip-text,
	.tooltip:focus-within .tooltip-text {
		opacity: 1;
	}

	.card-container {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-6);
		perspective: 1000px;
	}

	.card {
		width: 100%;
		max-width: 320px;
		aspect-ratio: 3/4;
		position: relative;
		transform-style: preserve-3d;
		transition: transform var(--duration-slow) var(--ease-standard);
		cursor: pointer;
	}

	.card.flipped {
		transform: rotateY(180deg);
	}

	.card-face {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-6);
		background: var(--color-bg);
		border: var(--border-strong);
		border-radius: var(--radius-3);
		backface-visibility: hidden;
		box-shadow: var(--shadow-2);
	}

	.card-back {
		transform: rotateY(180deg);
	}

	.card-text {
		font-family: var(--font-serif);
		font-size: var(--font-size-5-5);
		font-weight: 600;
		color: var(--color-text);
		text-align: center;
		margin: 0;
		line-height: var(--line-height-1-4);
	}

	.card-back .card-text {
		font-size: var(--font-size-4);
		font-weight: 400;
	}

	.tap-hint {
		position: absolute;
		bottom: var(--space-6);
		font-size: var(--font-size-1-5);
		color: var(--color-text-subtle);
	}

	.actions {
		display: flex;
		gap: var(--space-3);
		padding: var(--space-6);
		justify-content: center;
	}

	.action-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-3) var(--space-5);
		border: none;
		border-radius: var(--radius-2);
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 600;
		cursor: pointer;
		transition: transform var(--duration-fast) var(--ease-standard),
			background var(--duration-fast) var(--ease-standard);
	}

	.action-btn:active {
		transform: scale(0.96);
	}

	.action-btn.again {
		background: var(--color-accent-tint);
		color: var(--color-accent);
	}

	.action-btn.again:hover {
		background: var(--overlay-red-soft);
	}

	.action-btn.good {
		background: var(--color-success-tint);
		color: var(--color-success);
	}

	.action-btn.good:hover {
		background: rgba(26, 107, 74, 0.15);
	}

	.action-btn.hard {
		background: var(--color-bg-soft);
		color: var(--color-highlight);
	}

	.action-btn.hard:hover {
		background: var(--color-bg-pressed);
	}

	.action-btn.easy {
		background: var(--color-accent-secondary-tint);
		color: var(--color-accent-secondary);
	}

	.action-btn.easy:hover {
		background: rgba(196, 92, 62, 0.18);
	}

	.action-icon {
		font-size: var(--font-size-4-5);
	}

	.complete-message {
		text-align: center;
		padding: var(--space-6);
	}

	.loading,
	.error,
	.empty {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		text-align: center;
		color: var(--color-text-subtle);
		padding: var(--space-6);
	}

	.complete-message p {
		font-family: var(--font-display);
		font-size: var(--font-size-5);
		color: var(--color-text);
		margin: 0 0 var(--space-4);
	}
</style>