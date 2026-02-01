<script lang="ts">
	import ClickableText from 'avvai-frontend/components/ClickableText';

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
		background: linear-gradient(180deg, var(--color-bg-soft) 0%, var(--color-bg) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-7) var(--space-7);
		box-shadow: var(--shadow-warm);
		position: relative;
	}

	.poetry-section::before {
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
		.poetry-section {
			padding: var(--space-5) var(--space-6);
			border-radius: var(--radius-2);
		}

		.poetry-section::before {
			left: var(--space-4);
			right: var(--space-4);
		}
	}

	.section-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 700;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-wide);
		margin: 0 0 var(--space-6) 0;
		padding-bottom: var(--space-3);
		border-bottom: 1px solid var(--color-bg-soft);
		text-align: center;
	}

	.verses {
		display: flex;
		flex-direction: column;
		gap: var(--space-7);
	}

	.verse {
		display: flex;
		gap: var(--space-3);
		align-items: flex-start;
	}

	.verse-number {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 700;
		color: var(--color-highlight);
		min-width: var(--space-6);
		text-align: right;
		padding-top: var(--space-0);
		flex-shrink: 0;
	}

	.verse-content {
		flex: 1;
		min-width: 0;
	}

	.verse-lines {
		padding-left: var(--space-3);
		border-left: 2px solid var(--color-bg-soft);
	}

	.line {
		margin: 0;
		padding: var(--space-0) 0;
		font-weight: 500;
		color: var(--color-text);
		text-align: left;
		overflow-wrap: break-word;
	}

	.translation {
		margin: var(--space-2) 0 0 0;
		padding-left: var(--space-3);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		text-align: left;
		line-height: var(--line-height-3);
	}
</style>