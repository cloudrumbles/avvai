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
	<div class="leaf-texture"></div>

	<aside class="sidebar">
		<div class="sidebar-content">
			<header class="sidebar-header">
				<a href="/" class="logo">avvai</a>
			</header>

			<div class="progress-card">
				<div class="progress-stats">
					<div class="stat-item">
						<span class="stat-value">{completedLessons}</span>
						<span class="stat-label">Completed</span>
					</div>
					<div class="stat-divider"></div>
					<div class="stat-item">
						<span class="stat-value">{totalLessons - completedLessons}</span>
						<span class="stat-label">Remaining</span>
					</div>
				</div>
				<div class="progress-bar-container">
					<div class="progress-bar">
						<div class="progress-fill" style="width: {progressPct}%"></div>
					</div>
					<span class="progress-text">{progressPct}%</span>
				</div>
			</div>

			<nav class="quick-nav">
				<a href="/lesson" class="nav-link">
					<span class="nav-icon">◈</span>
					<span class="nav-label">Lessons</span>
				</a>
				<a href="/flashcards" class="nav-link">
					<span class="nav-icon">◉</span>
					<span class="nav-label">Flashcards</span>
				</a>
			</nav>
		</div>
	</aside>

	<div class="main-content">
		<header class="mobile-header">
			<a href="/" class="logo">avvai</a>
		</header>

		<section class="greeting-section">
			<div class="greeting">
				<span class="greeting-tamil">வணக்கம்</span>
				<h1>Welcome back</h1>
			</div>
		</section>

		<section class="calendar-section">
			<div class="calendar-header">
				<span class="calendar-title">This Week</span>
			</div>
			<div class="week-grid">
				{#each ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su'] as day, i (day)}
					<div class="day" class:active={weekDays[i]} class:today={i === todayIndex}>
						<span class="day-name">{day}</span>
						<span class="day-mark">{weekDays[i] ? '●' : '○'}</span>
					</div>
				{/each}
			</div>
		</section>

		{#if currentLesson}
			<section class="continue-section">
				<div class="section-header">
					<span class="section-label">Continue Learning</span>
					<a href="/lesson" class="view-all">View all lessons →</a>
				</div>
				<a href="/lesson" class="continue-card">
					<div class="card-header">
						<span class="lesson-number">Lesson {currentLessonIndex}</span>
						<span class="progress-badge">{progressPct}% complete</span>
					</div>
					<h2 class="lesson-title">{currentLesson.title}</h2>
					{#if currentLesson.description}
						<p class="lesson-description">{currentLesson.description}</p>
					{/if}
					<div class="card-footer">
						<span class="continue-btn">Continue reading</span>
						<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
							<path d="M4 10H16M16 10L11 5M16 10L11 15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</div>
				</a>
			</section>
		{/if}

		<section class="lessons-preview">
			<div class="section-header">
				<span class="section-label">All Lessons</span>
				<a href="/lesson" class="view-all">See all →</a>
			</div>
			<div class="lessons-grid">
				{#each lessons.slice(0, 4) as lesson, index (lesson.id)}
					{@const status = progress?.[lesson.id] ? 'completed' : (currentLesson?.id === lesson.id ? 'current' : 'not_started')}
					<a href="/lesson" class="lesson-mini-card {status}">
						<div class="mini-card-header">
							<span class="mini-number">{index + 1}</span>
							{#if status === 'completed'}
								<span class="mini-check">✓</span>
							{/if}
						</div>
						<h3 class="mini-title">{lesson.title}</h3>
					</a>
				{/each}
			</div>
		</section>

		<section class="flashcards-cta">
			<a href="/flashcards" class="cta-card">
				<div class="cta-content">
					<span class="cta-icon">◉</span>
					<div class="cta-text">
						<h3>Practice with Flashcards</h3>
						<p>Review vocabulary and test your knowledge</p>
					</div>
				</div>
				<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
					<path d="M5 12H19M19 12L13 6M19 12L13 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
			</a>
		</section>
	</div>
</div>

<style>
	.home {
		min-height: 100dvh;
		display: flex;
		background: var(--color-bg);
		font-family: 'Cormorant Garamond', Georgia, serif;
		position: relative;
	}

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

	/* Sidebar */
	.sidebar {
		width: 350px;
		flex-shrink: 0;
		background: var(--color-bg);
		border-right: 1px solid var(--color-bg-soft);
		display: none;
	}

	.sidebar-content {
		padding: var(--space-6);
		position: sticky;
		top: 0;
	}

	.sidebar-header {
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

	/* Progress Card */
	.progress-card {
		background: var(--color-bg-soft);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-5);
		margin-bottom: var(--space-6);
	}

	.progress-stats {
		display: flex;
		align-items: center;
		justify-content: space-around;
		margin-bottom: var(--space-4);
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
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
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-highlight);
	}

	/* Quick Nav */
	.quick-nav {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		background: var(--color-bg);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-2);
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}

	.nav-link:hover {
		background: var(--color-accent-tint);
		border-color: var(--color-accent);
	}

	.nav-icon {
		font-size: var(--font-size-3);
		color: var(--color-accent);
	}

	.nav-label {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-2);
		font-weight: 600;
	}

	/* Main Content */
	.main-content {
		flex: 1;
		padding: var(--space-5) var(--space-4) var(--space-10);
		max-width: 600px;
		margin: 0 auto;
		width: 100%;
	}

	.mobile-header {
		margin-bottom: var(--space-6);
		padding-bottom: var(--space-4);
		border-bottom: 2px solid var(--color-bg-pressed);
	}

	/* Greeting Section */
	.greeting-section {
		margin-bottom: var(--space-6);
	}

	.greeting {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.greeting-tamil {
		font-family: 'Noto Serif Tamil', serif;
		font-size: var(--font-size-2);
		color: var(--color-accent);
		letter-spacing: var(--letter-2);
	}

	.greeting h1 {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-6);
		font-weight: 600;
		color: var(--color-text);
		margin: 0;
	}

	/* Calendar Section */
	.calendar-section {
		background: var(--color-bg);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-2);
		padding: var(--space-4);
		margin-bottom: var(--space-6);
	}

	.calendar-header {
		margin-bottom: var(--space-4);
	}

	.calendar-title {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-10);
	}

	.week-grid {
		display: flex;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.day {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-1);
		transition: background 0.2s;
		flex: 1;
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
	}

	.day.today {
		background: var(--color-accent);
	}

	.day.today .day-name {
		color: var(--color-bg);
	}

	.day.today .day-mark {
		color: var(--color-bg);
	}

	.day.active .day-mark {
		color: var(--color-accent);
	}

	/* Section Header */
	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-4);
	}

	.section-label {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: var(--letter-10);
	}

	.view-all {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		color: var(--color-accent);
		text-decoration: none;
		transition: color 0.2s;
	}

	.view-all:hover {
		color: var(--color-accent-strong);
	}

	/* Continue Card */
	.continue-section {
		margin-bottom: var(--space-6);
	}

	.continue-card {
		display: block;
		background: var(--color-bg-soft);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-5);
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
		position: relative;
		overflow: hidden;
	}

	.continue-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: linear-gradient(90deg, var(--color-highlight), var(--color-accent));
	}

	.continue-card:hover {
		background: var(--color-accent-tint);
		transform: translateY(-2px);
	}

	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-3);
	}

	.lesson-number {
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: var(--letter-4);
	}

	.progress-badge {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-1);
		font-weight: 600;
		color: var(--color-accent);
	}

	.lesson-title {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-4);
		font-weight: 600;
		color: var(--color-text);
		margin: 0 0 var(--space-2);
		line-height: var(--line-height-1-3);
	}

	.lesson-description {
		margin: 0 0 var(--space-4);
		color: var(--color-text-muted);
		font-size: var(--font-size-2);
		line-height: var(--line-height-3);
	}

	.card-footer {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.continue-btn {
		font-family: var(--font-sans);
		font-size: var(--font-size-1-5);
		font-weight: 600;
		color: var(--color-accent);
	}

	/* Lessons Preview Grid */
	.lessons-preview {
		margin-bottom: var(--space-6);
	}

	.lessons-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-3);
	}

	.lesson-mini-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-4);
		background: var(--color-bg);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-2);
		border-left: 3px solid var(--color-bg-soft);
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}

	.lesson-mini-card:hover {
		border-left-color: var(--color-accent);
		background: var(--color-bg-soft);
	}

	.lesson-mini-card.completed {
		border-left-color: var(--color-success);
	}

	.lesson-mini-card.current {
		border-left-color: var(--color-accent);
		background: var(--color-accent-tint);
	}

	.mini-card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.mini-number {
		font-size: var(--font-size-0-5);
		font-weight: 600;
		color: var(--color-text-muted);
	}

	.mini-check {
		font-size: var(--font-size-2);
		color: var(--color-success);
	}

	.mini-title {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-2);
		font-weight: 500;
		color: var(--color-text);
		margin: 0;
		line-height: var(--line-height-1-3);
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	/* Flashcards CTA */
	.cta-card {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-5);
		background: linear-gradient(135deg, var(--color-bg-soft) 0%, var(--color-accent-tint) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}

	.cta-card:hover {
		background: var(--color-accent-tint);
		transform: translateY(-2px);
	}

	.cta-content {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.cta-icon {
		font-size: var(--font-size-5);
		color: var(--color-accent);
	}

	.cta-text h3 {
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-3);
		font-weight: 600;
		color: var(--color-text);
		margin: 0 0 var(--space-1);
	}

	.cta-text p {
		margin: 0;
		color: var(--color-text-muted);
		font-size: var(--font-size-2);
	}

	/* Responsive */
	@media (min-width: 1024px) {
		.sidebar {
			display: block;
		}

		.main-content {
			max-width: none;
			margin: 0;
			padding: var(--space-6) var(--space-8) var(--space-10);
		}

		.mobile-header {
			display: none;
		}

		.lessons-grid {
			grid-template-columns: repeat(3, 1fr);
			gap: var(--space-4);
		}

		.continue-card {
			max-width: 600px;
		}
	}

	@media (min-width: 1400px) {
		.lessons-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	@media (max-width: 640px) {
		.main-content {
			padding: var(--space-4) var(--space-3) var(--space-8);
		}

		.greeting h1 {
			font-size: var(--font-size-5);
		}

		.lessons-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: var(--space-2);
		}

		.lesson-mini-card {
			padding: var(--space-3);
		}
	}
</style>
