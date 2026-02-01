<script lang="ts">
	import type { LessonSummary } from 'avvai-frontend/types/lesson';

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

	function handleLessonClick(id: string) {
		onselect(id);
	}

	type LessonStatus = 'completed' | 'current' | 'not_started';

	function getStatus(lesson: LessonSummary): LessonStatus {
		if (progress?.[lesson.id]) return 'completed';
		if (currentLesson?.id === lesson.id) return 'current';
		return 'not_started';
	}

	function getStatusMarker(status: LessonStatus): string {
		switch (status) {
			case 'completed':
				return '✓';
			case 'current':
				return '●';
			case 'not_started':
				return '○';
		}
	}
</script>

<div class="lesson-list">
	<!-- Overall Progress Band -->
	<section class="leaf-band progress-band">
		<div class="band-content">
			<span class="label">YOUR JOURNEY</span>
			<div class="progress-main">
				<span class="progress-text">{completedLessons} of {totalLessons} lessons completed</span>
			</div>
			<div class="progress-track">
				<div class="progress-fill" style="width: {progressPct}%"></div>
			</div>
		</div>
		<div class="band-line"></div>
	</section>

	<!-- Now Reading Band -->
	{#if currentLesson}
		<section class="leaf-band now-reading-band">
			<button class="now-reading-link" onclick={() => handleLessonClick(currentLesson.id)}>
				<div class="now-reading-header">
					<span class="label">NOW READING</span>
					<span class="chapter-meta">
						<span class="chapter-english">Lesson {lessons.findIndex((lesson) => lesson.id === currentLesson.id) + 1}</span>
					</span>
				</div>
				<div class="now-reading-main">
					<h2 class="section-title">{currentLesson.title}</h2>
					<span class="arrow">→</span>
				</div>
				{#if currentLesson.description}
					<p class="now-reading-description">{currentLesson.description}</p>
				{/if}
			</button>
			<div class="band-line"></div>
		</section>
	{/if}

	<!-- Lessons -->
	<section class="lessons-section">
		<h2 class="chapters-heading">All Lessons</h2>

		{#if lessons.length === 0}
			<div class="empty-state">
				<p>No lessons are available yet. Check back soon.</p>
			</div>
		{:else}
			{#each lessons as lesson, index (lesson.id)}
				{@const status = getStatus(lesson)}
				<button
					class="lesson-card"
					class:completed={status === 'completed'}
					class:current={status === 'current'}
					onclick={() => handleLessonClick(lesson.id)}
				>
					<div class="lesson-number-circle">
						<span class="lesson-number">{index + 1}</span>
					</div>
					<div class="lesson-content">
						<div class="lesson-title-row">
							<span class="lesson-title">{lesson.title}</span>
							<span class="lesson-status">{status.replace('_', ' ')}</span>
						</div>
						{#if lesson.description}
							<p class="lesson-description">{lesson.description}</p>
						{/if}
						<div class="lesson-footer">
							<span class="lesson-marker">{getStatusMarker(status)}</span>
							<span class="lesson-action">Open</span>
						</div>
					</div>
				</button>
			{/each}
		{/if}
	</section>
</div>

<style>


	.lesson-list {
		min-height: 100dvh;
		background: var(--color-bg);
		font-family: var(--font-serif);
		max-width: 600px;
		margin: 0 auto;
		padding: var(--space-5) var(--space-4) var(--space-10);
	}

	/* Leaf Band base styles */
	.leaf-band {
		background: var(--color-bg-soft);
		border-radius: var(--radius-1);
		margin-bottom: var(--space-3);
		position: relative;
		overflow: hidden;
	}

	.band-content {
		padding: var(--space-4) var(--space-5);
	}

	.band-line {
		height: var(--space-0-75);
		background: linear-gradient(90deg, var(--color-bg-pressed) 0%, var(--color-bg) 50%, var(--color-bg-pressed) 100%);
	}

	.label {
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-subtle);
		letter-spacing: var(--letter-10);
		text-transform: uppercase;
	}

	/* Progress Band */
	.progress-band .band-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.progress-main {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
	}

	.progress-text {
		font-family: var(--font-display);
		font-size: var(--font-size-3);
		font-weight: 600;
		color: var(--color-text);
	}

	.progress-track {
		height: var(--space-0-75);
		background: var(--color-bg-pressed);
		border-radius: var(--radius-0-5);
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--color-success);
		border-radius: var(--radius-0-5);
		transition: width 0.3s ease;
	}

	/* Now Reading Band */
	.now-reading-band {
		background: var(--color-bg-soft);
	}

	.now-reading-link {
		display: block;
		width: 100%;
		padding: var(--space-4) var(--space-5);
		text-decoration: none;
		color: inherit;
		transition: background 0.2s;
		cursor: pointer;
		border: none;
		background: transparent;
		text-align: left;
		font: inherit;
	}

	.now-reading-link:hover {
		background: var(--color-accent-secondary-tint);
	}

	.now-reading-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-3);
		flex-wrap: wrap;
	}

	.chapter-meta {
		display: flex;
		align-items: baseline;
		gap: var(--space-1-5);
	}

	.chapter-english {
		font-size: var(--font-size-1-5);
		color: var(--color-text-muted);
		font-style: italic;
	}

	.now-reading-main {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		margin: var(--space-2-5) 0 var(--space-2-5);
	}

	.section-title {
		flex: 1;
		font-family: var(--font-display);
		font-size: var(--font-size-4);
		font-weight: 600;
		color: var(--color-text);
		margin: 0;
		line-height: var(--line-height-1-3);
	}

	.now-reading-description {
		margin: 0 0 var(--space-1);
		color: var(--color-text-muted);
		font-size: var(--font-size-1-5);
	}

	.arrow {
		font-size: var(--font-size-4-5);
		color: var(--color-text-subtle);
		transition:
			color 0.2s,
			transform 0.2s;
	}

	.now-reading-link:hover .arrow {
		color: var(--color-accent-secondary);
		transform: translateX(4px);
	}

	/* Lessons Section */
	.lessons-section {
		margin-top: var(--space-6);
	}

	.chapters-heading {
		font-family: var(--font-display);
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-8);
		margin: 0 0 var(--space-3) var(--space-1);
	}

	.lesson-card {
		background: var(--color-bg-soft);
		border-radius: var(--radius-1-5);
		margin-bottom: var(--space-3);
		border: 1px solid transparent;
		transition: border-color 0.2s, transform 0.2s, box-shadow 0.2s;
		position: relative;
		padding: var(--space-4) var(--space-4) var(--space-4) 52px;
		text-align: left;
		font: inherit;
		width: 100%;
		cursor: pointer;
	}

	.lesson-card:hover {
		border-color: var(--color-bg-pressed);
		box-shadow: var(--shadow-soft);
		transform: translateY(-2px);
	}

	.lesson-card.current {
		border-color: var(--color-accent-secondary);
	}

	.lesson-card.completed {
		border-color: var(--color-success);
	}

	.lesson-number-circle {
		position: absolute;
		left: var(--space-3);
		top: var(--space-5);
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background: var(--color-text);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-bg-soft);
		font-family: var(--font-display);
		font-size: var(--font-size-2);
	}

	.lesson-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.lesson-title-row {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		justify-content: space-between;
	}

	.lesson-title {
		font-family: var(--font-display);
		font-size: var(--font-size-4);
		color: var(--color-text);
		font-weight: 600;
	}

	.lesson-status {
		font-size: var(--font-size-0-75);
		text-transform: uppercase;
		letter-spacing: var(--letter-8);
		color: var(--color-text-subtle);
	}

	.lesson-description {
		margin: 0;
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
	}

	.lesson-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		color: var(--color-text-subtle);
		font-size: var(--font-size-1);
		letter-spacing: var(--letter-4);
		text-transform: uppercase;
	}

	.lesson-marker {
		font-size: var(--font-size-2);
		color: var(--color-accent-secondary);
	}

	.lesson-card.completed .lesson-marker {
		color: var(--color-success);
	}

	.lesson-card.current .lesson-marker {
		color: var(--color-accent-secondary);
	}

	.lesson-action {
		font-family: var(--font-display);
		color: var(--color-text);
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
	@media (max-width: 480px) {
		.lesson-list {
			padding: var(--space-4) var(--space-3) var(--space-8);
		}

		.lesson-card {
			padding: var(--space-4) var(--space-3) var(--space-4) 46px;
		}

		.lesson-title {
			font-size: var(--font-size-3);
		}

		.progress-text {
			font-size: var(--font-size-2);
		}

		.section-title {
			font-size: var(--font-size-3);
		}
	}
</style>
