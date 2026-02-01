<script lang="ts">
	import type { LessonSummary } from '$lib/types/lesson';

	interface Props {
		data: {
			lessons: LessonSummary[];
			progress: Record<string, boolean>;
		};
	}

	let { data }: Props = $props();

	const lessons = $derived(data.lessons ?? []);
	const progress = $derived(data.progress ?? {});

	const totalLessons = $derived(lessons.length);
	const completedLessons = $derived(lessons.filter((lesson) => progress?.[lesson.id]).length);
	const currentLesson = $derived(lessons.find((lesson) => !progress?.[lesson.id]) ?? null);
	const currentLessonIndex = $derived(
		currentLesson ? lessons.findIndex((lesson) => lesson.id === currentLesson.id) + 1 : 0
	);
	const progressPct = $derived(totalLessons > 0 ? Math.round((completedLessons / totalLessons) * 100) : 0);

	const todayIndex = $derived((() => {
		const day = new Date().getDay();
		return day === 0 ? 6 : day - 1;
	})());

	const weekDays = $derived(new Array(7).fill(false));
</script>

<svelte:head>
	<title>Home — avvai</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,400;0,600;1,400&family=Eczar:wght@400;600;700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="home">
	<!-- Palm leaf texture overlay -->
	<div class="leaf-texture"></div>

	<header class="home-header">
		<a href="/" class="logo">avvai</a>
	</header>

	<!-- Greeting Band -->
	<section class="leaf-band greeting-band">
		<div class="band-content">
			<div class="greeting">
				<span class="greeting-tamil">வணக்கம்</span>
				<h1>Welcome back</h1>
			</div>
			<div class="streak">
				<span class="streak-count">{completedLessons}</span>
				<span class="streak-label">lessons completed</span>
			</div>
		</div>
		<div class="band-line"></div>
	</section>

	<!-- Week Progress Band -->
	<section class="leaf-band week-band">
		<div class="week-row">
		{#each ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su'] as day, i (day)}
				<div class="day" class:active={weekDays[i]} class:today={i === todayIndex}>
					<span class="day-name">{day}</span>
					<span class="day-mark">{weekDays[i] ? '●' : '○'}</span>
				</div>
			{/each}
		</div>
		<div class="band-line"></div>
	</section>

	<!-- Continue Learning Band -->
	{#if currentLesson}
		<section class="leaf-band continue-band">
			<a href="/lesson" class="continue-link">
				<div class="continue-row">
					<span class="label">Continue</span>
					<span class="meta">
						<span class="source-english">Lesson {currentLessonIndex}</span>
					</span>
				</div>
				<div class="continue-row main-row">
					<h2 class="lesson-title">{currentLesson.title}</h2>
					<span class="arrow">→</span>
				</div>
				{#if currentLesson.description}
					<p class="continue-description">{currentLesson.description}</p>
				{/if}
				<div class="continue-row progress-row">
					<div class="progress-track">
						<div class="progress-fill" style="width: {progressPct}%"></div>
					</div>
					<span class="progress-pct">{progressPct}%</span>
				</div>
			</a>
			<div class="band-line"></div>
		</section>
	{:else}
		<section class="leaf-band continue-band">
			<div class="continue-link empty-continue">
				<div class="continue-row">
					<span class="label">Continue</span>
				</div>
				<div class="continue-row main-row">
					<h2 class="lesson-title">No lessons available yet</h2>
				</div>
			</div>
			<div class="band-line"></div>
		</section>
	{/if}

	<!-- Actions Band -->
	<section class="leaf-band actions-band">
		<a href="/flashcards" class="action-row">
			<span class="action-name">Flashcards</span>
			<span class="arrow">→</span>
		</a>
		<div class="band-line thin"></div>
		<a href="/lesson" class="action-row">
			<span class="action-name">All Lessons</span>
			<span class="arrow">→</span>
		</a>
		<div class="band-line"></div>
	</section>

	<!-- Stats Band -->
	<section class="leaf-band stats-band">
		<div class="stats-row">
			<div class="stat">
				<span class="stat-value">{completedLessons}</span>
				<span class="stat-label">lessons</span>
			</div>
		</div>
	</section>
</div>

<style>


	.home {
		min-height: 100dvh;
		background: var(--color-bg);
		font-family: 'Cormorant Garamond', Georgia, serif;
		max-width: 600px;
		margin: 0 auto;
		padding: var(--space-5) var(--space-4) var(--space-10);
		position: relative;
	}

	/* Subtle horizontal line texture like palm leaf */
	.leaf-texture {
		position: fixed;
		inset: 0;
		pointer-events: none;
		opacity: 0.03;
		background-image: repeating-linear-gradient(
			0deg,
			var(--color-text) 0px,
			var(--color-text) 1px,
			transparent 1px,
			transparent 8px
		);
	}

	/* Header */
	.home-header {
		margin-bottom: var(--space-6);
		padding-bottom: var(--space-4);
		border-bottom: 2px solid var(--color-bg-pressed);
	}

	.logo {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-5);
		font-weight: 700;
		color: var(--color-text);
		text-decoration: none;
		letter-spacing: var(--letter-2);
	}

	/* Leaf Band - horizontal strips like manuscript lines */
	.leaf-band {
		background: var(--color-bg-soft);
		border-radius: var(--radius-1);
		margin-bottom: var(--space-3);
		position: relative;
		overflow: hidden;
	}

	.band-line {
		height: 3px;
		background: linear-gradient(
			90deg,
			var(--color-bg-pressed) 0%,
			var(--color-bg) 50%,
			var(--color-bg-pressed) 100%
		);
	}

	.band-line.thin {
		height: 1px;
		background: var(--color-bg-pressed);
		margin: 0 var(--space-4);
	}

	.band-content {
		padding: var(--space-5) var(--space-5) var(--space-4);
	}

	/* Greeting Band */
	.greeting-band .band-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.greeting {
		display: flex;
		flex-direction: column;
		gap: var(--space-0-5);
	}

	.greeting-tamil {
		font-family: 'Noto Serif Tamil', serif;
		font-size: var(--font-size-1-5);
		color: var(--color-accent-secondary);
		letter-spacing: var(--letter-2);
	}

	.greeting h1 {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-6-5);
		font-weight: 600;
		color: var(--color-text);
		margin: 0;
	}

	.streak {
		text-align: right;
	}

	.streak-count {
		display: block;
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-8);
		font-weight: 700;
		color: var(--color-accent-secondary);
		line-height: var(--line-height-0);
	}

	.streak-label {
		font-size: var(--font-size-0-75);
		color: var(--color-text-subtle);
		letter-spacing: var(--letter-5);
		text-transform: uppercase;
	}

	/* Week Band */
	.week-band {
		background: var(--color-bg);
	}

	.week-row {
		display: flex;
		padding: var(--space-3) var(--space-2);
	}

	.day {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-2) var(--space-1);
		border-radius: var(--radius-1);
		transition: background 0.2s;
	}

	.day-name {
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-subtle);
		letter-spacing: var(--letter-5);
		text-transform: uppercase;
	}

	.day-mark {
		font-size: var(--font-size-2);
		color: var(--color-bg-pressed);
		transition: color 0.2s;
	}

	.day.active .day-mark {
		color: var(--color-accent-secondary);
	}

	.day.today {
		background: var(--color-accent-secondary);
		border-radius: var(--radius-1);
	}

	.day.today .day-name {
		color: var(--color-bg);
		opacity: 0.85;
	}

	.day.today .day-mark {
		color: white;
	}

	/* Continue Band */
	.continue-band {
		background: var(--color-bg-soft);
	}

	.continue-link {
		display: block;
		padding: var(--space-4) var(--space-5);
		text-decoration: none;
		color: inherit;
		transition: background 0.2s;
	}

	.continue-link:hover {
		background: var(--color-accent-secondary-tint);
	}

	.continue-link.empty-continue {
		cursor: default;
	}

	.continue-row {
		display: flex;
		align-items: baseline;
		gap: var(--space-3);
	}

	.continue-row.main-row {
		margin: var(--space-2) 0 var(--space-3);
		align-items: center;
	}

	.continue-row.progress-row {
		align-items: center;
	}

	.label {
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-subtle);
		letter-spacing: var(--letter-10);
		text-transform: uppercase;
	}

	.meta {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
	}

	.source-english {
		font-size: var(--font-size-1);
		color: var(--color-text-subtle);
		font-style: italic;
	}

	.lesson-title {
		flex: 1;
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-4-5);
		font-weight: 600;
		color: var(--color-text);
		margin: 0;
		line-height: var(--line-height-1-25);
	}

	.continue-description {
		margin: 0 0 var(--space-2-5);
		color: var(--color-text-muted);
		font-size: var(--font-size-1-5);
	}

	.arrow {
		font-size: var(--font-size-4-5);
		color: var(--color-text-subtle);
		transition: color 0.2s, transform 0.2s;
	}

	.continue-link:hover .arrow {
		color: var(--color-accent-secondary);
		transform: translateX(4px);
	}

	.progress-track {
		flex: 1;
		height: 3px;
		background: var(--color-bg-pressed);
		border-radius: var(--radius-0-5);
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--color-success);
		border-radius: var(--radius-0-5);
	}

	.progress-pct {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		min-width: 32px;
		text-align: right;
	}

	/* Actions Band */
	.actions-band {
		background: var(--color-bg-soft);
	}

	.action-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: 14px var(--space-5);
		text-decoration: none;
		color: inherit;
		transition: background 0.2s;
	}

	.action-row:hover {
		background: var(--color-accent-secondary-tint);
	}

	.action-name {
		flex: 1;
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-3);
		font-weight: 600;
		color: var(--color-text);
	}

	.action-row:hover .arrow {
		color: var(--color-accent-secondary);
		transform: translateX(4px);
	}

	/* Stats Band */
	.stats-band {
		background: var(--color-bg);
		border: 2px solid var(--color-bg-pressed);
	}

	.stats-row {
		display: flex;
		padding: var(--space-4) var(--space-3);
		justify-content: center;
	}

	.stat {
		flex: 0 0 auto;
		text-align: center;
		padding: var(--space-2);
	}

	.stat-value {
		display: block;
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-6);
		font-weight: 700;
		color: var(--color-accent-secondary);
		line-height: var(--line-height-0);
	}

	.stat-label {
		display: block;
		font-size: var(--font-size-0-75);
		color: var(--color-text-subtle);
		margin-top: var(--space-1);
		letter-spacing: var(--letter-3);
	}

	/* Responsive */
	@media (max-width: 480px) {
		.home {
			padding: var(--space-4) var(--space-3) var(--space-8);
		}

		.greeting h1 {
			font-size: var(--font-size-5);
		}

		.streak-count {
			font-size: var(--font-size-6);
		}

		.lesson-title {
			font-size: var(--font-size-4);
		}

		.stat-value {
			font-size: var(--font-size-5-5);
		}

		.day-name {
			font-size: var(--font-size-0);
		}
	}
</style>
