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
		color: var(--color-text);
	}

	.section-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-4);
		font-weight: 700;
		color: var(--color-accent);
		margin: 0 0 var(--space-5);
	}

	.paragraph {
		margin: 0 0 var(--space-6);
		line-height: var(--line-height-2);
		overflow-wrap: break-word;
		text-indent: var(--space-6);
	}

	.paragraph:last-child {
		margin-bottom: 0;
	}
</style>