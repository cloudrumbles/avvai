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
		font-family: var(--font-sans);
		color: var(--color-text);
		background: var(--color-bg);
		border: 2px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-7) var(--space-7);
		box-shadow: var(--shadow-inset);
	}

	@media (max-width: 640px) {
		.vocabulary-section {
			padding: var(--space-5) var(--space-6);
			border-radius: var(--radius-2);
		}
	}

	.definition-list {
		margin: 0;
		padding: 0;
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
	}

	@media (max-width: 640px) {
		.definition-list {
			grid-template-columns: 1fr;
		}
	}

	.entry {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-4) var(--space-5);
		background: linear-gradient(135deg, var(--color-bg) 0%, var(--color-accent-tint) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-2-5);
		border-left: 3px solid var(--color-accent);
		transition: all 0.15s ease;
	}

	.entry:hover {
		border-color: var(--color-accent);
		box-shadow: var(--shadow-red);
	}

	.word {
		margin: 0;
		font-weight: 700;
		color: var(--color-accent-strong);
		font-size: var(--font-size-4);
	}

	.word-text {
		padding: 0.1em 0.2em;
		margin: -0.1em -0.2em;
	}

	.meaning {
		margin: 0;
		color: var(--color-text);
		line-height: var(--line-height-1-6);
	}
</style>
