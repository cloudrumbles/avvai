<script lang="ts">
	import { showDictionary } from 'avvai-frontend/stores/dictionary';

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

	function handleWordClick(word: string, event: MouseEvent) {
		const rect = (event.target as HTMLElement).getBoundingClientRect();
		showDictionary(word, {
			x: rect.left + rect.width / 2,
			y: rect.top,
			bottom: rect.bottom
		});
	}

	function handleKeydown(word: string, event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleWordClick(word, event as unknown as MouseEvent);
		}
	}
</script>

<section class="vocabulary-section">
	{#if data.title}
		<h2 class="section-title">{data.title}</h2>
	{/if}

	<dl class="definition-list">
		{#each data.entries as entry, i (i)}
			<div class="entry">
				<dt class="word">
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<span
						class="word-text interactive-text"
						role="button"
						tabindex="-1"
						onclick={(e) => handleWordClick(entry.word, e)}
						onkeydown={(e) => handleKeydown(entry.word, e)}
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

	.section-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-1);
		font-weight: 700;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-wide);
		margin: 0 0 var(--space-7);
		padding-bottom: var(--space-3);
		border-bottom: 1px solid var(--color-bg-soft);
		text-align: center;
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