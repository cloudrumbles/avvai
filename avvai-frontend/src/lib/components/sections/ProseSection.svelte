<script lang="ts">
	interface ProseSectionData {
		title?: string;
		paragraphs: string[];
	}

	interface Props {
		data: ProseSectionData;
		onwordclick?: (word: string, event: MouseEvent) => void;
	}

	let { data, onwordclick }: Props = $props();
</script>

<section class="prose-section">
	{#if data.title}
		<h2 class="section-title">{data.title}</h2>
	{/if}

	{#each data.paragraphs as paragraph, i (i)}
		<p class="paragraph">
			{#each paragraph.split(/(\s+)/) as token, ti (ti)}
				{#if /^\s+$/.test(token)}
					{token}
				{:else}
					<span
						class="word"
						role="button"
						tabindex="-1"
						onclick={(e) => onwordclick?.(token, e)}
						onkeydown={(e) => { if (e.key === 'Enter') onwordclick?.(token, e as unknown as MouseEvent); }}
					>{token}</span>
				{/if}
			{/each}
		</p>
	{/each}
</section>

<style>
	.prose-section {
		--ink: #1a0e06;
		--cream-mid: #f0e4cc;
		--red: #8b1a1a;
		--stone: #6b5a48;
		--red-faint: rgba(139, 26, 26, 0.08);

		color: var(--ink);
	}

	.section-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 1.1em;
		font-weight: 700;
		color: var(--red);
		margin: 0 0 1.2em;
	}

	.paragraph {
		margin: 0 0 1.5em;
		line-height: 2;
		overflow-wrap: break-word;
		text-indent: 1.5em;
	}

	.paragraph:last-child {
		margin-bottom: 0;
	}

	.word {
		cursor: pointer;
		border-radius: 3px;
		transition: background 0.1s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.word:hover {
		background: var(--red-faint);
	}

	.word:active {
		background: rgba(139, 26, 26, 0.14);
	}
</style>
