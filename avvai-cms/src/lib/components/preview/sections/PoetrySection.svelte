<script lang="ts">
	import { PoetrySection } from 'avvai-frontend/components/sections';
	import type { PoetrySection as PoetrySectionType } from '$lib/types/lesson';

	interface Props {
		data: PoetrySectionType;
	}

	let { data }: Props = $props();
</script>

<PoetrySection {data} />
