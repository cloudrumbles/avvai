<script lang="ts">
	import type { ContentSection } from '$lib/types/lesson';

	interface Props {
		sections: ContentSection[];
		activeSectionIndex: number;
		onSectionClick: (index: number) => void;
		progressPercent: number;
	}

	let { sections, activeSectionIndex, onSectionClick, progressPercent }: Props = $props();

	function getSectionIcon(type: string): string {
		switch (type) {
			case 'prose':
				return '📖';
			case 'poetry':
				return '✦';
			case 'vocabulary':
				return '📚';
			case 'exercises':
				return '✏️';
			case 'media':
				return '🎬';
			default:
				return '•';
		}
	}

	function getSectionLabel(type: string): string {
		switch (type) {
			case 'prose':
				return 'Reading';
			case 'poetry':
				return 'Poetry';
			case 'vocabulary':
				return 'Vocabulary';
			case 'exercises':
				return 'Exercises';
			case 'media':
				return 'Media';
			default:
			return 'Section';
		}
	}

	function getSectionTitle(section: ContentSection): string {
		if ('title' in section && section.title) {
			return section.title;
		}
		return getSectionLabel(section.type);
	}
</script>

<nav class="toc" aria-label="Lesson sections">
	<div class="toc-header">
		<span class="toc-title">Contents</span>
		<span class="toc-progress">{Math.round(progressPercent)}%</span>
	</div>

	<div class="progress-bar">
		<div class="progress-fill" style="width: {progressPercent}%"></div>
	</div>

	<ol class="toc-list">
		{#each sections as section, index (index)}
			<li class="toc-item" class:active={index === activeSectionIndex}>
				<button
					class="toc-link"
					onclick={() => onSectionClick(index)}
					aria-current={index === activeSectionIndex ? 'true' : undefined}
				>
					<span class="toc-icon">{getSectionIcon(section.type)}</span>
					<span class="toc-text">
						<span class="toc-label">{getSectionLabel(section.type)}</span>
						<span class="toc-section-title">{getSectionTitle(section)}</span>
					</span>
				</button>
			</li>
		{/each}
	</ol>
</nav>

<style>
	.toc {
		height: 100%;
		display: flex;
		flex-direction: column;
		background: var(--color-bg);
		border-right: 1px solid var(--color-bg-soft);
	}

	.toc-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--space-4) var(--space-5);
		border-bottom: 1px solid var(--color-bg-soft);
	}

	.toc-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		font-weight: 700;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-wide);
	}

	.toc-progress {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		font-weight: 600;
		color: var(--color-highlight);
	}

	.progress-bar {
		height: 2px;
		background: var(--color-bg-soft);
		margin: 0 var(--space-5);
		border-radius: var(--radius-0);
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--color-highlight);
		transition: width 0.3s ease;
	}

	.toc-list {
		list-style: none;
		margin: 0;
		padding: var(--space-3) var(--space-4);
		overflow-y: auto;
		flex: 1;
	}

	.toc-item {
		margin: 0;
	}

	.toc-item:not(:last-child) {
		border-bottom: 1px solid var(--color-bg-soft);
	}

	.toc-link {
		display: flex;
		align-items: flex-start;
		gap: var(--space-2-5);
		width: 100%;
		padding: var(--space-3) var(--space-2);
		background: transparent;
		border: none;
		border-radius: var(--radius-2);
		cursor: pointer;
		text-align: left;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.toc-link:hover {
		background: var(--color-accent-tint);
	}

	.toc-item.active .toc-link {
		background: var(--color-accent-tint);
	}

	.toc-item.active .toc-label {
		color: var(--color-accent);
	}

	.toc-item.active .toc-section-title {
		color: var(--color-accent-strong);
	}

	.toc-icon {
		font-size: var(--font-size-2);
		line-height: var(--line-height-1-4);
		flex-shrink: 0;
		width: 20px;
		text-align: center;
	}

	.toc-text {
		display: flex;
		flex-direction: column;
		gap: var(--space-0-5);
		min-width: 0;
		flex: 1;
	}

	.toc-label {
		font-family: var(--font-sans);
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-4);
		transition: color 0.15s ease;
	}

	.toc-section-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		font-weight: 500;
		color: var(--color-text);
		line-height: var(--line-height-1-4);
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		line-clamp: 2;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		transition: color 0.15s ease;
	}

	/* Scrollbar styling */
	.toc-list::-webkit-scrollbar {
		width: 6px;
	}

	.toc-list::-webkit-scrollbar-track {
		background: transparent;
	}

	.toc-list::-webkit-scrollbar-thumb {
		background: var(--color-bg-soft);
		border-radius: var(--radius-0-75);
	}

	.toc-list::-webkit-scrollbar-thumb:hover {
		background: var(--color-text-subtle);
	}
</style>
