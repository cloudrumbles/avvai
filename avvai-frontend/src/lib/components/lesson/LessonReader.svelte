<script lang="ts">
	import { ProseSection, PoetrySection, VocabularySection, MediaSection, ExercisesSection } from '$lib/components/sections';
	import type { Lesson, ContentSection } from '$lib/types/lesson';
	import TableOfContents from '$lib/components/TableOfContents.svelte';
	import Drawer from '$lib/components/Drawer.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import ReadingSettingsMenu from '$lib/components/ReadingSettingsMenu.svelte';
	import { DEFAULT_FONT } from '$lib/config/fonts';

	interface Props {
		lesson: Lesson;
		onclose: () => void;
	}

	let { lesson, onclose }: Props = $props();

	/* ── section navigation ── */
	let activeSectionIndex = $state(0);
	let sectionElements: HTMLElement[] = [];
	let readingSurface: HTMLElement | undefined = $state(undefined);
	let progressPercent = $state(0);

	function scrollToSection(index: number) {
		if (!readingSurface || !sectionElements[index]) return;
		const element = sectionElements[index];
		const offsetTop = element.offsetTop - 24;
		readingSurface.scrollTo({ top: offsetTop, behavior: 'smooth' });
	}

	function updateActiveSection() {
		if (!readingSurface) return;
		const scrollTop = readingSurface.scrollTop;
		const containerHeight = readingSurface.clientHeight;
		const scrollHeight = readingSurface.scrollHeight;

		progressPercent = Math.min(100, Math.round((scrollTop / (scrollHeight - containerHeight)) * 100));

		for (let i = sectionElements.length - 1; i >= 0; i--) {
			const element = sectionElements[i];
			if (element && element.offsetTop <= scrollTop + 100) {
				activeSectionIndex = i;
				break;
			}
		}
	}

	/* ── font settings ── */
	let fontFamily = $state(DEFAULT_FONT);
	let fontSize = $state(16);
	let lineHeight = $derived(fontSize * 1.7);

	/* ── UI state ── */
	let tocOpen = $state(false);
	let drawerOpen = $state(false);

	/* ── derived sections ── */
	let contentSections = $derived(
		lesson.sections?.filter((s: ContentSection) => s.type !== 'exercises') ?? []
	);
	let exercisesSection = $derived(
		lesson.sections?.find((s: ContentSection) => s.type === 'exercises')
	);
</script>

<div class="lesson-reader">
	{#if lesson.sections && lesson.sections.length > 0}
		<aside class="sidebar">
			<TableOfContents
				sections={lesson.sections}
				{activeSectionIndex}
				onSectionClick={scrollToSection}
				{progressPercent}
			/>
		</aside>
	{/if}

	<div class="main">
		<header class="header">
			<IconButton label="Back to lessons" onclick={onclose}>
				<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
					<path d="M12 4L6 10L12 16" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
			</IconButton>
			<span class="title">{lesson.title}</span>

			<div class="header-actions">
				<IconButton label="Table of Contents" expanded={tocOpen} onclick={() => tocOpen = !tocOpen}>
					{#snippet children()}
						<svg class="mobile-toc-icon" width="20" height="20" viewBox="0 0 20 20" fill="none">
							<path d="M3 5H17M3 10H17M3 15H17" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
						</svg>
					{/snippet}
				</IconButton>

				<ReadingSettingsMenu
					{fontFamily}
					{fontSize}
					onfontchange={(v) => fontFamily = v}
					onsizechange={(v) => fontSize = v}
				/>
			</div>
		</header>

		{#if lesson.sections && lesson.sections.length > 0}
			<div
				class="reading-surface"
				class:drawer-open={drawerOpen}
				style="font-family: {fontFamily}; font-size: {fontSize}px; line-height: {lineHeight}px;"
				bind:this={readingSurface}
				onscroll={updateActiveSection}
			>
				{#each contentSections as section, i (i)}
					<div class="section-wrapper" bind:this={sectionElements[i]}>
						{#if section.type === 'prose'}
							<ProseSection data={section} />
						{:else if section.type === 'poetry'}
							<PoetrySection data={section} />
						{:else if section.type === 'vocabulary'}
							<VocabularySection data={section} />
						{:else if section.type === 'media'}
							<MediaSection data={section} />
						{/if}
					</div>
				{/each}
			</div>

			<!-- Mobile TOC Drawer -->
			<Drawer open={tocOpen} title="Contents" onclose={() => tocOpen = false} position="left">
				<TableOfContents
					sections={lesson.sections}
					{activeSectionIndex}
					onSectionClick={(index) => {
						scrollToSection(index);
						tocOpen = false;
					}}
					{progressPercent}
				/>
			</Drawer>

			<!-- Exercises Drawer -->
			{#if exercisesSection}
				<button class="drawer-tab" class:open={drawerOpen} onclick={() => drawerOpen = !drawerOpen}>
					<span class="drawer-tab-text">{exercisesSection.title || 'பயிற்சிகள்'}</span>
					<svg class="drawer-tab-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
						<path d="M4 6L8 10L12 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
					</svg>
				</button>

				<Drawer open={drawerOpen} title={exercisesSection.title || 'பயிற்சிகள்'} onclose={() => drawerOpen = false} position="bottom">
					<ExercisesSection data={exercisesSection} />
				</Drawer>
			{/if}
		{:else}
			<div class="reading-surface">
				<p class="empty-lesson">This lesson has no content yet.</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.lesson-reader {
		display: flex;
		flex-direction: row;
		height: 100dvh;
		background: var(--cream);
		overflow: hidden;
	}

	.sidebar {
		width: 280px;
		flex-shrink: 0;
		height: 100%;
		overflow: hidden;
	}

	@media (max-width: 768px) {
		.sidebar {
			display: none;
		}
	}

	.main {
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

	.header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-bottom: 1px solid var(--cream-mid);
		background: var(--cream);
	}

	.title {
		flex: 1;
		font-family: 'Catamaran', sans-serif;
		font-size: 15px;
		font-weight: 600;
		color: var(--red-deep);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

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

	.reading-surface.drawer-open {
		padding-bottom: 80px;
	}

	.empty-lesson {
		font-family: 'Catamaran', sans-serif;
		color: var(--stone);
		text-align: center;
		padding: 48px 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	/* Hide mobile TOC button on desktop */
	:global(.header-actions > button:first-child) {
		display: flex;
	}

	@media (min-width: 769px) {
		:global(.header-actions > button:first-child) {
			display: none;
		}
	}

	.drawer-tab {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px 24px;
		background: var(--red);
		color: var(--cream);
		border: none;
		border-radius: 0;
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		z-index: 100;
		transition: all 0.2s ease;
		box-shadow: 0 -2px 12px rgba(139, 26, 26, 0.2);
	}

	.drawer-tab:hover {
		background: var(--red-deep);
	}

	.drawer-tab.open {
		opacity: 0;
		pointer-events: none;
	}

	.drawer-tab-icon {
		transition: transform 0.2s ease;
	}

	.drawer-tab.open .drawer-tab-icon {
		transform: rotate(180deg);
	}

	@media (min-width: 769px) {
		.drawer-tab {
			left: auto;
			right: 48px;
			bottom: 24px;
			border-radius: 30px;
			padding: 12px 24px;
		}
	}
</style>
