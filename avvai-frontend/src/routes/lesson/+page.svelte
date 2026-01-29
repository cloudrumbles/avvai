<script lang="ts">
	import { ProseSection, PoetrySection, VocabularySection, MediaSection, ExercisesSection } from '$lib/components/sections';
	import type { Lesson, ContentSection, LessonSummary } from '$lib/types/lesson';
	import DictionaryPopup from '$lib/components/DictionaryPopup.svelte';
	import TableOfContents from '$lib/components/TableOfContents.svelte';

	interface Props {
		data: {
			lessons: LessonSummary[];
			progress: Record<string, boolean>;
		};
	}

	let { data }: Props = $props();

	let activeLesson: Lesson | null = $state(null);
	let loading = $state(false);

	/* ── section navigation ── */
	let activeSectionIndex = $state(0);
	let sectionElements: HTMLElement[] = [];
	let readingSurface: HTMLElement | undefined = $state(undefined);
	let progressPercent = $state(0);

	function scrollToSection(index: number) {
		if (!readingSurface || !sectionElements[index]) return;
		const container = readingSurface;
		const element = sectionElements[index];
		const offsetTop = element.offsetTop - 24;
		container.scrollTo({ top: offsetTop, behavior: 'smooth' });
	}

	function updateActiveSection() {
		if (!readingSurface) return;
		const scrollTop = readingSurface.scrollTop;
		const containerHeight = readingSurface.clientHeight;
		const scrollHeight = readingSurface.scrollHeight;

		// Update progress percentage
		progressPercent = Math.min(100, Math.round((scrollTop / (scrollHeight - containerHeight)) * 100));

		// Find active section based on scroll position
		for (let i = sectionElements.length - 1; i >= 0; i--) {
			const element = sectionElements[i];
			if (element && element.offsetTop <= scrollTop + 100) {
				activeSectionIndex = i;
				break;
			}
		}
	}

	/* ── font settings ── */
	const FONTS = [
		{ label: 'Mukta Malar', value: "'Mukta Malar', sans-serif" },
		{ label: 'Catamaran', value: "'Catamaran', sans-serif" },
		{ label: 'Noto Sans Tamil', value: "'Noto Sans Tamil', sans-serif" },
		{ label: 'Noto Serif Tamil', value: "'Noto Serif Tamil', serif" },
		{ label: 'Tiro Tamil', value: "'Tiro Tamil', serif" },
		{ label: 'Hind Madurai', value: "'Hind Madurai', sans-serif" },
		{ label: 'Pavanam', value: "'Pavanam', sans-serif" },
		{ label: 'Baloo Thambi 2', value: "'Baloo Thambi 2', cursive" }
	];

	let fontFamily = $state(FONTS[0].value);
	let fontSize = $state(18);
	let lineHeight = $derived(fontSize * 1.7);

	const MIN_FONT = 16;
	const MAX_FONT = 32;
	const STEP = 2;

	function increaseFontSize() {
		if (fontSize < MAX_FONT) fontSize += STEP;
	}

	function decreaseFontSize() {
		if (fontSize > MIN_FONT) fontSize -= STEP;
	}

	/* ── settings menu ── */
	let menuOpen = $state(false);

	function toggleMenu() {
		menuOpen = !menuOpen;
	}

	function closeMenu() {
		menuOpen = false;
	}

	/* ── dictionary popup state ── */
	let popupWord = $state('');
	let popupAnchor = $state({ x: 0, y: 0, bottom: 0 });
	let popupVisible = $state(false);

	function handleWordClick(word: string, event: MouseEvent) {
		const rect = (event.target as HTMLElement).getBoundingClientRect();
		popupAnchor = { x: rect.left + rect.width / 2, y: rect.top, bottom: rect.bottom };
		popupWord = word;
		popupVisible = true;
	}

	function closePopup() {
		popupVisible = false;
		popupWord = '';
	}

	async function openLesson(id: string) {
		loading = true;
		const res = await fetch(`/api/lesson/${id}`);
		if (res.ok) {
			activeLesson = await res.json();
		}
		loading = false;
	}

	function closeLesson() {
		activeLesson = null;
	}

	async function markComplete(id: string) {
		await fetch('/api/progress', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ lessonId: id, completed: true })
		});
		data.progress[id] = true;
	}
</script>

