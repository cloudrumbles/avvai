<script lang="ts">
	import { fly, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import EditorSection from '$lib/components/editor/EditorSection.svelte';
	import ProseEditor from '$lib/components/editor/ProseEditor.svelte';
	import PoetryEditor from '$lib/components/editor/PoetryEditor.svelte';
	import VocabularyEditor from '$lib/components/editor/VocabularyEditor.svelte';
	import MediaEditor from '$lib/components/editor/MediaEditor.svelte';
	import DialogueEditor from '$lib/components/editor/DialogueEditor.svelte';
	import ExercisesEditor from '$lib/components/editor/ExercisesEditor.svelte';
	import { PlusIcon } from '$lib/components/icons';
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import type { Lesson } from '$lib/types/lesson';

	interface Props {
		lesson: Lesson;
		isDirty: boolean;
		onLessonChange: () => void;
		onMoveSection: (index: number, direction: -1 | 1) => void;
		onDeleteSection: (index: number) => void;
		onAddSection: (type: string) => void;
		sectionRefs: HTMLElement[];
	}

	let {
		lesson = $bindable(),
		isDirty,
		onLessonChange,
		onMoveSection,
		onDeleteSection,
		onAddSection,
		sectionRefs = $bindable([])
	}: Props = $props();

	function handleSourceChange() {
		onLessonChange();
	}

	// Section type configuration
	const sectionTypes = [
		{ type: 'prose', label: 'Prose', color: '#2B6CB0', bgColor: '#EBF8FF' },
		{ type: 'poetry', label: 'Poetry', color: '#805AD5', bgColor: '#FAF5FF' },
		{ type: 'vocabulary', label: 'Vocabulary', color: '#2F855A', bgColor: '#F0FFF4' },
		{ type: 'exercises', label: 'Exercises', color: '#B8533D', bgColor: '#FEF3F0' },
		{ type: 'media', label: 'Media', color: '#B7791F', bgColor: '#FFFFF0' },
		{ type: 'dialogue', label: 'Dialogue', color: '#D53F8C', bgColor: '#FFF5F7' }
	] as const;

	function getSectionTypeConfig(type: string) {
		return sectionTypes.find((t) => t.type === type) ?? sectionTypes[0];
	}
</script>

<div class="middle-panel">
	<header class="panel-header">
		<div class="header-content">
			<h1 class="lesson-title">{lesson.title}</h1>
			{#if isDirty}
				<span class="unsaved-indicator" in:scale={{ duration: 200 }}>
					<span class="dot"></span>
					Unsaved
				</span>
			{/if}
		</div>
	</header>

	<div class="sections-container">
		<div class="lesson-meta-card">
			<div class="meta-header">
				<h2 class="meta-title">Lesson</h2>
			</div>
			<div class="meta-grid">
				<TextInput label="Title" bind:value={lesson.title} oninput={onLessonChange} />
				<TextInput
					label="Source Title"
					value={lesson.source?.title ?? ''}
					oninput={(e) => {
						if (!lesson.source) lesson.source = { title: '' };
						lesson.source.title = e.currentTarget.value;
						handleSourceChange();
					}}
				/>
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
				<Textarea
					label="Description"
					bind:value={lesson.description}
					rows={2}
					oninput={onLessonChange}
				/>
			</div>
		</div>
		{#if lesson.sections.length === 0}
			<div class="empty-state" in:scale={{ duration: 400, easing: cubicOut }}>
				<div class="empty-illustration">
					<svg width="80" height="80" viewBox="0 0 80 80" fill="none">
						<circle
							cx="40"
							cy="40"
							r="35"
							stroke="#E8DFD4"
							stroke-width="2"
							stroke-dasharray="6 4"
						/>
						<path
							d="M30 33h20M30 40h15M30 47h18"
							stroke="#C4B5A4"
							stroke-width="2"
							stroke-linecap="round"
						/>
						<circle cx="57" cy="53" r="10" fill="#FAF6F0" stroke="#B8533D" stroke-width="2" />
						<path d="M57 48v10M52 53h10" stroke="#B8533D" stroke-width="2" stroke-linecap="round" />
					</svg>
				</div>
				<h3 class="empty-title">Begin Your Lesson</h3>
				<p class="empty-text">Add your first section to start crafting this lesson.</p>

				<div class="quick-add-grid">
					{#each sectionTypes as sectionType}
						<button
							type="button"
							class="quick-add-btn"
							style="--btn-color: {sectionType.color}; --btn-bg: {sectionType.bgColor}"
							onclick={() => onAddSection(sectionType.type)}
						>
							{sectionType.label}
						</button>
					{/each}
				</div>
			</div>
		{:else}
			{#each lesson.sections as section, index (index)}
				{@const config = getSectionTypeConfig(section.type)}
				<div
					class="section-wrapper"
					style="--section-color: {config.color}; --section-bg: {config.bgColor}"
					in:fly={{ y: 20, duration: 300, delay: index * 30 }}
					bind:this={sectionRefs[index]}
					data-index={index}
				>
					<EditorSection
						id="section-{index}"
						title={section.title || `Untitled ${section.type}`}
						type={section.type}
						isFirst={index === 0}
						isLast={index === lesson.sections.length - 1}
						onMoveUp={() => onMoveSection(index, -1)}
						onMoveDown={() => onMoveSection(index, 1)}
						onDelete={() => onDeleteSection(index)}
					>
						{#if section.type === 'prose'}
							<ProseEditor bind:section={lesson.sections[index] as any} />
						{:else if section.type === 'poetry'}
							<PoetryEditor bind:section={lesson.sections[index] as any} />
						{:else if section.type === 'vocabulary'}
							<VocabularyEditor bind:section={lesson.sections[index] as any} />
						{:else if section.type === 'media'}
							<MediaEditor bind:section={lesson.sections[index] as any} />
						{:else if section.type === 'dialogue'}
							<DialogueEditor bind:section={lesson.sections[index] as any} />
						{:else if section.type === 'exercises'}
							<ExercisesEditor bind:section={lesson.sections[index] as any} />
						{/if}
					</EditorSection>
				</div>
			{/each}

			<!-- Inline add section buttons -->
			<div class="inline-add-bar">
				<span class="add-label">Add</span>
				<div class="add-buttons">
					{#each sectionTypes as sectionType}
						<button
							type="button"
							class="add-btn"
							style="--btn-color: {sectionType.color}"
							onclick={() => onAddSection(sectionType.type)}
							title="Add {sectionType.label}"
						>
							<PlusIcon size={12} color={sectionType.color} />
							{sectionType.label}
						</button>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.middle-panel {
		height: 100vh;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		background: var(--c-cream);
	}

	.panel-header {
		position: sticky;
		top: 0;
		z-index: 10;
		background: rgba(250, 246, 240, 0.95);
		backdrop-filter: blur(8px);
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
		padding: var(--space-4) var(--space-6);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.lesson-title {
		font-family: var(--font-display);
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.unsaved-indicator {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--c-gold-dark, #b38600);
		background: rgba(201, 154, 46, 0.1);
		padding: 0.2rem 0.5rem;
		border-radius: 12px;
	}

	.dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: currentColor;
		animation: pulse 1.5s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}

	.sections-container {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
		scroll-behavior: smooth;
	}

	.lesson-meta-card {
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.08);
		border-radius: var(--radius-md);
		padding: var(--space-4);
		margin-bottom: var(--space-5);
		box-shadow: var(--shadow-sm);
	}

	.meta-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-3);
	}

	.meta-title {
		font-family: var(--font-display);
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.meta-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: var(--space-3);
	}

	.meta-grid :global(textarea) {
		grid-column: 1 / -1;
	}

	.section-wrapper {
		margin-bottom: var(--space-4);
		scroll-margin-top: calc(60px + var(--space-4));
	}

	/* Empty state */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-10) var(--space-6);
		background: white;
		border: 2px dashed rgba(92, 74, 61, 0.12);
		border-radius: var(--radius-lg);
		text-align: center;
	}

	.empty-illustration {
		margin-bottom: var(--space-4);
		opacity: 0.8;
	}

	.empty-title {
		font-family: var(--font-display);
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0 0 var(--space-2);
	}

	.empty-text {
		font-size: 0.9rem;
		color: var(--c-brown-muted);
		margin: 0 0 var(--space-6);
	}

	.quick-add-grid {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: var(--space-2);
	}

	.quick-add-btn {
		padding: var(--space-2) var(--space-4);
		background: var(--btn-bg);
		color: var(--btn-color);
		border: 1px solid color-mix(in srgb, var(--btn-color) 20%, transparent);
		border-radius: var(--radius-sm);
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.quick-add-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px color-mix(in srgb, var(--btn-color) 15%, transparent);
	}

	/* Inline add bar */
	.inline-add-bar {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-4);
		background: white;
		border-radius: var(--radius-md);
		border: 1px dashed rgba(92, 74, 61, 0.15);
	}

	.add-label {
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: var(--c-brown-muted);
	}

	.add-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}

	.add-btn {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-1) var(--space-2);
		background: none;
		border: 1px solid rgba(92, 74, 61, 0.1);
		border-radius: var(--radius-sm);
		color: var(--btn-color);
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.add-btn:hover {
		background: color-mix(in srgb, var(--btn-color) 8%, transparent);
		border-color: var(--btn-color);
	}
</style>
