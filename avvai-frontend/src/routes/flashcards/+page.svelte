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
		<a href="/home" class="back-btn" aria-label="Back to home">
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
			<button class="reset-btn" disabled={settingsSaving} onclick={saveSettings}>
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
			<button class="reset-btn" onclick={reset}>Try again</button>
		</div>
	{:else if !currentCard}
		<div class="empty">
			<p>No flashcards due right now.</p>
			<button class="reset-btn" onclick={reset}>Refresh</button>
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
				<button class="reset-btn" onclick={reset}>Review again</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.flashcards {
		min-height: 100dvh;
		background: var(--cream);
		font-family: 'Catamaran', sans-serif;
		display: flex;
		flex-direction: column;
	}

	.header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
	}

	.back-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: 8px;
		color: var(--stone);
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.back-btn:hover {
		background: var(--red-faint);
		color: var(--red);
	}

	.header h1 {
		flex: 1;
		font-size: 17px;
		font-weight: 600;
		color: var(--red-deep);
		margin: 0;
	}

	.progress-text {
		font-size: 14px;
		font-weight: 600;
		color: var(--stone);
	}

	.progress-bar {
		height: 4px;
		background: var(--cream-mid);
	}

	.progress-fill {
		height: 100%;
		background: var(--gold);
		transition: width 0.3s ease;
	}

	.settings {
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding: 12px 16px;
		border-bottom: 1px solid var(--cream-mid);
	}

	.settings-label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		font-weight: 600;
		color: var(--stone);
	}

	.settings-controls {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.settings-controls input[type='range'] {
		flex: 1;
	}

	.settings-value {
		font-size: 14px;
		font-weight: 600;
		color: var(--ink);
		min-width: 36px;
		text-align: right;
	}

	.settings-error {
		font-size: 13px;
		color: var(--red);
		margin: 0;
	}

	.tooltip {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: var(--cream-mid);
		color: var(--stone);
		font-size: 11px;
		font-weight: 700;
		cursor: help;
	}

	.tooltip-text {
		position: absolute;
		left: 50%;
		bottom: 140%;
		transform: translateX(-50%);
		width: 220px;
		background: var(--ink);
		color: var(--cream);
		padding: 10px 12px;
		border-radius: 10px;
		font-size: 12px;
		font-weight: 400;
		line-height: 1.4;
		opacity: 0;
		pointer-events: none;
		transition: opacity 0.15s ease;
		box-shadow: 0 12px 24px rgba(26, 14, 6, 0.15);
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
		padding: 24px;
		perspective: 1000px;
	}

	.card {
		width: 100%;
		max-width: 320px;
		aspect-ratio: 3/4;
		position: relative;
		transform-style: preserve-3d;
		transition: transform 0.5s ease;
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
		padding: 24px;
		background: var(--cream);
		border: 2px solid var(--cream-mid);
		border-radius: 16px;
		backface-visibility: hidden;
		box-shadow: 0 8px 32px rgba(26, 14, 6, 0.08);
	}

	.card-back {
		transform: rotateY(180deg);
	}

	.card-text {
		font-size: 24px;
		font-weight: 600;
		color: var(--ink);
		text-align: center;
		margin: 0;
		line-height: 1.4;
	}

	.card-back .card-text {
		font-size: 18px;
		font-weight: 400;
	}

	.tap-hint {
		position: absolute;
		bottom: 24px;
		font-size: 13px;
		color: var(--stone);
	}

	.actions {
		display: flex;
		gap: 16px;
		padding: 24px;
		justify-content: center;
	}

	.action-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		padding: 16px 32px;
		border: none;
		border-radius: 12px;
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn.again {
		background: var(--red-faint);
		color: var(--red);
	}

	.action-btn.again:hover {
		background: rgba(139, 26, 26, 0.15);
	}

	.action-btn.good {
		background: rgba(45, 106, 79, 0.1);
		color: var(--green);
	}

	.action-btn.hard {
		background: rgba(197, 148, 26, 0.12);
		color: var(--gold);
	}

	.action-btn.easy {
		background: rgba(73, 86, 166, 0.12);
		color: var(--purple);
	}

	.action-btn.good:hover {
		background: rgba(45, 106, 79, 0.15);
	}

	.action-btn.hard:hover {
		background: rgba(197, 148, 26, 0.2);
	}

	.action-btn.easy:hover {
		background: rgba(73, 86, 166, 0.2);
	}

	.action-icon {
		font-size: 20px;
	}

	.complete-message {
		text-align: center;
		padding: 24px;
	}

	.loading,
	.error,
	.empty {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		text-align: center;
		color: var(--stone);
		padding: 24px;
	}

	.complete-message p {
		font-size: 18px;
		color: var(--ink);
		margin: 0 0 16px;
	}

	.reset-btn {
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		font-weight: 600;
		color: var(--gold);
		background: transparent;
		border: 1.5px solid var(--cream-mid);
		border-radius: 8px;
		padding: 10px 20px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.reset-btn:hover {
		border-color: var(--gold);
		background: rgba(197, 148, 26, 0.1);
	}
</style>
