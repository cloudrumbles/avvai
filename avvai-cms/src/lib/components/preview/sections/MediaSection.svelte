<script lang="ts">
	import { MediaSection } from 'avvai-frontend/components/sections';
	import type { MediaSection as MediaSectionType } from '$lib/types/lesson';

	interface Props {
		data: MediaSectionType;
	}

	let { data }: Props = $props();
</script>

<MediaSection {data} />
