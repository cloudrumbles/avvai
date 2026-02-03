<script lang="ts">
	import type { ExercisesSection as ExercisesSectionType } from '$lib/types/lesson';
	import { getDictionaryPanelState, hideDictionaryPanel, setPanelEntry, setPanelLoading } from '$lib/stores/dictionary.svelte';
	import { lookup } from '$lib/services/dictionary';
	import ExercisesSection from './sections/ExercisesSection.svelte';

	interface Props {
		exercisesSection?: ExercisesSectionType;
	}

	let { exercisesSection }: Props = $props();

	const panel = getDictionaryPanelState();

	$effect(() => {
		if (!panel.visible || !panel.word) return;

		setPanelLoading(true);
		setPanelEntry(null);

		lookup(panel.word).then((result) => {
			setPanelEntry(result);
			setPanelLoading(false);
		});
	});
</script>

<div class="right-panel">
	<div class="panel-section dictionary-section">
		<div class="panel-header">
			<span class="panel-title">Dictionary</span>
			{#if panel.visible}
				<button class="close-btn" onclick={hideDictionaryPanel} aria-label="Clear dictionary">
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
						<path d="M4 4L12 12M4 12L12 4" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
					</svg>
				</button>
			{/if}
		</div>

		<div class="dictionary-content">
			{#if !panel.visible}
				<p class="placeholder-text">Click any word in the lesson to look it up</p>
			{:else if panel.loading}
				<p class="loading-text">Looking up…</p>
			{:else if panel.entry}
				<div class="entry">
					<p class="entry-word">{panel.word}</p>
					<p class="entry-definition">{panel.entry.definition}</p>
					{#if panel.entry.examples?.length}
						<div class="entry-examples">
							{#each panel.entry.examples as ex, ei (ei)}
								<p class="entry-example">{ex}</p>
							{/each}
						</div>
					{/if}
				</div>
			{:else}
				<p class="no-result">No definition found for “{panel.word}”</p>
			{/if}
		</div>
	</div>

	{#if exercisesSection}
		<div class="panel-section exercises-section">
			<div class="panel-header">
				<span class="panel-title">{exercisesSection.title || 'பயிற்சிகள்'}</span>
			</div>
			<div class="exercises-content">
				<ExercisesSection data={exercisesSection} />
			</div>
		</div>
	{/if}
</div>

<style>
	.right-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--color-bg);
		border-left: 1px solid var(--color-bg-soft);
	}

	.panel-section {
		display: flex;
		flex-direction: column;
	}

	.panel-section:first-child {
		flex: 0 0 auto;
		max-height: 50%;
	}

	.panel-section:last-child {
		flex: 1;
		min-height: 0;
	}

	.panel-section + .panel-section {
		border-top: 1px solid var(--color-bg-soft);
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-5);
		border-bottom: 1px solid var(--color-bg-soft);
		background: var(--color-bg-soft);
	}

	.panel-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		font-weight: 700;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-wide);
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		border: none;
		border-radius: var(--radius-2);
		background: transparent;
		color: var(--color-text-subtle);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.close-btn:hover {
		background: var(--color-bg-pressed);
		color: var(--color-text);
	}

	.dictionary-content {
		padding: var(--space-4) var(--space-5);
		overflow-y: auto;
	}

	.placeholder-text,
	.loading-text,
	.no-result {
		margin: 0;
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		font-style: italic;
		line-height: var(--line-height-3);
	}

	.entry-word {
		font-family: var(--font-sans);
		font-size: var(--font-size-3);
		font-weight: 700;
		color: var(--color-accent-strong);
		margin: 0 0 var(--space-2);
	}

	.entry-definition {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text);
		margin: 0;
		line-height: var(--line-height-3);
	}

	.entry-examples {
		margin-top: var(--space-3);
		padding-top: var(--space-3);
		border-top: 1px solid var(--color-bg-soft);
	}

	.entry-example {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		margin: var(--space-2) 0 0;
		line-height: var(--line-height-2);
		font-style: italic;
	}

	.entry-example:first-child {
		margin-top: 0;
	}

	.exercises-content {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-4) var(--space-5);
	}
</style>
