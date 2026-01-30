<script lang="ts">
	import { VocabularySection } from 'avvai-frontend/components/sections';
	import type { VocabularySection as VocabularySectionType } from '$lib/types/lesson';

	interface Props {
		data: VocabularySectionType;
	}

	let { data }: Props = $props();
</script>

<VocabularySection {data} />
