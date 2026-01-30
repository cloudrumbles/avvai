<script lang="ts">
	import { ExercisesSection } from 'avvai-frontend/components/sections';
	import type { ExercisesSection as ExercisesSectionType } from '$lib/types/lesson';

	interface Props {
		data: ExercisesSectionType;
	}

	let { data }: Props = $props();
</script>

<ExercisesSection {data} />
