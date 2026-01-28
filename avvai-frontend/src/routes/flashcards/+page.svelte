<script lang="ts">
	interface Card {
		id: string;
		front: string;
		back: string;
	}

	// Mock flashcard data - will come from backend
	const cards: Card[] = [
		{ id: '1', front: 'அகரம்', back: 'The letter "A" - the first letter of Tamil alphabet' },
		{ id: '2', front: 'முதல்', back: 'First, beginning, origin' },
		{ id: '3', front: 'எழுத்து', back: 'Letter, character, writing' },
		{ id: '4', front: 'ஆதி', back: 'Beginning, origin, primordial' },
		{ id: '5', front: 'உலகு', back: 'World, earth' },
	];

	let currentIndex = $state(0);
	let flipped = $state(false);
	let completed = $state<string[]>([]);

	let currentCard = $derived(cards[currentIndex]);
	let progress = $derived(Math.round((completed.length / cards.length) * 100));

	function flip() {
		flipped = !flipped;
	}

	function next(known: boolean) {
		if (known && !completed.includes(currentCard.id)) {
			completed = [...completed, currentCard.id];
		}
		flipped = false;
		if (currentIndex < cards.length - 1) {
			currentIndex++;
		} else {
			currentIndex = 0;
		}
	}

	function reset() {
		currentIndex = 0;
		completed = [];
		flipped = false;
	}
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
			<button class="action-btn again" onclick={() => next(false)}>
				<span class="action-icon">✗</span>
				<span>Again</span>
			</button>
			<button class="action-btn good" onclick={() => next(true)}>
				<span class="action-icon">✓</span>
				<span>Good</span>
			</button>
		</div>
	{/if}

	{#if completed.length === cards.length}
		<div class="complete-message">
			<p>🎉 All cards reviewed!</p>
			<button class="reset-btn" onclick={reset}>Review again</button>
		</div>
	{/if}
</div>

<style>
	.flashcards {
		--ink: #1a0e06;
		--ink-soft: #3a2a1a;
		--cream: #faf3e6;
		--cream-mid: #f0e4cc;
		--red: #8b1a1a;
		--red-deep: #5c0e0e;
		--red-faint: rgba(139, 26, 26, 0.08);
		--gold: #c5941a;
		--stone: #6b5a48;
		--green: #2d6a4f;

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

	.action-btn.good:hover {
		background: rgba(45, 106, 79, 0.15);
	}

	.action-icon {
		font-size: 20px;
	}

	.complete-message {
		text-align: center;
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
