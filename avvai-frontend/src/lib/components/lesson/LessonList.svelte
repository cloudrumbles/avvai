<script lang="ts">
	import type { LessonSummary } from '$lib/types/lesson';

	interface Props {
		lessons: LessonSummary[];
		progress: Record<string, boolean>;
		onselect: (id: string) => void;
	}

	let { lessons, progress, onselect }: Props = $props();

	const totalLessons = $derived(lessons.length);
	const completedLessons = $derived(lessons.filter((lesson) => progress?.[lesson.id]).length);
	const currentLesson = $derived(lessons.find((lesson) => !progress?.[lesson.id]));
	const progressPct = $derived(totalLessons > 0 ? (completedLessons / totalLessons) * 100 : 0);
	const currentLessonIndex = $derived(
		currentLesson ? lessons.findIndex((lesson) => lesson.id === currentLesson.id) + 1 : 0
	);

	function handleLessonClick(id: string) {
		onselect(id);
	}

	type LessonStatus = 'completed' | 'current' | 'not_started';

	function getStatus(lesson: LessonSummary): LessonStatus {
		if (progress?.[lesson.id]) return 'completed';
		if (currentLesson?.id === lesson.id) return 'current';
		return 'not_started';
	}
</script>

<div class="lesson-list">
	<!-- Colophon-style Progress Header -->
	<header class="progress-header">
		<span class="progress-label">Lesson {currentLessonIndex} of {totalLessons}</span>
		<div class="progress-track">
			<div class="progress-fill" style="width: {progressPct}%"></div>
		</div>
	</header>

	<!-- Now Reading Section -->
	{#if currentLesson}
		<section class="now-reading">
			<button class="now-reading-button" onclick={() => handleLessonClick(currentLesson.id)}>
				<span class="now-reading-label">Now Reading</span>
				<h2 class="now-reading-title">{currentLesson.title}</h2>
				{#if currentLesson.description}
					<p class="now-reading-description">{currentLesson.description}</p>
				{/if}
				<span class="continue-reading">Continue reading →</span>
			</button>
		</section>
	{/if}

	<!-- Fleuron Divider -->
	<div class="fleuron">❧</div>

	<!-- All Lessons Section -->
	<section class="lessons-section">
		<h2 class="section-heading">All Lessons</h2>
		<div class="section-divider"></div>

		{#if lessons.length === 0}
			<div class="empty-state">
				<p>No lessons are available yet. Check back soon.</p>
			</div>
		{:else}
			<div class="lesson-entries">
				{#each lessons as lesson, index (lesson.id)}
					{@const status = getStatus(lesson)}
					<button
						class="lesson-entry"
						class:completed={status === 'completed'}
						class:current={status === 'current'}
						onclick={() => handleLessonClick(lesson.id)}
					>
						<span class="lesson-title">{index + 1}. {lesson.title}</span>
						{#if lesson.description}
							<p class="lesson-description">{lesson.description}</p>
						{/if}
					</button>
				{/each}
			</div>
		{/if}
	</section>
</div>

<style>
	.lesson-list {
		min-height: 100dvh;
		background: var(--color-bg);
		max-width: 600px;
		margin: 0 auto;
		padding: var(--space-5) var(--space-4) var(--space-10);
	}

	/* Colophon-style Progress Header */
	.progress-header {
		text-align: center;
		margin-bottom: var(--space-7);
	}

	.progress-label {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		display: block;
		margin-bottom: var(--space-2);
	}

	.progress-track {
		height: var(--space-0-75);
		background: var(--color-bg-pressed);
		border-radius: var(--radius-0-5);
		overflow: hidden;
		max-width: 200px;
		margin: 0 auto;
	}

	.progress-fill {
		height: 100%;
		background: var(--color-highlight);
		border-radius: var(--radius-0-5);
		transition: width 0.3s ease;
	}

	/* Now Reading Section */
	.now-reading {
		background: linear-gradient(180deg, var(--color-bg-soft) 0%, var(--color-bg) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		box-shadow: var(--shadow-warm);
		position: relative;
		overflow: hidden;
		margin-bottom: var(--space-7);
	}

	.now-reading::before {
		content: '';
		position: absolute;
		top: 0;
		left: var(--space-6);
		right: var(--space-6);
		height: 3px;
		background: linear-gradient(90deg, transparent, var(--color-highlight), transparent);
		border-radius: 0 0 var(--radius-0-5) var(--radius-0-5);
	}

	.now-reading-button {
		display: block;
		width: 100%;
		padding: var(--space-7);
		text-decoration: none;
		color: inherit;
		cursor: pointer;
		border: none;
		background: transparent;
		text-align: left;
		font: inherit;
		transition: background 0.2s;
	}

	.now-reading-button:hover {
		background: var(--color-accent-secondary-tint);
	}

	.now-reading-label {
		font-family: var(--font-sans);
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-subtle);
		letter-spacing: var(--letter-10);
		text-transform: uppercase;
		display: block;
		margin-bottom: var(--space-2);
	}

	.now-reading-title {
		font-family: var(--font-serif);
		font-size: var(--font-size-5);
		font-weight: 500;
		color: var(--color-text);
		margin: 0 0 var(--space-2);
		line-height: var(--line-height-1-3);
	}

	.now-reading-description {
		margin: 0 0 var(--space-4);
		color: var(--color-text-muted);
		font-size: var(--font-size-2);
		line-height: var(--line-height-3);
	}

	.continue-reading {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		color: var(--color-text-subtle);
		transition: color 0.2s;
	}

	.now-reading-button:hover .continue-reading {
		color: var(--color-accent-secondary);
	}

	/* Fleuron Divider */
	.fleuron {
		text-align: center;
		color: var(--color-highlight);
		font-size: var(--font-size-5);
		margin: var(--space-6) 0;
	}

	/* All Lessons Section */
	.lessons-section {
		margin-top: var(--space-4);
	}

	.section-heading {
		font-family: var(--font-display);
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-10);
		text-align: center;
		margin: 0 0 var(--space-3);
	}

	.section-divider {
		height: 1px;
		background: linear-gradient(90deg, transparent, var(--color-bg-soft), transparent);
		margin-bottom: var(--space-6);
	}

	/* Lesson Entries */
	.lesson-entries {
		display: flex;
		flex-direction: column;
		gap: var(--space-5);
	}

	.lesson-entry {
		display: flex;
		flex-direction: column;
		gap: var(--space-1-5);
		padding-left: var(--space-4);
		padding-bottom: var(--space-5);
		border-left: 2px solid var(--color-bg-soft);
		border-bottom: 1px solid var(--color-bg-soft);
		background: transparent;
		border-right: none;
		border-top: none;
		text-align: left;
		font: inherit;
		cursor: pointer;
		transition: border-left-color 0.2s, background 0.2s;
	}

	.lesson-entry:last-child {
		padding-bottom: 0;
		border-bottom: none;
	}

	.lesson-entry:hover {
		border-left-color: var(--color-highlight);
		background: var(--overlay-gold-08);
	}

	.lesson-entry.current {
		border-left-color: var(--color-highlight);
	}

	.lesson-entry.completed {
		border-left-color: var(--color-success);
	}

	.lesson-title {
		font-family: var(--font-serif);
		font-size: var(--font-size-4);
		font-weight: 500;
		color: var(--color-text);
		line-height: var(--line-height-1-3);
	}

	.lesson-description {
		margin: 0;
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		line-height: var(--line-height-3);
	}

	.empty-state {
		background: var(--color-bg-soft);
		border: 1px dashed var(--color-bg-pressed);
		padding: var(--space-4-5);
		border-radius: var(--radius-1-5);
		color: var(--color-text-muted);
		text-align: center;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.lesson-list {
			padding: var(--space-4) var(--space-3) var(--space-8);
		}

		.now-reading {
			border-radius: var(--radius-2);
		}

		.now-reading::before {
			left: var(--space-4);
			right: var(--space-4);
		}

		.now-reading-button {
			padding: var(--space-5) var(--space-6);
		}

		.now-reading-title {
			font-size: var(--font-size-4);
		}

		.lesson-title {
			font-size: var(--font-size-3);
		}
	}
</style>
