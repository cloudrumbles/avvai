<script lang="ts">
	import {
		ProseSection,
		PoetrySection,
		VocabularySection,
		MediaSection,
		DialogueSection,
		ExercisesSection
	} from './sections';
	import type { Lesson, ContentSection } from '$lib/types/lesson';

	interface Props {
		lesson: Lesson;
		compact?: boolean;
	}

	let { lesson, compact = false }: Props = $props();

	// Filter out empty sections
	let contentSections = $derived(
		lesson.sections?.filter((s: ContentSection) => {
			if (s.type === 'exercises') return false; // Exercises shown separately
			// Check if section has content
			switch (s.type) {
				case 'prose':
					return s.paragraphs.some(p => p.trim().length > 0);
				case 'poetry':
					return s.verses.some(v => v.lines.some(l => l.trim().length > 0));
				case 'vocabulary':
					return s.entries.some(e => e.word.trim().length > 0);
				case 'media':
					return s.url.trim().length > 0;
				case 'dialogue':
					return s.lines.some(l => l.text.trim().length > 0);
				default:
					return false;
			}
		}) ?? []
	);

	let exercisesSection = $derived(
		lesson.sections?.find((s: ContentSection) => s.type === 'exercises') as { type: 'exercises'; title?: string; exercise_groups: any[] } | undefined
	);
</script>

<div class="lesson-preview" class:compact>
	<div class="preview-header">
		<h2 class="preview-title">{lesson.title}</h2>
		{#if lesson.source}
			<div class="source-info">
				{#if lesson.source.author}
					<span class="source-author">{lesson.source.author}</span>
				{/if}
				{#if lesson.source.title}
					<span class="source-work">{lesson.source.title}</span>
				{/if}
			</div>
		{/if}
		{#if lesson.description}
			<p class="preview-description">{lesson.description}</p>
		{/if}
	</div>

	<div class="preview-content">
		{#if contentSections.length === 0}
			<p class="empty-notice">No content sections yet. Add sections to see the preview.</p>
		{:else}
			{#each contentSections as section, i (i)}
				<div class="section-wrapper">
					{#if section.type === 'prose'}
						<ProseSection data={section} />
					{:else if section.type === 'poetry'}
						<PoetrySection data={section} />
					{:else if section.type === 'vocabulary'}
						<VocabularySection data={section} />
					{:else if section.type === 'media'}
						<MediaSection data={section} />
					{:else if section.type === 'dialogue'}
						<DialogueSection data={section} />
					{/if}
				</div>
			{/each}
		{/if}

		{#if exercisesSection && exercisesSection.exercise_groups.length > 0}
			<div class="exercises-wrapper">
				<h3 class="exercises-heading">{exercisesSection.title || 'பயிற்சிகள்'}</h3>
				<ExercisesSection data={exercisesSection} />
			</div>
		{/if}
	</div>

	<div class="preview-footer">
		<span class="end-marker">❧</span>
	</div>
</div>

<style>
	.lesson-preview {
		background: var(--cream, #faf3e6);
		min-height: 100%;
		padding: 24px;
		font-family: 'Catamaran', sans-serif;
		color: var(--ink, #1a0e06);
	}

	.lesson-preview.compact {
		padding: 16px;
	}

	.lesson-preview.compact .preview-header {
		margin-bottom: 20px;
		padding-bottom: 16px;
	}

	.lesson-preview.compact .preview-title {
		font-size: 1.4rem;
	}

	.preview-header {
		margin-bottom: 32px;
		padding-bottom: 24px;
		border-bottom: 1px solid var(--cream-mid, #f0e4cc);
	}

	.preview-title {
		font-family: 'Cormorant Garamond', Georgia, serif;
		font-size: 1.75rem;
		font-weight: 600;
		color: var(--red, #8b1a1a);
		margin: 0 0 12px;
	}

	.source-info {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
		font-size: 0.9rem;
		color: var(--stone, #6b5a48);
		margin-bottom: 12px;
	}

	.source-author {
		font-weight: 600;
	}

	.source-work {
		font-style: italic;
	}

	.preview-description {
		font-size: 0.95rem;
		color: var(--ink-soft, #3a2a1a);
		line-height: 1.6;
		margin: 0;
	}

	.preview-content {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.section-wrapper {
		margin-bottom: 8px;
	}

	.empty-notice {
		text-align: center;
		padding: 48px 24px;
		color: var(--stone, #6b5a48);
		font-style: italic;
	}

	.exercises-wrapper {
		margin-top: 32px;
		padding-top: 24px;
		border-top: 2px solid var(--cream-mid, #f0e4cc);
	}

	.exercises-heading {
		font-family: 'Catamaran', sans-serif;
		font-size: 1.1rem;
		font-weight: 700;
		color: var(--red, #8b1a1a);
		text-align: center;
		margin: 0 0 24px;
	}

	.preview-footer {
		display: flex;
		justify-content: center;
		padding: 32px 0 16px;
	}

	.end-marker {
		font-size: 24px;
		color: var(--gold, #c5941a);
	}
</style>
