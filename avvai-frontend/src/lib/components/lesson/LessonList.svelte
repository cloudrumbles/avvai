<script lang="ts">
	import type { LessonSummary } from 'avvai-frontend/types/lesson';

	interface Props {
		lessons: LessonSummary[];
		progress: Record<string, boolean>;
		onselect: (id: string) => void;
	}

	let { lessons, progress, onselect }: Props = $props();

	// Types for chapter/section structure
	interface Section {
		id: string;
		number: number;
		title: string;
		status: 'completed' | 'current' | 'not_started';
	}

	interface ChapterMetadata {
		topic: string; // தெரிந்துகொள்வோம்
		source: string; // இலக்கியம்
		thinaiDescription: string; // தினை description
	}

	interface Chapter {
		id: string;
		number: number;
		tamilName: string;
		englishName: string;
		landscape: string;
		thinaiKey: 'mullai' | 'marutham' | 'kurinji' | 'neithal' | 'paalai' | 'placeholder';
		metadata: ChapterMetadata;
		sections: Section[];
	}

	// Thinai color gradients
	const thinaiColors: Record<string, { from: string; to: string; accent: string }> = {
		mullai: { from: '#5c7a52', to: '#7a9968', accent: '#5c7a52' }, // Sage green
		marutham: { from: '#c5941a', to: '#d4a82a', accent: '#c5941a' }, // Gold
		kurinji: { from: '#3d7a7a', to: '#4a9494', accent: '#3d7a7a' }, // Teal
		neithal: { from: '#2d5a7a', to: '#3d6a8a', accent: '#2d5a7a' }, // Deep blue
		paalai: { from: '#c45c3e', to: '#d46a4e', accent: '#c45c3e' }, // Terracotta
		placeholder: { from: '#8a7a62', to: '#9a8a72', accent: '#8a7a62' } // Muted
	};

	// Mock data for 5 Tamil Sangam landscapes with metadata
	const chapters: Chapter[] = [
		{
			id: 'ch-1',
			number: 1,
			tamilName: 'முல்லை',
			englishName: 'Mullai',
			landscape: 'Forest',
			thinaiKey: 'mullai',
			metadata: {
				topic: 'காட்டுப் பூக்கள், இயற்கை',
				source: 'குறுந்தொகை',
				thinaiDescription: 'முல்லை - காடும் காடு சார்ந்த இடமும்'
			},
			sections: Array.from({ length: 10 }, (_, i) => ({
				id: `ch-1-s-${i + 1}`,
				number: i + 1,
				title: `The ${['Jasmine', 'Deer', 'Shepherd', 'Monsoon', 'Waiting', 'Reunion', 'Evening', 'Pasture', 'Thunder', 'Return'][i]}`,
				status: 'completed' as const
			}))
		},
		{
			id: 'ch-2',
			number: 2,
			tamilName: 'மருதம்',
			englishName: 'Marutham',
			landscape: 'Lowlands',
			thinaiKey: 'marutham',
			metadata: {
				topic: 'வேளாண்மை, நீர்நிலைகள்',
				source: 'புறநானூறு',
				thinaiDescription: 'மருதம் - வயலும் வயல் சார்ந்த இடமும்'
			},
			sections: Array.from({ length: 10 }, (_, i) => ({
				id: `ch-2-s-${i + 1}`,
				number: i + 1,
				title: `The ${['River', 'Paddy', 'Heron', 'Festival', 'Jealousy', 'Sulking', 'Lotus', 'Buffalo', 'Dancer', 'Reconciliation'][i]}`,
				status: 'completed' as const
			}))
		},
		{
			id: 'ch-3',
			number: 3,
			tamilName: 'குறிஞ்சி',
			englishName: 'Kurinji',
			landscape: 'Mountain',
			thinaiKey: 'kurinji',
			metadata: {
				topic: 'மலைப்பூக்கள், குறிஞ்சி மலர்',
				source: 'அகநானூறு',
				thinaiDescription: 'குறிஞ்சி - மலையும் மலை சார்ந்த இடமும்'
			},
			sections: Array.from({ length: 10 }, (_, i) => ({
				id: `ch-3-s-${i + 1}`,
				number: i + 1,
				title: `The ${['Peak', 'Waterfall', 'Millet', 'First Meeting', 'Secret Love', 'Mountain Stream', 'Honey', 'Bamboo', 'Mist', 'Elopement'][i]}`,
				status: i < 3 ? ('completed' as const) : i === 3 ? ('current' as const) : ('not_started' as const)
			}))
		},
		{
			id: 'ch-4',
			number: 4,
			tamilName: 'நெய்தல்',
			englishName: 'Neithal',
			landscape: 'Coastal',
			thinaiKey: 'neithal',
			metadata: {
				topic: 'கடல், மீன்பிடி',
				source: 'நற்றிணை',
				thinaiDescription: 'நெய்தல் - கடலும் கடல் சார்ந்த இடமும்'
			},
			sections: Array.from({ length: 10 }, (_, i) => ({
				id: `ch-4-s-${i + 1}`,
				number: i + 1,
				title: `The ${['Shore', 'Fisherman', 'Conch', 'Separation', 'Longing', 'Waves', 'Salt Wind', 'Shark', 'Boat', 'Sea Song'][i]}`,
				status: 'not_started' as const
			}))
		},
		{
			id: 'ch-5',
			number: 5,
			tamilName: 'பாலை',
			englishName: 'Paalai',
			landscape: 'Desert',
			thinaiKey: 'paalai',
			metadata: {
				topic: 'பாலை நிலம், பிரிவு',
				source: 'கலித்தொகை',
				thinaiDescription: 'பாலை - சுரமும் சுரம் சார்ந்த இடமும்'
			},
			sections: Array.from({ length: 10 }, (_, i) => ({
				id: `ch-5-s-${i + 1}`,
				number: i + 1,
				title: `The ${['Wasteland', 'Journey', 'Bandits', 'Parched', 'Endurance', 'Mirage', 'Vulture', 'Bones', 'Oasis', 'Arrival'][i]}`,
				status: 'not_started' as const
			}))
		}
	];

	// Placeholder chapters 6-14
	const placeholderChapters: Chapter[] = Array.from({ length: 9 }, (_, i) => ({
		id: `ch-${i + 6}`,
		number: i + 6,
		tamilName: '—',
		englishName: `Chapter ${i + 6}`,
		landscape: 'Coming Soon',
		thinaiKey: 'placeholder' as const,
		metadata: {
			topic: '—',
			source: '—',
			thinaiDescription: '—'
		},
		sections: []
	}));

	const allChapters = [...chapters, ...placeholderChapters];

	// Calculate totals
	const totalSections = $derived(
		chapters.reduce((sum, ch) => sum + ch.sections.length, 0) + placeholderChapters.length * 10
	);
	const completedSections = $derived(
		chapters.reduce((sum, ch) => sum + ch.sections.filter((s) => s.status === 'completed').length, 0)
	);

	// Find current chapter and section
	const currentChapter = $derived(chapters.find((ch) => ch.sections.some((s) => s.status === 'current')));
	const currentSection = $derived(currentChapter?.sections.find((s) => s.status === 'current'));
	const currentChapterProgress = $derived(
		currentChapter
			? (currentChapter.sections.filter((s) => s.status === 'completed').length / currentChapter.sections.length) * 100
			: 0
	);

	// Track which chapter is expanded (default to current chapter, ch-3)
	let expandedChapterId = $state('ch-3');

	function toggleChapter(chapterId: string) {
		if (expandedChapterId === chapterId) {
			expandedChapterId = '';
		} else {
			expandedChapterId = chapterId;
		}
	}

	function handleSectionClick(sectionId: string) {
		onselect(sectionId);
	}

	function getCompletedCount(chapter: Chapter): number {
		return chapter.sections.filter((s) => s.status === 'completed').length;
	}

	function getStatusMarker(status: Section['status']): string {
		switch (status) {
			case 'completed':
				return '✓';
			case 'current':
				return '●';
			case 'not_started':
				return '○';
		}
	}

	function getChapterGradient(thinaiKey: string): string {
		const colors = thinaiColors[thinaiKey] || thinaiColors.placeholder;
		return `linear-gradient(135deg, ${colors.from} 0%, ${colors.to} 100%)`;
	}

	function getChapterAccent(thinaiKey: string): string {
		const colors = thinaiColors[thinaiKey] || thinaiColors.placeholder;
		return colors.accent;
	}
