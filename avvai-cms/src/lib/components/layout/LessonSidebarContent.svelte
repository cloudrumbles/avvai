<script lang="ts">
	import StructureList from '$lib/components/editor/StructureList.svelte';
	import { ChevronIcon, PlusIcon } from '$lib/components/icons';
	import type { Lesson } from '$lib/types/lesson';

	interface Props {
		lesson: Lesson;
		activeSectionIndex: number;
		onSectionClick: (index: number) => void;
		onAddSection: (type: string) => void;
		onMoveSection: (index: number, direction: -1 | 1) => void;
		onDeleteSection: (index: number) => void;
	}

	let {
		lesson,
		activeSectionIndex,
		onSectionClick,
		onAddSection,
		onMoveSection,
		onDeleteSection
	}: Props = $props();

	let showAddMenu = $state(false);

	const sectionTypes = [
		{ type: 'prose', label: 'Prose', color: '#2B6CB0' },
		{ type: 'poetry', label: 'Poetry', color: '#805AD5' },
		{ type: 'vocabulary', label: 'Vocabulary', color: '#2F855A' },
		{ type: 'exercises', label: 'Exercises', color: '#B8533D' },
		{ type: 'media', label: 'Media', color: '#B7791F' },
		{ type: 'dialogue', label: 'Dialogue', color: '#D53F8C' }
	] as const;

	function handleAddSection(type: string) {
		onAddSection(type);
		showAddMenu = false;
	}

</script>

<div class="sidebar-content">
	<div class="structure-block">
		<StructureList
			sections={lesson.sections}
			activeIndex={activeSectionIndex}
			{onSectionClick}
			{onMoveSection}
			{onDeleteSection}
		/>

		<div class="add-section-wrapper">
			<button
				type="button"
				class="add-section-btn"
				onclick={() => (showAddMenu = !showAddMenu)}
			>
				<PlusIcon size={14} color="currentColor" />
				<span>Add Section</span>
				<ChevronIcon expanded={showAddMenu} size={12} color="currentColor" />
			</button>

			{#if showAddMenu}
				<div class="add-menu">
				{#each sectionTypes as sectionType (sectionType.type)}
						<button
							type="button"
							class="add-menu-item"
							onclick={() => handleAddSection(sectionType.type)}
						>
							<span
								class="type-dot"
								style="background-color: {sectionType.color}"
							></span>
							{sectionType.label}
						</button>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.sidebar-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.structure-block {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.add-section-wrapper {
		position: relative;
		margin-top: var(--space-3);
		padding-top: var(--space-3);
		border-top: 1px solid rgba(92, 74, 61, 0.1);
	}

	.add-section-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: white;
		border: 1px dashed rgba(92, 74, 61, 0.2);
		border-radius: var(--radius-sm);
		color: var(--c-brown);
		font-size: 0.8rem;
		font-weight: 500;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.add-section-btn:hover {
		border-color: var(--c-terracotta);
		color: var(--c-terracotta);
		background: rgba(184, 83, 61, 0.04);
	}

	.add-menu {
		position: absolute;
		bottom: 100%;
		left: 0;
		right: 0;
		margin-bottom: var(--space-2);
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.1);
		border-radius: var(--radius-sm);
		box-shadow: var(--shadow-md);
		overflow: hidden;
		z-index: 10;
	}

	.add-menu-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: none;
		border: none;
		font-size: 0.8rem;
		color: var(--c-brown);
		cursor: pointer;
		text-align: left;
		transition: background var(--transition-fast);
	}

	.add-menu-item:hover {
		background: var(--c-cream);
	}

	.type-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}
</style>
