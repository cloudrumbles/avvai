<script lang="ts">
	import ClickableText from '$lib/components/ClickableText.svelte';

	interface Verse {
		number?: number;
		lines: string[];
		translation?: string;
	}

	interface Props {
		data: {
			title?: string;
			verses: Verse[];
		};
	}

	let { data }: Props = $props();
</script>

<section class="poetry-section">
	{#if data.title}
		<h2 class="section-title">{data.title}</h2>
	{/if}

	<div class="verses">
		{#each data.verses as verse, vi (vi)}
			<div class="verse">
				{#if verse.number !== undefined}
					<span class="verse-number">{verse.number}</span>
				{/if}

				<div class="verse-content">
					<div class="verse-lines">
						{#each verse.lines as line, li (li)}
							<p class="line">
								<ClickableText text={line} />
							</p>
						{/each}
					</div>

					{#if verse.translation}
						<p class="translation">{verse.translation}</p>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</section>

<style>
	.poetry-section {
		background: linear-gradient(180deg, var(--gold-faint) 0%, var(--cream) 100%);
		border: 1px solid var(--gold-dim);
		border-radius: 12px;
		padding: 28px 32px;
		box-shadow: 0 2px 12px rgba(197, 148, 26, 0.08);
		position: relative;
	}

	.poetry-section::before {
		content: '';
		position: absolute;
		top: 0;
		left: 24px;
		right: 24px;
		height: 3px;
		background: linear-gradient(90deg, transparent, var(--gold), transparent);
		border-radius: 0 0 2px 2px;
	}

	@media (max-width: 640px) {
		.poetry-section {
			padding: 20px 24px;
			border-radius: 8px;
		}

		.poetry-section::before {
			left: 16px;
			right: 16px;
		}
	}

	.section-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 0.85em;
		font-weight: 700;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0 0 1.5em 0;
		padding-bottom: 0.8em;
		border-bottom: 1px solid var(--cream-mid);
		text-align: center;
	}

	.verses {
		display: flex;
		flex-direction: column;
		gap: 2em;
	}

	.verse {
		display: flex;
		gap: 0.75em;
		align-items: flex-start;
	}

	.verse-number {
		font-family: 'Catamaran', sans-serif;
		font-size: 0.85em;
		font-weight: 700;
		color: var(--gold);
		min-width: 1.5em;
		text-align: right;
		padding-top: 0.1em;
		flex-shrink: 0;
	}

	.verse-content {
		flex: 1;
		min-width: 0;
	}

	.verse-lines {
		padding-left: 0.8em;
		border-left: 2px solid var(--gold-dim);
	}

	.line {
		margin: 0;
		padding: 0.15em 0;
		font-weight: 500;
		color: var(--ink);
		text-align: left;
		overflow-wrap: break-word;
	}

	.translation {
		margin: 0.6em 0 0 0;
		padding-left: 0.8em;
		font-size: 0.88em;
		color: var(--ink-soft);
		text-align: left;
		line-height: 1.5;
	}
</style>
