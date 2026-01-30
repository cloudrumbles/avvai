<script lang="ts">
	import { onMount } from 'svelte';
	import { 
		PlusIcon, 
		TrashIcon, 
		SearchIcon,
		FilterIcon,
		MoreVerticalIcon,
		EditIcon,
		ClockIcon,
		CheckCircleIcon,
		ProseIcon,
		PoetryIcon,
		DialogueIcon,
		MediaIcon,
		VocabularyIcon
	} from '$lib/components/icons';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import type { LessonSummary } from '$lib/types/lesson';

	let lessons = $state<LessonSummary[]>([]);
	let loading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let filterType = $state('all');

	onMount(async () => {
		try {
			const res = await fetch('/api/admin/lessons');
			if (!res.ok) throw new Error('Failed to fetch lessons');
			lessons = await res.json();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	});

	async function deleteLesson(id: string) {
		if (!confirm(`Delete lesson "${id}"?`)) return;
		try {
			const res = await fetch(`/api/admin/lessons/${id}`, { method: 'DELETE' });
			if (!res.ok) throw new Error('Failed to delete');
			lessons = lessons.filter((l) => l.id !== id);
		} catch (e) {
			alert('Delete failed');
		}
	}

	const typeOptions = [
		{ value: 'all', label: 'All Types' },
		{ value: 'prose', label: 'Prose' },
		{ value: 'poetry', label: 'Poetry' },
		{ value: 'dialogue', label: 'Dialogue' },
		{ value: 'mixed', label: 'Mixed' },
	];

	const typeConfig: Record<string, { icon: any; label: string; color: string; bg: string }> = {
		prose: { icon: ProseIcon, label: 'Prose', color: 'var(--c-prose)', bg: 'var(--c-prose-bg)' },
		poetry: { icon: PoetryIcon, label: 'Poetry', color: 'var(--c-poetry)', bg: 'var(--c-poetry-bg)' },
		dialogue: { icon: DialogueIcon, label: 'Dialogue', color: 'var(--c-dialogue)', bg: 'var(--c-dialogue-bg)' },
		vocabulary: { icon: VocabularyIcon, label: 'Vocabulary', color: 'var(--c-vocabulary)', bg: 'var(--c-vocabulary-bg)' },
		mixed: { icon: MediaIcon, label: 'Mixed', color: 'var(--c-media)', bg: 'var(--c-media-bg)' },
	};

	function getLessonType(lesson: LessonSummary): string {
		// In a real app, you'd determine this from the lesson data
		// For now, we'll distribute them somewhat deterministically
		const types = ['prose', 'poetry', 'dialogue', 'mixed'];
		const index = lesson.id.charCodeAt(0) % types.length;
		return types[index];
	}

	function getLessonStats(lesson: LessonSummary): { sections: number; exercises: number } {
		// Mock stats - in real app, these would come from the lesson data
		const base = lesson.id.charCodeAt(0);
		return {
			sections: (base % 8) + 2,
			exercises: (base % 12) + 3
		};
	}

	const filteredLessons = $derived(
		lessons.filter(lesson => {
			const matchesSearch = lesson.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				lesson.id.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesType = filterType === 'all' || getLessonType(lesson) === filterType;
			return matchesSearch && matchesType;
		})
	);
</script>

<div class="lessons-page">
	<div class="page-header">
		<div class="header-content">
			<h1 class="page-title">All Lessons</h1>
			<p class="page-description">{filteredLessons.length} lessons in your collection</p>
		</div>
		<Button size="lg" onclick={() => (window.location.href = '/lessons/new')}>
			<PlusIcon size={18} />
			<span>Create Lesson</span>
		</Button>
	</div>

	<div class="filters-bar">
		<div class="search-box">
			<SearchIcon size={18} color="var(--c-brown-muted)" />
			<input 
				type="text" 
				placeholder="Search lessons..." 
				bind:value={searchQuery}
			/>
		</div>
		<div class="filter-select">
			<FilterIcon size={16} color="var(--c-brown-muted)" />
			<select bind:value={filterType}>
				{#each typeOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
		</div>
	</div>

	{#if loading}
		<div class="loading-state">
			<div class="loader">
				<div class="loader-ring"></div>
				<div class="loader-ring"></div>
				<div class="loader-ring"></div>
			</div>
			<p class="loading-text">Gathering lessons...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<div class="error-icon">
				<svg width="48" height="48" viewBox="0 0 48 48" fill="none">
					<circle cx="24" cy="24" r="20" stroke="var(--c-terracotta)" stroke-width="2" fill="var(--c-error-bg)"/>
					<path d="M24 16v10M24 30v2" stroke="var(--c-terracotta)" stroke-width="2.5" stroke-linecap="round"/>
				</svg>
			</div>
			<h2 class="error-title">Unable to Load</h2>
			<p class="error-text">{error}</p>
			<Button variant="outline" onclick={() => window.location.reload()}>Try Again</Button>
		</div>
	{:else if filteredLessons.length === 0}
		<div class="empty-state">
			{#if searchQuery || filterType !== 'all'}
				<div class="empty-content">
					<SearchIcon size={48} color="var(--c-brown-muted)" />
					<h2 class="empty-title">No matches found</h2>
					<p class="empty-text">Try adjusting your search or filters</p>
					<Button variant="outline" onclick={() => { searchQuery = ''; filterType = 'all'; }}>
						Clear Filters
					</Button>
				</div>
			{:else}
				<div class="empty-content">
					<div class="empty-illustration">
						<ProseIcon size={64} color="var(--c-terracotta)" />
					</div>
					<h2 class="empty-title">Begin Your Collection</h2>
					<p class="empty-text">No lessons yet. Create your first lesson to start sharing Tamil literature.</p>
					<Button onclick={() => (window.location.href = '/lessons/new')}>
						<PlusIcon size={16} />
						<span>Create First Lesson</span>
					</Button>
				</div>
			{/if}
		</div>
	{:else}
		<div class="lessons-grid">
			{#each filteredLessons as lesson (lesson.id)}
				{@const type = getLessonType(lesson)}
				{@const typeInfo = typeConfig[type] || typeConfig.mixed}
				{@const stats = getLessonStats(lesson)}
				<article class="lesson-card">
					<div class="card-type-badge" style="background: {typeInfo.bg}; color: {typeInfo.color}">
						<typeInfo.icon size={14} />
						<span>{typeInfo.label}</span>
					</div>
					
					<div class="card-content">
						<h2 class="lesson-title">
							<a href="/lessons/{lesson.id}">{lesson.title || lesson.id}</a>
						</h2>
						
						{#if lesson.description}
							<p class="lesson-description">{lesson.description}</p>
						{/if}
						
						<div class="lesson-meta">
							<span class="meta-item">
								{typeInfo.label === 'Prose' ? '📄' : typeInfo.label === 'Poetry' ? '🌸' : typeInfo.label === 'Dialogue' ? '🎭' : '📚'}
								{stats.sections} sections
							</span>
							<span class="meta-divider">•</span>
							<span class="meta-item">
								✏️ {stats.exercises} exercises
							</span>
						</div>
						
						<div class="card-footer">
							<div class="lesson-status">
								<CheckCircleIcon size={14} color="var(--c-sage)" />
								<span>Published</span>
							</div>
							<div class="card-actions">
								<button 
									class="action-btn edit"
									onclick={() => (window.location.href = `/lessons/${lesson.id}`)}
									title="Edit"
								>
									<EditIcon size={16} color="var(--c-brown-light)" />
								</button>
								<button 
									class="action-btn delete"
									onclick={() => deleteLesson(lesson.id)}
									title="Delete"
								>
									<TrashIcon size={16} color="var(--c-terracotta)" />
								</button>
							</div>
						</div>
						
						<div class="last-edited">
							<ClockIcon size={12} color="var(--c-brown-muted)" />
							<span>Edited 2 hours ago</span>
						</div>
					</div>
				</article>
			{/each}
		</div>
	{/if}
</div>

<style>
	.lessons-page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: var(--space-4);
	}

	.header-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.page-description {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-muted);
		margin: 0;
	}

	.filters-bar {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex: 1;
		max-width: 400px;
		padding: 0.625rem 1rem;
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.12);
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
	}

	.search-box:focus-within {
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(184, 83, 61, 0.08);
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.95rem;
		color: var(--c-brown);
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--c-brown-muted);
	}

	.filter-select {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.12);
		border-radius: var(--radius-md);
	}

	.filter-select select {
		border: none;
		background: transparent;
		font-size: 0.95rem;
		color: var(--c-brown);
		outline: none;
		cursor: pointer;
	}

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 0;
		text-align: center;
		gap: 1rem;
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
		to { transform: rotate(360deg); }
	}

	.loading-text {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-light);
		font-style: italic;
	}

	.error-title,
	.empty-title {
		font-family: var(--font-heading);
		font-size: 1.35rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.error-text,
	.empty-text {
		font-family: var(--font-serif);
		color: var(--c-brown-muted);
		margin: 0 0 0.5rem;
		max-width: 400px;
	}

	.empty-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.empty-illustration {
		width: 96px;
		height: 96px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--c-exercises-bg);
		border-radius: var(--radius-lg);
		border: 1px solid rgba(184, 83, 61, 0.15);
	}

	.lessons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.25rem;
	}

	.lesson-card {
		position: relative;
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.08);
		border-radius: var(--radius-lg);
		overflow: hidden;
		transition: all var(--transition-base);
		box-shadow: var(--shadow-sm);
	}

	.lesson-card:hover {
		box-shadow: var(--shadow-md);
		transform: translateY(-2px);
		border-color: rgba(92, 74, 61, 0.12);
	}

	.card-type-badge {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.4rem 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.card-content {
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.lesson-title {
		font-family: var(--font-heading);
		font-size: 1.15rem;
		font-weight: 600;
		margin: 0;
		line-height: 1.4;
	}

	.lesson-title a {
		color: var(--c-brown-dark);
		transition: color var(--transition-fast);
	}

	.lesson-title a:hover {
		color: var(--c-terracotta);
	}

	.lesson-description {
		font-family: var(--font-serif);
		font-size: 0.9rem;
		color: var(--c-brown-light);
		margin: 0;
		display: -webkit-box;
		line-clamp: 2;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.lesson-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		color: var(--c-brown-muted);
		margin-top: 0.25rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.meta-divider {
		opacity: 0.5;
	}

	.card-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.5rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(92, 74, 61, 0.06);
	}

	.lesson-status {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.8rem;
		color: var(--c-sage);
		font-weight: 500;
	}

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: var(--radius-sm);
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.action-btn:hover {
		background: var(--c-cream-dark);
	}

	.action-btn.edit:hover {
		background: var(--c-info-bg);
	}

	.action-btn.delete:hover {
		background: var(--c-error-bg);
	}

	.last-edited {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.75rem;
		color: var(--c-brown-muted);
		margin-top: 0.25rem;
	}
</style>
