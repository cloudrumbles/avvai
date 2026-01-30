<script lang="ts">
	import ClickableText from 'avvai-frontend/components/ClickableText';

	interface ProseSectionData {
		title?: string;
		paragraphs: string[];
	}

	interface Props {
		data: ProseSectionData;
	}

	let { data }: Props = $props();
</script>

<section class="prose-section">
	{#if data.title}
		<h2 class="section-title">{data.title}</h2>
	{/if}

	{#each data.paragraphs as paragraph, i (i)}
		<p class="paragraph">
			<ClickableText text={paragraph} />
		</p>
	{/each}
</section>

<style>
	.prose-section {
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
</style>
