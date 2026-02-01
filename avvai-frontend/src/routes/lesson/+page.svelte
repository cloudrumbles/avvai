<script lang="ts">
	import { LessonList, LessonReader } from '$lib/components/lesson';
	import type { Lesson, LessonSummary } from '$lib/types/lesson';

	interface Props {
		data: {
			lessons: LessonSummary[];
			progress: Record<string, boolean>;
		};
	}

	let { data }: Props = $props();

	let activeLesson = $state<Lesson | null>(null);
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
</script>

{#if activeLesson}
	<LessonReader lesson={activeLesson} onclose={closeLesson} />
{:else if loading}
	<div class="loading-container">
		<p class="loading">Loading...</p>
	</div>
{:else}
	<LessonList lessons={data.lessons} progress={data.progress} onselect={openLesson} />
{/if}

<style>
	.loading-container {
		min-height: 100dvh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg);
	}

	.loading {
		font-family: var(--font-sans);
		color: var(--color-text-subtle);
		text-align: center;
	}
</style>