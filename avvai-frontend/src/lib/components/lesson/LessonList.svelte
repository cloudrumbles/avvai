<script lang="ts">
	import type { LessonSummary } from 'avvai-frontend/types/lesson';

	interface Props {
		lessons: LessonSummary[];
		progress: Record<string, boolean>;
		onselect: (id: string) => void;
	}

	let { lessons, progress, onselect }: Props = $props();
</script>

<div class="lesson-list">
	<header class="list-header">
		<h1>Lessons</h1>
	</header>

	<ul class="lessons">
		{#each lessons as lesson (lesson.id)}
			<li>
				<button class="lesson-card" onclick={() => onselect(lesson.id)}>
					<div class="lesson-info">
						<h2 class="lesson-card-title">{lesson.title}</h2>
						<p class="lesson-card-desc">{lesson.description}</p>
					</div>
					{#if progress[lesson.id]}
						<span class="completed-badge">✓</span>
					{/if}
				</button>
			</li>
		{/each}
	</ul>
</div>

<style>
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
</style>