{#if activeLesson}
	<div class="lesson-view">
		{#if activeLesson.sections && activeLesson.sections.length > 0}
			<aside class="lesson-sidebar">
				<TableOfContents
					sections={activeLesson.sections}
					{activeSectionIndex}
					onSectionClick={scrollToSection}
					{progressPercent}
				/>
			</aside>
		{/if}
		<div class="lesson-content">
			<header class="lesson-header">
			<button class="back-btn" onclick={closeLesson} aria-label="Back to lessons">
				<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
					<path d="M12 4L6 10L12 16" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
			</button>
			<span class="lesson-title">{activeLesson.title}</span>

			<div class="header-actions">
				<div class="menu-container">
					<button class="menu-btn" onclick={toggleMenu} aria-label="Reading settings" aria-expanded={menuOpen}>
						<svg width="20" height="20" viewBox="0 0 20 20" fill="none" aria-hidden="true">
							<circle cx="10" cy="4" r="1.5" fill="currentColor"/>
							<circle cx="10" cy="10" r="1.5" fill="currentColor"/>
							<circle cx="10" cy="16" r="1.5" fill="currentColor"/>
						</svg>
					</button>

					{#if menuOpen}
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<!-- svelte-ignore a11y_click_events_have_key_events -->
						<div class="menu-backdrop" onclick={closeMenu}></div>
						<div class="menu-dropdown" role="menu">
							<label class="menu-label" for="font-select">Font</label>
							<select id="font-select" class="font-select" bind:value={fontFamily}>
								{#each FONTS as font (font.value)}
									<option value={font.value}>{font.label}</option>
								{/each}
							</select>

							<label class="menu-label">Size</label>
							<div class="size-controls">
								<button
									class="ctrl-btn"
									onclick={decreaseFontSize}
									disabled={fontSize <= MIN_FONT}
									aria-label="Decrease font size"
								>
									<svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
										<line x1="2" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
									</svg>
								</button>

								<span class="size-label" aria-live="polite">{fontSize}</span>

								<button
									class="ctrl-btn"
									onclick={increaseFontSize}
									disabled={fontSize >= MAX_FONT}
									aria-label="Increase font size"
								>
									<svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
										<line x1="2" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
										<line x1="7" y1="2" x2="7" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
									</svg>
								</button>
							</div>
						</div>
					{/if}
				</div>

				<button class="complete-btn" onclick={() => markComplete(activeLesson!.id)}>
					{data.progress[activeLesson.id] ? '✓ Done' : 'Mark done'}
				</button>
			</div>
		</header>

		{#if activeLesson.sections && activeLesson.sections.length > 0}
				<div 
					class="reading-surface" 
					style="font-family: {fontFamily}; font-size: {fontSize}px; line-height: {lineHeight}px;"
					bind:this={readingSurface}
					onscroll={updateActiveSection}
				>
					{#each activeLesson.sections as section, i (i)}
						<div class="section-wrapper" bind:this={sectionElements[i]}>
							{#if section.type === 'prose'}
								<ProseSection data={section} onwordclick={handleWordClick} />
							{:else if section.type === 'poetry'}
								<PoetrySection data={section} onwordclick={handleWordClick} />
							{:else if section.type === 'vocabulary'}
								<VocabularySection data={section} onwordclick={handleWordClick} />
							{:else if section.type === 'media'}
								<MediaSection data={section} />
							{:else if section.type === 'exercises'}
								<ExercisesSection data={section} />
							{/if}
						</div>
					{/each}
				</div>
				<DictionaryPopup word={popupWord} anchor={popupAnchor} visible={popupVisible} onclose={closePopup} />
			{:else}
				<div class="reading-surface">
					<p class="empty-lesson">This lesson has no content yet.</p>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="lesson-list">
		<header class="list-header">
			<h1>Lessons</h1>
		</header>

		{#if loading}
			<p class="loading">Loading...</p>
		{:else}
			<ul class="lessons">
				{#each data.lessons as lesson (lesson.id)}
					<li>
						<button class="lesson-card" onclick={() => openLesson(lesson.id)}>
							<div class="lesson-info">
								<h2 class="lesson-card-title">{lesson.title}</h2>
								<p class="lesson-card-desc">{lesson.description}</p>
							</div>
							{#if data.progress[lesson.id]}
								<span class="completed-badge">✓</span>
							{/if}
						</button>
					</li>
				{/each}
			</ul>
		{/if}
	</div>
{/if}

<style>
	/* ========================================
	   TOKENS
	   ======================================== */
	:global(body) {
		--ink: #1a0e06;
		--ink-soft: #3a2a1a;
		--cream: #faf3e6;
		--cream-mid: #f0e4cc;
		--red: #8b1a1a;
		--red-deep: #5c0e0e;
		--red-faint: rgba(139, 26, 26, 0.08);
		--gold: #c5941a;
		--stone: #6b5a48;
	}

	/* ========================================
	   LESSON LIST VIEW
	   ======================================== */
	.lesson-list {
		max-width: 640px;
		margin: 0 auto;
		padding: 24px 16px;
		min-height: 100dvh;
		background: var(--cream);
	}

	.list-header h1 {
		font-family: 'Catamaran', sans-serif;
		font-size: 24px;
		font-weight: 700;
		color: var(--red-deep);
		margin: 0 0 24px;
	}

	.lessons {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.lesson-card {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding: 16px;
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		cursor: pointer;
		text-align: left;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.lesson-card:hover {
		border-color: var(--red);
		background: var(--red-faint);
	}

	.lesson-info {
		flex: 1;
		min-width: 0;
	}

	.lesson-card-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 16px;
		font-weight: 600;
		color: var(--ink);
		margin: 0 0 4px;
	}

	.lesson-card-desc {
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		color: var(--ink-soft);
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.completed-badge {
		font-size: 18px;
		color: var(--gold);
	}

	.loading {
		font-family: 'Catamaran', sans-serif;
		color: var(--stone);
		text-align: center;
		padding: 48px 0;
	}

	/* ========================================
	   LESSON VIEW (Reader wrapper)
	   ======================================== */
	.lesson-view {
		display: flex;
		flex-direction: row;
		height: 100dvh;
		background: var(--cream);
		overflow: hidden;
	}

	.lesson-sidebar {
		width: 280px;
		flex-shrink: 0;
		height: 100%;
		overflow: hidden;
	}

	@media (max-width: 768px) {
		.lesson-sidebar {
			display: none;
		}
	}

	.lesson-content {
		display: flex;
		flex-direction: column;
		flex: 1;
		height: 100%;
		min-width: 0;
	}

	.section-wrapper {
		margin-bottom: 32px;
	}

	.section-wrapper:last-child {
		margin-bottom: 0;
	}

	.lesson-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-bottom: 1px solid var(--cream-mid);
		background: var(--cream);
	}

	.back-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: none;
		border-radius: 8px;
		background: transparent;
		color: var(--stone);
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.back-btn:hover {
		background: var(--red-faint);
		color: var(--red);
	}

	.lesson-title {
		flex: 1;
		font-family: 'Catamaran', sans-serif;
		font-size: 15px;
		font-weight: 600;
		color: var(--red-deep);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.complete-btn {
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 600;
		color: var(--gold);
		background: transparent;
		border: 1.5px solid var(--cream-mid);
		border-radius: 8px;
		padding: 6px 12px;
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.complete-btn:hover {
		border-color: var(--gold);
		background: rgba(197, 148, 26, 0.1);
	}

	/* ========================================
	   READING SURFACE (for sections)
	   ======================================== */
	.reading-surface {
		flex: 1;
		padding: 24px 24px 32px;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		overscroll-behavior: contain;
		color: var(--ink);
		text-align: left;
		scrollbar-width: none;
	}

	.reading-surface::-webkit-scrollbar {
		display: none;
	}

	.empty-lesson {
		font-family: 'Catamaran', sans-serif;
		color: var(--stone);
		text-align: center;
		padding: 48px 0;
	}

	/* ========================================
	   HEADER ACTIONS
	   ======================================== */
	.header-actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	/* ========================================
	   SETTINGS MENU
	   ======================================== */
	.menu-container {
		position: relative;
	}

	.menu-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: none;
		border-radius: 8px;
		background: transparent;
		color: var(--stone);
		cursor: pointer;
		transition: background 0.15s ease, color 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.menu-btn:hover {
		background: var(--red-faint);
		color: var(--red);
	}

	.menu-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
	}

	.menu-dropdown {
		position: absolute;
		top: calc(100% + 8px);
		right: 0;
		z-index: 51;
		min-width: 200px;
		padding: 12px;
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		box-shadow: 0 8px 32px rgba(26, 14, 6, 0.12);
	}

	.menu-label {
		display: block;
		font-family: 'Catamaran', sans-serif;
		font-size: 11px;
		font-weight: 600;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 6px;
	}

	.menu-label:not(:first-child) {
		margin-top: 12px;
	}

	.font-select {
		width: 100%;
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		font-weight: 600;
		color: var(--red);
		background: transparent;
		border: 1.5px solid var(--cream-mid);
		border-radius: 8px;
		padding: 0 10px;
		height: 36px;
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
		outline: none;
	}

	.font-select:hover {
		background: var(--red-faint);
		border-color: var(--red);
	}

	.font-select:focus-visible {
		border-color: var(--red);
	}

	.size-controls {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.ctrl-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: 8px;
		border: 1.5px solid var(--cream-mid);
		background: transparent;
		color: var(--red);
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.ctrl-btn:hover:not(:disabled) {
		background: var(--red-faint);
		border-color: var(--red);
	}

	.ctrl-btn:active:not(:disabled) {
		background: rgba(139, 26, 26, 0.14);
		transform: scale(0.96);
	}

	.ctrl-btn:disabled {
		opacity: 0.2;
		cursor: not-allowed;
	}

	.size-label {
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		font-weight: 600;
		color: var(--ink);
		min-width: 28px;
		text-align: center;
		font-variant-numeric: tabular-nums;
	}
</style>
