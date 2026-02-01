<script lang="ts">
	import ClickableText from '$lib/components/ClickableText.svelte';

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
		<h2 class="section-title section-title--size-4 section-title--accent section-title--mb-5">{data.title}</h2>
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
