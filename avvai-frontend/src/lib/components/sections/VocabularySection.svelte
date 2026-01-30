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
						class="word-text"
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
		font-family: 'Catamaran', sans-serif;
		color: var(--ink);
		background: var(--cream);
		border: 2px solid var(--cream-mid);
		border-radius: 12px;
		padding: 28px 32px;
		box-shadow: inset 0 2px 8px rgba(26, 14, 6, 0.03);
	}

	@media (max-width: 640px) {
		.vocabulary-section {
			padding: 20px 24px;
			border-radius: 8px;
		}
	}

	.section-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 0.75em;
		font-weight: 700;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0 0 1.8em;
		padding-bottom: 0.8em;
		border-bottom: 1px solid var(--cream-mid);
		text-align: center;
	}

	.definition-list {
		margin: 0;
		padding: 0;
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}

	@media (max-width: 640px) {
		.definition-list {
			grid-template-columns: 1fr;
		}
	}

	.entry {
		display: flex;
		flex-direction: column;
		gap: 8px;
		padding: 16px 20px;
		background: linear-gradient(135deg, var(--cream) 0%, var(--red-subtle) 100%);
		border: 1px solid var(--cream-mid);
		border-radius: 10px;
		border-left: 3px solid var(--red);
		transition: all 0.15s ease;
	}

	.entry:hover {
		border-color: var(--red);
		box-shadow: 0 2px 8px rgba(139, 26, 26, 0.08);
	}

	.word {
		margin: 0;
		font-weight: 700;
		color: var(--red-deep);
		font-size: 1.1em;
	}

	.word-text {
		cursor: pointer;
		border-radius: 3px;
		padding: 0.1em 0.2em;
		margin: -0.1em -0.2em;
		transition: background 0.1s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.word-text:hover {
		background: var(--red-faint);
	}

	.word-text:active {
		background: rgba(139, 26, 26, 0.14);
	}

	.meaning {
		margin: 0;
		color: var(--ink);
		line-height: 1.6;
	}
</style>
