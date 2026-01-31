<script lang="ts">
	// Home page - user dashboard
	// Mock data for now
	let currentLesson = $state({
		id: 'lesson-3',
		title: 'குறுந்தொகை',
		englishTitle: 'Kuruntokai',
		subtitle: 'Poem 40: The Jasmine Speaks',
		progress: 65,
		lastAccessed: 'Today'
	});

	let streak = $state({
		current: 7,
		best: 12,
		lastPracticed: 'Today',
		weekDays: [true, true, true, false, true, true, true] // Mon-Sun, current week
	});

	let stats = $state({
		lessonsCompleted: 3,
		wordsLearned: 47,
		flashcardsReviewed: 124,
		flashcardsDue: 12
	});
</script>

<svelte:head>
	<title>Home — avvai</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,400;0,600;1,400&family=Eczar:wght@400;600;700&display=swap" rel="stylesheet" />
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
				<span class="streak-count">{streak.current}</span>
				<span class="streak-label">day streak</span>
			</div>
		</div>
		<div class="band-line"></div>
	</section>

	<!-- Week Progress Band -->
	<section class="leaf-band week-band">
		<div class="week-row">
			{#each ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su'] as day, i}
				<div class="day" class:active={streak.weekDays[i]} class:today={i === 6}>
					<span class="day-name">{day}</span>
					<span class="day-mark">{streak.weekDays[i] ? '●' : '○'}</span>
				</div>
			{/each}
		</div>
		<div class="band-line"></div>
	</section>

	<!-- Continue Learning Band -->
	<section class="leaf-band continue-band">
		<a href="/lesson/{currentLesson.id}" class="continue-link">
			<div class="continue-row">
				<span class="label">Continue</span>
				<span class="meta">
					<span class="source-tamil">{currentLesson.title}</span>
					<span class="source-english">{currentLesson.englishTitle}</span>
				</span>
			</div>
			<div class="continue-row main-row">
				<h2 class="lesson-title">{currentLesson.subtitle}</h2>
				<span class="arrow">→</span>
			</div>
			<div class="continue-row progress-row">
				<div class="progress-track">
					<div class="progress-fill" style:width="{currentLesson.progress}%"></div>
				</div>
				<span class="progress-pct">{currentLesson.progress}%</span>
			</div>
		</a>
		<div class="band-line"></div>
	</section>

	<!-- Actions Band -->
	<section class="leaf-band actions-band">
		<a href="/flashcards" class="action-row">
			<span class="action-name">Flashcards</span>
			{#if stats.flashcardsDue > 0}
				<span class="action-count">{stats.flashcardsDue} due</span>
			{/if}
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
				<span class="stat-value">{stats.lessonsCompleted}</span>
				<span class="stat-label">lessons</span>
			</div>
			<div class="stat-divider"></div>
			<div class="stat">
				<span class="stat-value">{stats.wordsLearned}</span>
				<span class="stat-label">words</span>
			</div>
			<div class="stat-divider"></div>
			<div class="stat">
				<span class="stat-value">{stats.flashcardsReviewed}</span>
				<span class="stat-label">reviews</span>
			</div>
		</div>
	</section>
</div>

<style>
	:root {
		--font-serif: 'Cormorant Garamond', Georgia, serif;
		--font-heading: 'Eczar', Georgia, serif;

		/* Palm leaf manuscript palette */
		--c-leaf: #e8e0c8;             /* dried palm leaf */
		--c-leaf-light: #f2edd8;       /* lighter leaf */
		--c-leaf-dark: #d4c9a8;        /* aged leaf edge */
		--c-ink: #2d2418;              /* dark writing ink */
		--c-ink-light: #5c4d3a;        /* faded ink */
		--c-ink-faint: #8a7a62;        /* very faded */
		--c-stylus: #8b4513;           /* stylus/etching marks - sienna */
		--c-teal: #3d7a7a;             /* pond water accent */
		--c-terracotta: #c45c3e;       /* roof tile accent */
		--c-sage: #5c7a52;             /* vegetation */
	}

	.home {
		min-height: 100dvh;
		background: var(--c-leaf);
		font-family: var(--font-serif);
		max-width: 600px;
		margin: 0 auto;
		padding: 20px 16px 48px;
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
			var(--c-ink) 0px,
			var(--c-ink) 1px,
			transparent 1px,
			transparent 8px
		);
	}

	/* Header */
	.home-header {
		margin-bottom: 24px;
		padding-bottom: 16px;
		border-bottom: 2px solid var(--c-leaf-dark);
	}

	.logo {
		font-family: var(--font-heading);
		font-size: 1.4rem;
		font-weight: 700;
		color: var(--c-ink);
		text-decoration: none;
		letter-spacing: 0.02em;
	}

	/* Leaf Band - horizontal strips like manuscript lines */
	.leaf-band {
		background: var(--c-leaf-light);
		border-radius: 4px;
		margin-bottom: 12px;
		position: relative;
		overflow: hidden;
	}

	.band-line {
		height: 3px;
		background: linear-gradient(
			90deg,
			var(--c-leaf-dark) 0%,
			var(--c-leaf) 50%,
			var(--c-leaf-dark) 100%
		);
	}

	.band-line.thin {
		height: 1px;
		background: var(--c-leaf-dark);
		margin: 0 16px;
	}

	.band-content {
		padding: 20px 20px 16px;
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
		gap: 2px;
	}

	.greeting-tamil {
		font-family: 'Noto Serif Tamil', serif;
		font-size: 13px;
		color: var(--c-teal);
		letter-spacing: 0.02em;
	}

	.greeting h1 {
		font-family: var(--font-heading);
		font-size: 26px;
		font-weight: 600;
		color: var(--c-ink);
		margin: 0;
	}

	.streak {
		text-align: right;
	}

	.streak-count {
		display: block;
		font-family: var(--font-heading);
		font-size: 32px;
		font-weight: 700;
		color: var(--c-terracotta);
		line-height: 1;
	}

	.streak-label {
		font-size: 11px;
		color: var(--c-ink-faint);
		letter-spacing: 0.05em;
		text-transform: uppercase;
	}

	/* Week Band */
	.week-band {
		background: var(--c-leaf);
	}

	.week-row {
		display: flex;
		padding: 12px 8px;
	}

	.day {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		padding: 8px 4px;
		border-radius: 4px;
		transition: background 0.2s;
	}

	.day-name {
		font-size: 10px;
		font-weight: 600;
		color: var(--c-ink-faint);
		letter-spacing: 0.05em;
		text-transform: uppercase;
	}

	.day-mark {
		font-size: 14px;
		color: var(--c-leaf-dark);
		transition: color 0.2s;
	}

	.day.active .day-mark {
		color: var(--c-teal);
	}

	.day.today {
		background: var(--c-terracotta);
		border-radius: 4px;
	}

	.day.today .day-name {
		color: rgba(255,255,255,0.8);
	}

	.day.today .day-mark {
		color: white;
	}

	/* Continue Band */
	.continue-band {
		background: var(--c-leaf-light);
	}

	.continue-link {
		display: block;
		padding: 16px 20px;
		text-decoration: none;
		color: inherit;
		transition: background 0.2s;
	}

	.continue-link:hover {
		background: rgba(61, 122, 122, 0.04);
	}

	.continue-row {
		display: flex;
		align-items: baseline;
		gap: 12px;
	}

	.continue-row.main-row {
		margin: 8px 0 12px;
		align-items: center;
	}

	.continue-row.progress-row {
		align-items: center;
	}

	.label {
		font-size: 10px;
		font-weight: 600;
		color: var(--c-ink-faint);
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	.meta {
		display: flex;
		align-items: baseline;
		gap: 8px;
	}

	.source-tamil {
		font-family: 'Noto Serif Tamil', serif;
		font-size: 13px;
		color: var(--c-teal);
	}

	.source-english {
		font-size: 12px;
		color: var(--c-ink-faint);
		font-style: italic;
	}

	.lesson-title {
		flex: 1;
		font-family: var(--font-heading);
		font-size: 20px;
		font-weight: 600;
		color: var(--c-ink);
		margin: 0;
		line-height: 1.25;
	}

	.arrow {
		font-size: 20px;
		color: var(--c-ink-faint);
		transition: color 0.2s, transform 0.2s;
	}

	.continue-link:hover .arrow {
		color: var(--c-teal);
		transform: translateX(4px);
	}

	.progress-track {
		flex: 1;
		height: 3px;
		background: var(--c-leaf-dark);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--c-sage);
		border-radius: 2px;
	}

	.progress-pct {
		font-family: var(--font-heading);
		font-size: 12px;
		font-weight: 600;
		color: var(--c-ink-faint);
		min-width: 32px;
		text-align: right;
	}

	/* Actions Band */
	.actions-band {
		background: var(--c-leaf-light);
	}

	.action-row {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 20px;
		text-decoration: none;
		color: inherit;
		transition: background 0.2s;
	}

	.action-row:hover {
		background: rgba(61, 122, 122, 0.04);
	}

	.action-name {
		flex: 1;
		font-family: var(--font-heading);
		font-size: 16px;
		font-weight: 600;
		color: var(--c-ink);
	}

	.action-count {
		font-size: 11px;
		font-weight: 600;
		color: var(--c-terracotta);
		background: rgba(196, 92, 62, 0.12);
		padding: 3px 8px;
		border-radius: 3px;
	}

	.action-row:hover .arrow {
		color: var(--c-teal);
		transform: translateX(4px);
	}

	/* Stats Band */
	.stats-band {
		background: var(--c-leaf);
		border: 2px solid var(--c-leaf-dark);
	}

	.stats-row {
		display: flex;
		padding: 16px 12px;
	}

	.stat {
		flex: 1;
		text-align: center;
		padding: 8px;
	}

	.stat-divider {
		width: 1px;
		background: var(--c-leaf-dark);
		margin: 4px 0;
	}

	.stat-value {
		display: block;
		font-family: var(--font-heading);
		font-size: 28px;
		font-weight: 700;
		color: var(--c-teal);
		line-height: 1;
	}

	.stat-label {
		display: block;
		font-size: 11px;
		color: var(--c-ink-faint);
		margin-top: 4px;
		letter-spacing: 0.03em;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.home {
			padding: 16px 12px 40px;
		}

		.greeting h1 {
			font-size: 22px;
		}

		.streak-count {
			font-size: 28px;
		}

		.lesson-title {
			font-size: 18px;
		}

		.stat-value {
			font-size: 24px;
		}

		.day-name {
			font-size: 9px;
		}
	}
</style>
