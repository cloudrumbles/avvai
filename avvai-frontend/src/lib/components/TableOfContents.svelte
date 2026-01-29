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
		background: var(--cream);
		border-right: 1px solid var(--cream-mid);
	}

	.toc-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-bottom: 1px solid var(--cream-mid);
	}

	.toc-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 700;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.06em;
	}

	.toc-progress {
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 600;
		color: var(--gold);
	}

	.progress-bar {
		height: 2px;
		background: var(--cream-mid);
		margin: 0 20px;
		border-radius: 1px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--gold);
		transition: width 0.3s ease;
	}

	.toc-list {
		list-style: none;
		margin: 0;
		padding: 12px 16px;
		overflow-y: auto;
		flex: 1;
	}

	.toc-item {
		margin: 0;
	}

	.toc-item:not(:last-child) {
		border-bottom: 1px solid var(--cream-mid);
	}

	.toc-link {
		display: flex;
		align-items: flex-start;
		gap: 10px;
		width: 100%;
		padding: 12px 8px;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		text-align: left;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.toc-link:hover {
		background: var(--red-faint);
	}

	.toc-item.active .toc-link {
		background: var(--red-faint);
	}

	.toc-item.active .toc-label {
		color: var(--red);
	}

	.toc-item.active .toc-section-title {
		color: var(--red-deep);
	}

	.toc-icon {
		font-size: 14px;
		line-height: 1.4;
		flex-shrink: 0;
		width: 20px;
		text-align: center;
	}

	.toc-text {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
		flex: 1;
	}

	.toc-label {
		font-family: 'Catamaran', sans-serif;
		font-size: 10px;
		font-weight: 600;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.04em;
		transition: color 0.15s ease;
	}

	.toc-section-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 500;
		color: var(--ink);
		line-height: 1.4;
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
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
		background: var(--cream-mid);
		border-radius: 3px;
	}

	.toc-list::-webkit-scrollbar-thumb:hover {
		background: var(--stone);
	}
</style>
