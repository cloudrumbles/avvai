<script lang="ts">
	import Reader from '$lib/components/Reader.svelte';

	interface Lesson {
		id: string;
		title: string;
		description: string;
		content?: string;
	}

	interface Props {
		data: {
			lessons: Lesson[];
			progress: Record<string, boolean>;
		};
	}

	let { data }: Props = $props();

	let activeLesson: Lesson | null = $state(null);
	let loading = $state(false);

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
		<header class="lesson-header">
			<button class="back-btn" onclick={closeLesson} aria-label="Back to lessons">
				<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
					<path d="M12 4L6 10L12 16" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
			</button>
			<span class="lesson-title">{activeLesson.title}</span>
			<button class="complete-btn" onclick={() => markComplete(activeLesson!.id)}>
				{data.progress[activeLesson.id] ? '✓ Done' : 'Mark done'}
			</button>
		</header>
		<Reader text={activeLesson.content ?? ''} />
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
		flex-direction: column;
		height: 100dvh;
		background: var(--cream);
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
</style>
