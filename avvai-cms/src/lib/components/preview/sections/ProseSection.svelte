<script lang="ts">
	import { ProseSection } from 'avvai-frontend/components/sections';
	import type { ProseSection as ProseSectionType } from '$lib/types/lesson';

	interface Props {
		data: ProseSectionType;
	}

	let { data }: Props = $props();
</script>

<ProseSection {data} />
