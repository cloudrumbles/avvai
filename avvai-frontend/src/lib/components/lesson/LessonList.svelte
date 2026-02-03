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

	function getStatusLabel(status: LessonStatus): string {
		switch (status) {
			case 'completed': return 'Completed';
			case 'current': return 'In Progress';
			default: return 'Not Started';
		}
	}
</script>

<div class="lesson-list-container">
	<aside class="sidebar">
		<div class="sidebar-content">
			<div class="progress-card">
				<div class="progress-stats">
					<div class="stat">
						<span class="stat-value">{completedLessons}</span>
						<span class="stat-label">Completed</span>
					</div>
					<div class="stat-divider"></div>
					<div class="stat">
						<span class="stat-value">{totalLessons - completedLessons}</span>
						<span class="stat-label">Remaining</span>
					</div>
				</div>
				<div class="progress-bar-container">
					<div class="progress-bar">
						<div class="progress-fill" style="width: {progressPct}%"></div>
					</div>
					<span class="progress-text">{Math.round(progressPct)}%</span>
				</div>
			</div>

			{#if currentLesson}
				<div class="now-reading-sidebar">
					<span class="now-reading-label">Now Reading</span>
					<h3 class="now-reading-title-sidebar">{currentLesson.title}</h3>
					{#if currentLesson.description}
						<p class="now-reading-desc-sidebar">{currentLesson.description}</p>
					{/if}
					<button class="continue-btn" onclick={() => handleLessonClick(currentLesson.id)}>
						Continue
						<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
							<path d="M3 8H13M13 8L9 4M13 8L9 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</button>
				</div>
			{/if}
		</div>
	</aside>

	<div class="main-content">
		<header class="mobile-header">
			<span class="mobile-progress">Lesson {currentLessonIndex} of {totalLessons}</span>
			<div class="mobile-progress-track">
				<div class="mobile-progress-fill" style="width: {progressPct}%"></div>
			</div>
		</header>

		{#if currentLesson}
			<section class="now-reading-mobile">
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

		<div class="fleuron-mobile">❧</div>

		<section class="lessons-section">
			<div class="section-header">
				<h2 class="section-heading">All Lessons</h2>
				<span class="lesson-count">{lessons.length} lessons</span>
			</div>

			{#if lessons.length === 0}
				<div class="empty-state">
					<p>No lessons are available yet. Check back soon.</p>
				</div>
			{:else}
				<div class="lessons-grid">
					{#each lessons as lesson, index (lesson.id)}
						{@const status = getStatus(lesson)}
						<button
							class="lesson-card"
							class:completed={status === 'completed'}
							class:current={status === 'current'}
							onclick={() => handleLessonClick(lesson.id)}
						>
							<div class="card-header">
								<span class="lesson-number">{index + 1}</span>
								<span class="status-badge {status}">{getStatusLabel(status)}</span>
							</div>
							<h3 class="card-title">{lesson.title}</h3>
							{#if lesson.description}
								<p class="card-description">{lesson.description}</p>
							{/if}
							<div class="card-footer">
								{#if status === 'completed'}
									<span class="checkmark">✓</span>
								{:else if status === 'current'}
									<span class="continue-hint">Click to read</span>
								{/if}
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	.lesson-list-container {
		display: flex;
		min-height: 100dvh;
		background: var(--color-bg);
	}

	.sidebar {
		width: 350px;
		flex-shrink: 0;
		background: var(--color-bg);
		border-right: 1px solid var(--color-bg-soft);
		display: none;
	}

	.sidebar-content {
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		position: sticky;
		top: 0;
	}

	.progress-card {
		background: var(--color-bg-soft);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-5);
	}

	.progress-stats {
		display: flex;
		align-items: center;
		justify-content: space-around;
		margin-bottom: var(--space-4);
	}

	.stat {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
	}

	.stat-value {
		font-family: var(--font-serif);
		font-size: var(--font-size-6);
		font-weight: 600;
		color: var(--color-text);
	}

	.stat-label {
		font-family: var(--font-sans);
		font-size: var(--font-size-1);
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: var(--letter-4);
	}

	.stat-divider {
		width: 1px;
		height: 40px;
		background: var(--color-bg-pressed);
	}

	.progress-bar-container {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.progress-bar {
		flex: 1;
		height: var(--space-1);
		background: var(--color-bg-pressed);
		border-radius: var(--radius-1);
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, var(--color-highlight), var(--color-accent));
		border-radius: var(--radius-1);
		transition: width 0.3s ease;
	}

	.progress-text {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-highlight);
	}

	.now-reading-sidebar {
		background: linear-gradient(135deg, var(--color-bg-soft) 0%, var(--color-accent-tint) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-5);
		position: relative;
		overflow: hidden;
	}

	.now-reading-sidebar::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: linear-gradient(90deg, var(--color-highlight), var(--color-accent));
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

	.now-reading-title-sidebar {
		font-family: var(--font-serif);
		font-size: var(--font-size-4);
		font-weight: 500;
		color: var(--color-text);
		margin: 0 0 var(--space-2);
		line-height: var(--line-height-1-3);
	}

	.now-reading-desc-sidebar {
		margin: 0 0 var(--space-4);
		color: var(--color-text-muted);
		font-size: var(--font-size-2);
		line-height: var(--line-height-3);
	}

	.continue-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-3) var(--space-4);
		background: var(--color-accent);
		color: var(--color-bg);
		border: none;
		border-radius: var(--radius-2);
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s;
	}

	.continue-btn:hover {
		background: var(--color-accent-strong);
	}

	.main-content {
		flex: 1;
		padding: var(--space-5) var(--space-4) var(--space-10);
		max-width: 600px;
		margin: 0 auto;
		width: 100%;
	}

	.mobile-header {
		text-align: center;
		margin-bottom: var(--space-7);
	}

	.mobile-progress {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		display: block;
		margin-bottom: var(--space-2);
	}

	.mobile-progress-track {
		height: var(--space-0-75);
		background: var(--color-bg-pressed);
		border-radius: var(--radius-0-5);
		overflow: hidden;
		max-width: 200px;
		margin: 0 auto;
	}

	.mobile-progress-fill {
		height: 100%;
		background: var(--color-highlight);
		border-radius: var(--radius-0-5);
		transition: width 0.3s ease;
	}

	.now-reading-mobile {
		background: linear-gradient(180deg, var(--color-bg-soft) 0%, var(--color-bg) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		box-shadow: var(--shadow-warm);
		position: relative;
		overflow: hidden;
		margin-bottom: var(--space-7);
	}

	.now-reading-mobile::before {
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

	.fleuron-mobile {
		text-align: center;
		color: var(--color-highlight);
		font-size: var(--font-size-5);
		margin: var(--space-6) 0;
	}

	.lessons-section {
		margin-top: var(--space-4);
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-6);
		padding-bottom: var(--space-3);
		border-bottom: 1px solid var(--color-bg-soft);
	}

	.section-heading {
		font-family: var(--font-display);
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-10);
		margin: 0;
	}

	.lesson-count {
		font-family: var(--font-sans);
		font-size: var(--font-size-1);
		color: var(--color-text-muted);
	}

	.lessons-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
	}

	.lesson-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-5);
		background: var(--color-bg);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-2);
		border-left: 3px solid var(--color-bg-soft);
		text-align: left;
		font: inherit;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.lesson-card:hover {
		border-left-color: var(--color-highlight);
		background: var(--color-bg-soft);
		transform: translateX(4px);
	}

	.lesson-card.completed {
		border-left-color: var(--color-success);
	}

	.lesson-card.current {
		border-left-color: var(--color-highlight);
		background: var(--color-accent-tint);
	}

	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-2);
	}

	.lesson-number {
		font-family: var(--font-sans);
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg-soft);
		border-radius: var(--radius-5);
	}

	.status-badge {
		font-family: var(--font-sans);
		font-size: var(--font-size-0-5);
		font-weight: 600;
		padding: var(--space-0-5) var(--space-2);
		border-radius: var(--radius-5);
		text-transform: uppercase;
		letter-spacing: var(--letter-4);
	}

	.status-badge.not_started {
		background: var(--color-bg-pressed);
		color: var(--color-text-muted);
	}

	.status-badge.current {
		background: var(--color-accent-tint);
		color: var(--color-accent);
	}

	.status-badge.completed {
		background: var(--color-success-tint);
		color: var(--color-success);
	}

	.card-title {
		font-family: var(--font-serif);
		font-size: var(--font-size-3);
		font-weight: 500;
		color: var(--color-text);
		margin: 0;
		line-height: var(--line-height-1-3);
	}

	.card-description {
		margin: 0;
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		line-height: var(--line-height-3);
	}

	.card-footer {
		display: flex;
		justify-content: flex-end;
		margin-top: auto;
		padding-top: var(--space-3);
	}

	.checkmark {
		font-size: var(--font-size-3);
		color: var(--color-success);
	}

	.continue-hint {
		font-family: var(--font-sans);
		font-size: var(--font-size-1);
		color: var(--color-highlight);
	}

	.empty-state {
		background: var(--color-bg-soft);
		border: 1px dashed var(--color-bg-pressed);
		padding: var(--space-4-5);
		border-radius: var(--radius-1-5);
		color: var(--color-text-muted);
		text-align: center;
	}

	@media (min-width: 1024px) {
		.sidebar {
			display: block;
		}

		.main-content {
			max-width: none;
			margin: 0;
			padding: var(--space-6) var(--space-8) var(--space-10);
		}

		.mobile-header,
		.now-reading-mobile,
		.fleuron-mobile {
			display: none;
		}

		.lessons-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: var(--space-5);
		}
	}

	@media (min-width: 1400px) {
		.lessons-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 640px) {
		.main-content {
			padding: var(--space-4) var(--space-3) var(--space-8);
		}

		.now-reading-mobile {
			border-radius: var(--radius-2);
		}

		.now-reading-mobile::before {
			left: var(--space-4);
			right: var(--space-4);
		}

		.now-reading-button {
			padding: var(--space-5) var(--space-6);
		}

		.now-reading-title {
			font-size: var(--font-size-4);
		}
	}
</style>
