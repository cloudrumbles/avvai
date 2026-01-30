<script lang="ts">
	import { DialogueSection } from 'avvai-frontend/components/sections';
	import type { DialogueSection as DialogueSectionType } from '$lib/types/lesson';

	interface Props {
		data: DialogueSectionType;
	}

	let { data }: Props = $props();
</script>

<DialogueSection {data} />
