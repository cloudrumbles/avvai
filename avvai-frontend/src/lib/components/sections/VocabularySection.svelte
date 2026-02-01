<script lang="ts">
	import { showDictionaryAtElement } from '$lib/actions/dictionary-lookup';

	interface VocabularyEntry {
		word: string;
		meaning: string;
	}

	interface VocabularySectionData {
		title?: string;
		entries: VocabularyEntry[];
	}

	interface Props {
		data: VocabularySectionData;
	}

	let { data }: Props = $props();

	let wordCount = $derived(data.entries.length);
	let activeIndex = $state(0);
	let wordElements = $state<(HTMLElement | undefined)[]>([]);
	let currentIndex = $derived.by(() => {
		if (wordCount === 0) return 0;
		return Math.max(0, Math.min(activeIndex, wordCount - 1));
	});

	function focusWord(index: number) {
		if (wordCount === 0) return;
		const clamped = Math.max(0, Math.min(index, wordCount - 1));
		activeIndex = clamped;
		const el = wordElements[clamped];
		if (el) el.focus();
	}

	function storeWordElement(index: number) {
		return (node: HTMLElement) => {
			wordElements[index] = node;
			return () => {
				if (wordElements[index] === node) {
					wordElements[index] = undefined;
				}
			};
		};
	}

	function handleWordClick(word: string, event: MouseEvent) {
		showDictionaryAtElement(word, event.currentTarget as HTMLElement | null);
	}

	function handleKeydown(word: string, wordIndex: number, event: KeyboardEvent) {
		switch (event.key) {
			case 'ArrowLeft':
			case 'ArrowUp':
				event.preventDefault();
				focusWord(wordIndex - 1);
				break;
			case 'ArrowRight':
			case 'ArrowDown':
				event.preventDefault();
				focusWord(wordIndex + 1);
				break;
			case 'Home':
				event.preventDefault();
				focusWord(0);
				break;
			case 'End':
				event.preventDefault();
				focusWord(wordCount - 1);
				break;
			case 'Enter':
			case ' ':
			case 'Spacebar':
				event.preventDefault();
				showDictionaryAtElement(word, event.currentTarget as HTMLElement | null);
				break;
		}
	}
</script>

<section class="vocabulary-section">
	{#if data.title}
		<h2 class="section-title section-title--size-1 section-title--subtle section-title--uppercase section-title--divider section-title--center section-title--mb-7">{data.title}</h2>
	{/if}

	<dl class="definition-list">
		{#each data.entries as entry, i (i)}
			<div class="entry">
				<dt class="word">
					<span
						class="word-text interactive-text"
						role="button"
						tabindex={i === currentIndex ? 0 : -1}
						{@attach storeWordElement(i)}
						onclick={(e) => {
							activeIndex = i;
							handleWordClick(entry.word, e);
						}}
						onfocus={() => {
							activeIndex = i;
						}}
						onkeydown={(e) => handleKeydown(entry.word, i, e)}
					>{entry.word}</span>
				</dt>
				<dd class="meaning">{entry.meaning}</dd>
			</div>
		{/each}
	</dl>
</section>

<style>
	.vocabulary-section {
		background: linear-gradient(180deg, var(--color-bg-soft) 0%, var(--color-bg) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-7) var(--space-7);
		box-shadow: var(--shadow-warm);
		position: relative;
	}

	.vocabulary-section::before {
		content: '';
		position: absolute;
		top: 0;
		left: var(--space-6);
		right: var(--space-6);
		height: 3px;
		background: linear-gradient(90deg, transparent, var(--color-highlight), transparent);
		border-radius: 0 0 var(--radius-0-5) var(--radius-0-5);
	}

	@media (max-width: 640px) {
		.vocabulary-section {
			padding: var(--space-5) var(--space-6);
			border-radius: var(--radius-2);
		}

		.vocabulary-section::before {
			left: var(--space-4);
			right: var(--space-4);
		}
	}

	.definition-list {
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-5);
	}

	.entry {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding-left: var(--space-3);
		border-left: 2px solid var(--color-bg-soft);
		padding-bottom: var(--space-5);
		border-bottom: 1px solid var(--color-bg-soft);
	}

	.entry:last-child {
		padding-bottom: 0;
		border-bottom: none;
	}

	.word {
		margin: 0;
		font-family: var(--font-serif);
		font-weight: 500;
		color: var(--color-text);
		font-size: var(--font-size-4);
	}

	.word-text {
		padding: 0.1em 0.2em;
		margin: -0.1em -0.2em;
	}

	.meaning {
		margin: 0;
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		line-height: var(--line-height-3);
	}
</style>
