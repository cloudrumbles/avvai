<script lang="ts">
	import {
		DragHandleIcon,
		ArrowUpIcon,
		ArrowDownIcon,
		TrashIcon
	} from '$lib/components/icons';
	import type { ContentSection } from '$lib/types/lesson';

	interface Props {
		sections: ContentSection[];
		activeIndex: number;
		onSectionClick: (index: number) => void;
		onMoveSection: (index: number, direction: -1 | 1) => void;
		onDeleteSection: (index: number) => void;
	}

	let {
		sections,
		activeIndex,
		onSectionClick,
		onMoveSection,
		onDeleteSection
	}: Props = $props();

	// Section type configuration
	const sectionTypes: Record<string, { label: string; color: string }> = {
		prose: { label: 'Prose', color: '#2B6CB0' },
		poetry: { label: 'Poetry', color: '#805AD5' },
		vocabulary: { label: 'Vocabulary', color: '#2F855A' },
		exercises: { label: 'Exercises', color: '#B8533D' },
		media: { label: 'Media', color: '#B7791F' },
		dialogue: { label: 'Dialogue', color: '#D53F8C' }
	};

	function getTypeConfig(type: string) {
		return sectionTypes[type] ?? { label: type, color: '#5C4A3D' };
	}

	function getSectionTitle(section: ContentSection): string {
		if (section.title) return section.title;
		// Generate a preview from content
		switch (section.type) {
			case 'prose':
				return section.paragraphs[0]?.slice(0, 30) || 'Empty prose';
			case 'poetry':
				return section.verses[0]?.lines[0]?.slice(0, 30) || 'Empty poetry';
			case 'vocabulary':
				return section.entries[0]?.word || 'Empty vocabulary';
			case 'dialogue':
				return section.lines[0]?.text?.slice(0, 30) || 'Empty dialogue';
			case 'media':
				return section.url ? 'Media' : 'Empty media';
			case 'exercises':
				return `${section.exercise_groups?.length || 0} groups`;
			default:
				return 'Untitled';
		}
	}
</script>

<div class="structure-list">
	{#if sections.length === 0}
		<p class="empty-state">No sections yet</p>
	{:else}
		<ul class="section-items">
			{#each sections as section, index (index)}
				{@const config = getTypeConfig(section.type)}
				<li class="section-item" class:active={activeIndex === index}>
					<button
						type="button"
						class="section-btn"
						onclick={() => onSectionClick(index)}
					>
						<span class="drag-handle">
							<DragHandleIcon size={12} color="var(--c-brown-muted)" />
						</span>

						<span class="section-info">
							<span
								class="section-type"
								style="color: {config.color}"
							>
								{config.label}
							</span>
							<span class="section-title">{getSectionTitle(section)}</span>
						</span>
					</button>

					<div class="section-actions">
						<button
							type="button"
							class="action-btn"
							disabled={index === 0}
							onclick={() => onMoveSection(index, -1)}
							title="Move up"
						>
							<ArrowUpIcon
								size={12}
								color={index === 0 ? 'var(--c-brown-subtle)' : 'var(--c-brown)'}
							/>
						</button>
						<button
							type="button"
							class="action-btn"
							disabled={index === sections.length - 1}
							onclick={() => onMoveSection(index, 1)}
							title="Move down"
						>
							<ArrowDownIcon
								size={12}
								color={index === sections.length - 1 ? 'var(--c-brown-subtle)' : 'var(--c-brown)'}
							/>
						</button>
						<button
							type="button"
							class="action-btn delete"
							onclick={() => onDeleteSection(index)}
							title="Delete section"
						>
							<TrashIcon size={12} color="var(--c-terracotta)" />
						</button>
					</div>
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.structure-list {
		display: flex;
		flex-direction: column;
	}

	.empty-state {
		font-size: 0.8rem;
		color: var(--c-brown-muted);
		text-align: center;
		padding: var(--space-4);
		font-style: italic;
	}

	.section-items {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.section-item {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		border-radius: var(--radius-sm);
		transition: background var(--transition-fast);
	}

	.section-item:hover {
		background: rgba(92, 74, 61, 0.04);
	}

	.section-item.active {
		background: rgba(184, 83, 61, 0.08);
	}

	.section-btn {
		flex: 1;
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2);
		background: none;
		border: none;
		cursor: pointer;
		text-align: left;
		min-width: 0;
	}

	.drag-handle {
		opacity: 0.4;
		cursor: grab;
		display: flex;
		align-items: center;
		padding: 2px;
	}

	.section-item:hover .drag-handle {
		opacity: 0.8;
	}

	.section-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.section-type {
		font-size: 0.65rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}

	.section-title {
		font-size: 0.75rem;
		color: var(--c-brown);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.section-actions {
		display: flex;
		gap: 2px;
		opacity: 0;
		transition: opacity var(--transition-fast);
		padding-right: var(--space-2);
	}

	.section-item:hover .section-actions {
		opacity: 1;
	}

	.action-btn {
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: background var(--transition-fast);
	}

	.action-btn:hover:not(:disabled) {
		background: rgba(92, 74, 61, 0.08);
	}

	.action-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.action-btn.delete:hover {
		background: rgba(184, 83, 61, 0.1);
	}
</style>