</script>

<div class="lesson-list">
	<!-- Overall Progress Band -->
	<section class="leaf-band progress-band">
		<div class="band-content">
			<span class="label">YOUR JOURNEY</span>
			<div class="progress-main">
				<span class="progress-text">{completedSections} of {totalSections} sections completed</span>
			</div>
			<div class="progress-track">
				<div class="progress-fill" style:width="{(completedSections / totalSections) * 100}%"></div>
			</div>
		</div>
		<div class="band-line"></div>
	</section>

	<!-- Now Reading Band -->
	{#if currentChapter && currentSection}
		<section class="leaf-band now-reading-band">
			<button class="now-reading-link" onclick={() => handleSectionClick(currentSection.id)}>
				<div class="now-reading-header">
					<span class="label">NOW READING</span>
					<span class="chapter-meta">
						<span class="chapter-tamil">{currentChapter.tamilName}</span>
						<span class="chapter-english">{currentChapter.englishName}</span>
						<span class="chapter-landscape-inline">· {currentChapter.landscape}</span>
					</span>
				</div>
				<div class="now-reading-main">
					<h2 class="section-title">Section {currentSection.number}: {currentSection.title}</h2>
					<span class="arrow">→</span>
				</div>
				<div class="now-reading-progress">
					<div class="progress-track">
						<div class="progress-fill" style:width="{currentChapterProgress}%"></div>
					</div>
					<span class="progress-pct">{Math.round(currentChapterProgress)}%</span>
				</div>
			</button>
			<div class="band-line"></div>
		</section>
	{/if}

	<!-- Chapters Accordion -->
	<section class="chapters-section">
		<h2 class="chapters-heading">All Chapters</h2>

		{#each allChapters as chapter (chapter.id)}
			{@const isExpanded = expandedChapterId === chapter.id}
			{@const isPlaceholder = chapter.sections.length === 0}
			{@const completedCount = isPlaceholder ? 0 : getCompletedCount(chapter)}
			{@const isCurrent = chapter.id === currentChapter?.id}
			{@const totalCount = isPlaceholder ? 10 : chapter.sections.length}

			<div
				class="chapter-card"
				class:expanded={isExpanded}
				class:current={isCurrent}
				class:placeholder={isPlaceholder}
			>
				<!-- Large numbered circle -->
				<div class="chapter-number-circle" style:background={getChapterGradient(chapter.thinaiKey)}>
					<span class="chapter-number">{chapter.number}</span>
				</div>

				<button
					class="chapter-header"
					onclick={() => !isPlaceholder && toggleChapter(chapter.id)}
					disabled={isPlaceholder}
				>
					<div class="chapter-content">
						<div class="chapter-title-row">
							<div class="chapter-names">
								<span class="chapter-tamil">{chapter.tamilName}</span>
								<span class="chapter-english">{chapter.englishName}</span>
							</div>
							<span class="chapter-landscape">{chapter.landscape}</span>
						</div>

						{#if !isPlaceholder}
							<div class="chapter-metadata">
								<div class="metadata-row">
									<span class="metadata-label">தெரிந்துகொள்வோம்</span>
									<span class="metadata-colon">:</span>
									<span class="metadata-value">{chapter.metadata.topic}</span>
								</div>
								<div class="metadata-row">
									<span class="metadata-label">இலக்கியம்</span>
									<span class="metadata-colon">:</span>
									<span class="metadata-value">{chapter.metadata.source}</span>
								</div>
								<div class="metadata-row">
									<span class="metadata-label">தினை</span>
									<span class="metadata-colon">:</span>
									<span class="metadata-value">{chapter.metadata.thinaiDescription}</span>
								</div>
							</div>
						{/if}
					</div>

					<div class="chapter-right">
						<!-- Colored pill badge -->
						<div class="progress-pill" style:background={getChapterAccent(chapter.thinaiKey)}>
							{#if isPlaceholder}
								<span>—</span>
							{:else if completedCount === totalCount}
								<span>{totalCount}</span>
							{:else}
								<span>{completedCount}/{totalCount}</span>
							{/if}
						</div>

						{#if !isPlaceholder}
							<span class="expand-icon">{isExpanded ? '−' : '+'}</span>
						{/if}
					</div>
				</button>

				{#if isExpanded && !isPlaceholder}
					<div class="chapter-sections">
						<!-- Progress dots row -->
						<div class="sections-progress-row">
							<div class="progress-dots">
								{#each chapter.sections as section (section.id)}
									<span
										class="dot"
										class:completed={section.status === 'completed'}
										class:current={section.status === 'current'}
									></span>
								{/each}
							</div>
						</div>

						{#each chapter.sections as section (section.id)}
							<button
								class="section-row"
								class:completed={section.status === 'completed'}
								class:current={section.status === 'current'}
								class:not-started={section.status === 'not_started'}
								onclick={() => handleSectionClick(section.id)}
							>
								<span class="section-number">{section.number}</span>
								<span class="section-name">{section.title}</span>
								<span class="section-status">{getStatusMarker(section.status)}</span>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</section>
</div>

<style>
	:root {
		--font-serif: 'Cormorant Garamond', Georgia, serif;
		--font-heading: 'Eczar', Georgia, serif;

		/* Palm leaf manuscript palette */
		--c-leaf: #e8e0c8;
		--c-leaf-light: #f2edd8;
		--c-leaf-dark: #d4c9a8;
		--c-ink: #2d2418;
		--c-ink-light: #5c4d3a;
		--c-ink-faint: #8a7a62;
		--c-teal: #3d7a7a;
		--c-terracotta: #c45c3e;
		--c-sage: #5c7a52;
	}

	.lesson-list {
		min-height: 100dvh;
		background: var(--c-leaf);
		font-family: var(--font-serif);
		max-width: 600px;
		margin: 0 auto;
		padding: 20px 16px 48px;
	}

	/* Leaf Band base styles */
	.leaf-band {
		background: var(--c-leaf-light);
		border-radius: 4px;
		margin-bottom: 12px;
		position: relative;
		overflow: hidden;
	}

	.band-content {
		padding: 16px 20px;
	}

	.band-line {
		height: 3px;
		background: linear-gradient(90deg, var(--c-leaf-dark) 0%, var(--c-leaf) 50%, var(--c-leaf-dark) 100%);
	}

	.label {
		font-size: 10px;
		font-weight: 600;
		color: var(--c-ink-faint);
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	/* Progress Band */
	.progress-band .band-content {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.progress-main {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
	}

	.progress-text {
		font-family: var(--font-heading);
		font-size: 16px;
		font-weight: 600;
		color: var(--c-ink);
	}

	.progress-track {
		height: 3px;
		background: var(--c-leaf-dark);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--c-sage);
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	/* Now Reading Band */
	.now-reading-band {
		background: var(--c-leaf-light);
	}

	.now-reading-link {
		display: block;
		width: 100%;
		padding: 16px 20px;
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
		background: rgba(61, 122, 122, 0.04);
	}

	.now-reading-header {
		display: flex;
		align-items: baseline;
		gap: 12px;
		flex-wrap: wrap;
	}

	.chapter-meta {
		display: flex;
		align-items: baseline;
		gap: 6px;
	}

	.chapter-tamil {
		font-family: 'Noto Serif Tamil', serif;
		font-size: 13px;
		color: var(--c-teal);
	}

	.chapter-english {
		font-size: 13px;
		color: var(--c-ink-light);
		font-style: italic;
	}

	.chapter-landscape-inline {
		font-size: 12px;
		color: var(--c-ink-faint);
	}

	.now-reading-main {
		display: flex;
		align-items: center;
		gap: 12px;
		margin: 10px 0 14px;
	}

	.section-title {
		flex: 1;
		font-family: var(--font-heading);
		font-size: 18px;
		font-weight: 600;
		color: var(--c-ink);
		margin: 0;
		line-height: 1.3;
	}

	.arrow {
		font-size: 20px;
		color: var(--c-ink-faint);
		transition:
			color 0.2s,
			transform 0.2s;
	}

	.now-reading-link:hover .arrow {
		color: var(--c-teal);
		transform: translateX(4px);
	}

	.now-reading-progress {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.now-reading-progress .progress-track {
		flex: 1;
	}

	.progress-pct {
		font-family: var(--font-heading);
		font-size: 12px;
		font-weight: 600;
		color: var(--c-ink-faint);
		min-width: 32px;
		text-align: right;
	}

	/* Chapters Section */
	.chapters-section {
		margin-top: 24px;
	}

	.chapters-heading {
		font-family: var(--font-heading);
		font-size: 14px;
		font-weight: 600;
		color: var(--c-ink-faint);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		margin: 0 0 12px 4px;
	}

	/* Chapter Card */
	.chapter-card {
		background: var(--c-leaf-light);
		border-radius: 4px;
		margin-bottom: 12px;
		overflow: visible;
		border: 1px solid transparent;
		transition: border-color 0.2s;
		position: relative;
		margin-left: 28px;
	}

	.chapter-card.current {
		border-color: var(--c-teal);
	}

	.chapter-card.placeholder {
		opacity: 0.5;
	}

	/* Large numbered circle */
	.chapter-number-circle {
		position: absolute;
		left: -28px;
		top: 50%;
		transform: translateY(-50%);
		width: 52px;
		height: 52px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
		z-index: 1;
	}

	.chapter-card.expanded .chapter-number-circle {
		top: 40px;
		transform: translateY(0);
	}

	.chapter-number {
		font-family: var(--font-heading);
		font-size: 22px;
		font-weight: 700;
		color: white;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
	}

	.chapter-header {
		display: flex;
		align-items: flex-start;
		width: 100%;
		padding: 16px 16px 16px 32px;
		background: transparent;
		border: none;
		cursor: pointer;
		text-align: left;
		font: inherit;
		gap: 12px;
		transition: background 0.2s;
	}

	.chapter-header:hover:not(:disabled) {
		background: rgba(61, 122, 122, 0.04);
	}

	.chapter-header:disabled {
		cursor: default;
	}

	.chapter-content {
		flex: 1;
		min-width: 0;
	}

	.chapter-title-row {
		display: flex;
		flex-direction: column;
		gap: 2px;
		margin-bottom: 8px;
	}

	.chapter-names {
		display: flex;
		align-items: baseline;
		gap: 8px;
	}

	.chapter-content .chapter-tamil {
		font-family: 'Noto Serif Tamil', serif;
		font-size: 17px;
		color: var(--c-ink);
		font-weight: 600;
	}

	.chapter-content .chapter-english {
		font-family: var(--font-heading);
		font-size: 15px;
		font-weight: 500;
		color: var(--c-ink-light);
		font-style: italic;
	}

	.chapter-landscape {
		font-size: 11px;
		color: var(--c-ink-faint);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Chapter metadata rows */
	.chapter-metadata {
		display: flex;
		flex-direction: column;
		gap: 4px;
		margin-top: 8px;
	}

	.metadata-row {
		display: flex;
		align-items: baseline;
		gap: 4px;
		font-size: 12px;
		line-height: 1.5;
	}

	.metadata-label {
		font-family: 'Noto Serif Tamil', serif;
		color: var(--c-ink-faint);
		flex-shrink: 0;
	}

	.metadata-colon {
		color: var(--c-ink-faint);
	}

	.metadata-value {
		font-family: 'Noto Serif Tamil', serif;
		color: var(--c-ink-light);
	}

	/* Right side of chapter card */
	.chapter-right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 8px;
		flex-shrink: 0;
	}

	/* Colored pill badge */
	.progress-pill {
		padding: 4px 12px;
		border-radius: 12px;
		font-family: var(--font-heading);
		font-size: 12px;
		font-weight: 600;
		color: white;
		min-width: 40px;
		text-align: center;
	}

	.expand-icon {
		font-family: var(--font-heading);
		font-size: 18px;
		font-weight: 600;
		color: var(--c-ink-faint);
		width: 24px;
		text-align: center;
		transition: color 0.2s;
	}

	.chapter-header:hover .expand-icon {
		color: var(--c-teal);
	}

	/* Chapter Sections (expanded content) */
	.chapter-sections {
		border-top: 1px solid var(--c-leaf-dark);
		padding: 8px 0 8px 32px;
	}

	/* Progress dots row inside expanded section */
	.sections-progress-row {
		padding: 8px 16px 12px;
		border-bottom: 1px solid var(--c-leaf-dark);
		margin-bottom: 4px;
	}

	.progress-dots {
		display: flex;
		gap: 6px;
		flex-wrap: wrap;
	}

	.dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: var(--c-leaf-dark);
		transition: background 0.2s;
	}

	.dot.completed {
		background: var(--c-sage);
	}

	.dot.current {
		background: var(--c-terracotta);
	}

	.section-row {
		display: flex;
		align-items: center;
		width: 100%;
		padding: 10px 16px;
		background: transparent;
		border: none;
		cursor: pointer;
		text-align: left;
		font: inherit;
		gap: 12px;
		transition: background 0.2s;
	}

	.section-row:hover {
		background: rgba(61, 122, 122, 0.06);
	}

	.section-number {
		font-family: var(--font-heading);
		font-size: 13px;
		font-weight: 600;
		color: var(--c-ink-faint);
		width: 20px;
		text-align: center;
	}

	.section-name {
		flex: 1;
		font-family: var(--font-serif);
		font-size: 15px;
		color: var(--c-ink);
	}

	.section-row.completed .section-name {
		color: var(--c-ink-light);
	}

	.section-row.current .section-name {
		font-weight: 600;
		color: var(--c-ink);
	}

	.section-row.not-started .section-name {
		color: var(--c-ink-light);
	}

	.section-status {
		font-size: 14px;
		width: 20px;
		text-align: center;
	}

	.section-row.completed .section-status {
		color: var(--c-sage);
	}

	.section-row.current .section-status {
		color: var(--c-terracotta);
		font-size: 10px;
	}

	.section-row.not-started .section-status {
		color: var(--c-leaf-dark);
	}

	/* Responsive */
	@media (max-width: 480px) {
		.lesson-list {
			padding: 16px 12px 40px;
		}

		.chapter-card {
			margin-left: 24px;
		}

		.chapter-number-circle {
			width: 44px;
			height: 44px;
			left: -24px;
		}

		.chapter-number {
			font-size: 18px;
		}

		.chapter-header {
			padding: 14px 12px 14px 28px;
			flex-wrap: wrap;
		}

		.chapter-content {
			flex-basis: calc(100% - 60px);
		}

		.chapter-right {
			position: absolute;
			right: 12px;
			top: 14px;
		}

		.chapter-sections {
			padding-left: 28px;
		}

		.section-title {
			font-size: 16px;
		}

		.progress-text {
			font-size: 14px;
		}

		.metadata-row {
			font-size: 11px;
		}

		.chapter-content .chapter-tamil {
			font-size: 15px;
		}

		.chapter-content .chapter-english {
			font-size: 13px;
		}
	}
</style>
