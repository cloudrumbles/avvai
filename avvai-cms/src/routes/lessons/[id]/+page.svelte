<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/stores';
	import { fade, scale } from 'svelte/transition';
	import { elasticOut } from 'svelte/easing';
	import 'avvai-frontend/styles/tokens';

	import { EditorMiddlePanel, EditorPreviewPanel } from '$lib/components/editor/panels';
	import { useLayoutContext } from '$lib/layoutContext';
	import LessonSidebarContent from '$lib/components/layout/LessonSidebarContent.svelte';

	import type { Lesson, ContentSection } from '$lib/types/lesson';

	let lessonId = $derived($page.params.id);

	let lesson = $state<Lesson | null>(null);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let isDirty = $state(false);
	let activeSectionIndex = $state(0);
	let sectionRefs = $state<HTMLElement[]>([]);
	let middlePanelContainer: HTMLElement | undefined = $state();
	const { setSidebarOverride, clearSidebarOverride } = useLayoutContext();

	onMount(async () => {
		try {
			const res = await fetch(`/api/admin/lessons/${lessonId}`);
			if (!res.ok) throw new Error('Lesson not found');
			lesson = await res.json();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load lesson';
		} finally {
			loading = false;
		}
	});

	$effect(() => {
		if (!lesson) return;

		setSidebarOverride({
			structureTitle: 'Structure',
			structureCount: lesson.sections.length,
			structureComponent: LessonSidebarContent,
			structureProps: {
				lesson,
				activeSectionIndex,
				onSectionClick: scrollToSection,
				onAddSection: addSection,
				onMoveSection: moveSection,
				onDeleteSection: removeSection
			},
			fullWidth: true
		});
	});

	onDestroy(() => {
		clearSidebarOverride();
	});

	// Set up IntersectionObserver for scroll-spy
	$effect(() => {
		if (!lesson || sectionRefs.length === 0) return;

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const index = parseInt(entry.target.getAttribute('data-index') ?? '0');
						activeSectionIndex = index;
					}
				});
			},
			{
				root: middlePanelContainer,
				rootMargin: '-20% 0px -60% 0px',
				threshold: 0
			}
		);

		sectionRefs.forEach((el) => {
			if (el) observer.observe(el);
		});

		return () => observer.disconnect();
	});

	function createEmptySection(type: string): ContentSection {
		switch (type) {
			case 'prose':
				return { type: 'prose', paragraphs: [''] };
			case 'poetry':
				return { type: 'poetry', verses: [{ lines: [''] }] };
			case 'vocabulary':
				return { type: 'vocabulary', entries: [{ word: '', meaning: '' }] };
			case 'exercises':
				return { type: 'exercises', exercise_groups: [] };
			case 'media':
				return { type: 'media', media_type: 'image', url: '' };
			case 'dialogue':
				return { type: 'dialogue', lines: [{ text: '' }] };
			default:
				return { type: 'prose', paragraphs: [''] };
		}
	}

	function addSection(type: string) {
		if (!lesson) return;
		lesson.sections = [...lesson.sections, createEmptySection(type)];
		isDirty = true;

		// Scroll to new section after DOM updates
		setTimeout(() => {
			const newIndex = lesson!.sections.length - 1;
			scrollToSection(newIndex);
		}, 100);
	}

	function removeSection(index: number) {
		if (!lesson) return;
		const confirm = window.confirm('Are you sure you want to delete this section?');
		if (confirm) {
			lesson.sections = lesson.sections.filter((_, i) => i !== index);
			isDirty = true;
		}
	}

	function moveSection(index: number, direction: -1 | 1) {
		if (!lesson) return;
		const newIndex = index + direction;
		if (newIndex < 0 || newIndex >= lesson.sections.length) return;

		const sections = [...lesson.sections];
		[sections[index], sections[newIndex]] = [sections[newIndex], sections[index]];
		lesson.sections = sections;
		isDirty = true;
	}

	function scrollToSection(index: number) {
		const element = document.getElementById(`section-${index}`);
		if (element) {
			element.scrollIntoView({ behavior: 'smooth', block: 'start' });
		}
		activeSectionIndex = index;
	}

	async function saveLesson() {
		if (!lesson) return;

		saving = true;
		error = '';

		try {
			const res = await fetch(`/api/admin/lessons/${lessonId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ lesson })
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Failed to save');
			}

			isDirty = false;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			saving = false;
		}
	}

	function handleLessonChange() {
		isDirty = true;
	}

	// Track deep changes to sections
	let lastSectionsJson = $state('');

	$effect(() => {
		if (!lesson) return;
		const currentJson = JSON.stringify(lesson.sections);
		if (lastSectionsJson && currentJson !== lastSectionsJson) {
			isDirty = true;
		}
		lastSectionsJson = currentJson;
	});
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,400;0,500;0,600;0,700;1,400&family=Source+Sans+3:wght@400;500;600;700&display=swap"
		rel="stylesheet"
	/>
	<link
		href="https://fonts.googleapis.com/css2?family=Catamaran:wght@400;500;600;700&family=Tiro+Tamil:ital@0;1&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="editor-page">
	{#if loading}
		<div class="loading-state" in:fade>
			<div class="loader">
				<div class="loader-ring"></div>
				<div class="loader-ring"></div>
				<div class="loader-ring"></div>
			</div>
			<p class="loading-text">Loading lesson...</p>
		</div>
	{:else if error && !lesson}
		<div class="error-container" in:scale={{ duration: 300, easing: elasticOut }}>
			<div class="error-card">
				<div class="error-icon">
					<svg width="48" height="48" viewBox="0 0 48 48" fill="none">
						<circle cx="24" cy="24" r="20" stroke="#B8533D" stroke-width="2" fill="#FEF3F0" />
						<path
							d="M24 16v10M24 30v2"
							stroke="#B8533D"
							stroke-width="2.5"
							stroke-linecap="round"
						/>
					</svg>
				</div>
				<h2 class="error-title">Unable to Load Lesson</h2>
				<p class="error-text">{error}</p>
				<button class="btn-primary" onclick={() => (window.location.href = '/lessons')}>
					Return to Lessons
				</button>
			</div>
		</div>
	{:else if lesson}
		<div class="editor-layout">
			<div class="middle-wrapper" bind:this={middlePanelContainer}>
				<EditorMiddlePanel
					bind:lesson
					{isDirty}
					onLessonChange={handleLessonChange}
					onMoveSection={moveSection}
					onDeleteSection={removeSection}
					onAddSection={addSection}
					bind:sectionRefs
				/>
			</div>

			<EditorPreviewPanel {lesson} {isDirty} {saving} onSave={saveLesson} />
		</div>
	{/if}
</div>

<style>
	.editor-page {
		height: 100vh;
		overflow: hidden;
		background: var(--c-cream);
	}

	.editor-layout {
		display: grid;
		grid-template-columns: 1fr minmax(380px, 480px);
		height: 100vh;
		overflow: hidden;
	}

	.middle-wrapper {
		height: 100vh;
		overflow: hidden;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100vh;
		gap: 1.5rem;
	}

	.loader {
		position: relative;
		width: 48px;
		height: 48px;
	}

	.loader-ring {
		position: absolute;
		inset: 0;
		border: 2px solid transparent;
		border-top-color: var(--c-terracotta);
		border-radius: 50%;
		animation: spin 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
	}

	.loader-ring:nth-child(2) {
		inset: 6px;
		animation-delay: -0.15s;
		border-top-color: var(--c-gold);
	}

	.loader-ring:nth-child(3) {
		inset: 12px;
		animation-delay: -0.3s;
		border-top-color: var(--c-sage);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-text {
		font-family: var(--font-display);
		font-size: 1.1rem;
		color: var(--c-brown-light);
		font-style: italic;
	}

	/* Error State */
	.error-container {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
		padding: 2rem;
	}

	.error-card {
		background: white;
		border-radius: var(--radius-lg);
		padding: 3rem;
		text-align: center;
		max-width: 400px;
		box-shadow: var(--shadow-lg);
	}

	.error-icon {
		margin-bottom: 1.5rem;
	}

	.error-title {
		font-family: var(--font-display);
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0 0 0.75rem;
	}

	.error-text {
		color: var(--c-brown-light);
		margin: 0 0 2rem;
		line-height: 1.5;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		font-family: var(--font-body);
		font-size: 0.875rem;
		font-weight: 600;
		padding: 0.6rem 1.25rem;
		border-radius: var(--radius-sm);
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
		background: var(--c-terracotta);
		color: white;
	}

	.btn-primary:hover {
		background: var(--c-terracotta-dark);
	}
</style>
