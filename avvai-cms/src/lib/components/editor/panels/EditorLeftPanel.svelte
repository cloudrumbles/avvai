<script lang="ts">
	import Nav from '$lib/components/layout/Nav.svelte';
	import StructureList from '$lib/components/editor/StructureList.svelte';
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { ChevronIcon, PlusIcon } from '$lib/components/icons';
	import type { Lesson, ContentSection } from '$lib/types/lesson';

	interface Props {
		lesson: Lesson;
		activeSectionIndex: number;
		onSectionClick: (index: number) => void;
		onAddSection: (type: string) => void;
		onMoveSection: (index: number, direction: -1 | 1) => void;
		onDeleteSection: (index: number) => void;
		onLessonChange: () => void;
	}

	let {
		lesson = $bindable(),
		activeSectionIndex,
		onSectionClick,
		onAddSection,
		onMoveSection,
		onDeleteSection,
		onLessonChange
	}: Props = $props();

	let showAddMenu = $state(false);
	let metadataExpanded = $state(false);

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

	function handleSourceChange() {
		onLessonChange();
	}
</script>

<aside class="left-panel">
	<div class="panel-section nav-section">
		<Nav />
	</div>

	<div class="panel-section structure-section">
		<div class="section-header">
			<span class="section-title">Structure</span>
			<span class="section-count">{lesson.sections.length}</span>
		</div>

		<div class="structure-content">
			<StructureList
				sections={lesson.sections}
				activeIndex={activeSectionIndex}
				{onSectionClick}
				{onMoveSection}
				{onDeleteSection}
			/>
		</div>

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
					{#each sectionTypes as sectionType}
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

	<div class="panel-section metadata-section">
		<button
			type="button"
			class="section-header clickable"
			onclick={() => (metadataExpanded = !metadataExpanded)}
		>
			<span class="section-title">Lesson</span>
			<ChevronIcon expanded={metadataExpanded} size={12} color="var(--c-brown-muted)" />
		</button>

		{#if metadataExpanded}
			<div class="metadata-content">
				<div class="form-group">
					<TextInput
						label="Title"
						bind:value={lesson.title}
						oninput={onLessonChange}
					/>
				</div>

				<div class="form-group">
					<TextInput
						label="Source Title"
						value={lesson.source?.title ?? ''}
						oninput={(e) => {
							if (!lesson.source) lesson.source = { title: '' };
							lesson.source.title = e.currentTarget.value;
							handleSourceChange();
						}}
					/>
				</div>

				<div class="form-group">
					<TextInput
						label="Author"
						value={lesson.source?.author ?? ''}
						placeholder="Optional"
						oninput={(e) => {
							if (!lesson.source) lesson.source = { title: '' };
							lesson.source.author = e.currentTarget.value || undefined;
							handleSourceChange();
						}}
					/>
				</div>

				<div class="form-group">
					<Textarea
						label="Description"
						bind:value={lesson.description}
						rows={2}
						oninput={onLessonChange}
					/>
				</div>
			</div>
		{/if}
	</div>
</aside>

<style>
	.left-panel {
		height: 100vh;
		display: flex;
		flex-direction: column;
		background: var(--c-cream-dark);
		border-right: 1px solid rgba(92, 74, 61, 0.1);
		overflow: hidden;
	}

	.panel-section {
		padding: var(--space-4);
	}

	.nav-section {
		border-bottom: 1px solid rgba(92, 74, 61, 0.1);
	}

	.structure-section {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-height: 0;
		overflow: hidden;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-3);
	}

	.section-header.clickable {
		cursor: pointer;
		padding: var(--space-2) 0;
		margin-bottom: var(--space-2);
		border: none;
		background: none;
		width: 100%;
		text-align: left;
	}

	.section-header.clickable:hover {
		color: var(--c-terracotta);
	}

	.section-title {
		font-family: var(--font-heading);
		font-size: 0.7rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--c-brown-muted);
	}

	.section-count {
		font-size: 0.7rem;
		font-weight: 600;
		color: var(--c-brown-muted);
		background: rgba(92, 74, 61, 0.08);
		padding: 0.1rem 0.4rem;
		border-radius: 8px;
	}

	.structure-content {
		flex: 1;
		overflow-y: auto;
		margin: 0 calc(-1 * var(--space-2));
		padding: 0 var(--space-2);
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

	.metadata-section {
		border-top: 1px solid rgba(92, 74, 61, 0.1);
		background: rgba(255, 255, 255, 0.5);
	}

	.metadata-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.form-group {
		display: flex;
		flex-direction: column;
	}
</style>
