<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import type { ShortAnswerExercise } from '$lib/types/lesson';

	interface Props {
		exercise: ShortAnswerExercise;
	}

	let { exercise = $bindable() }: Props = $props();
</script>

<div class="sa-editor">
	<TextInput
		label="Question"
		value={exercise.question}
		placeholder="Enter the question..."
		oninput={(e) => {
			exercise.question = e.currentTarget.value;
		}}
	/>

	<Textarea
		label="Model Answer (for grading reference)"
		value={exercise.model_answer ?? ''}
		placeholder="Provide a sample correct answer..."
		oninput={(e) => {
			exercise.model_answer = e.currentTarget.value || undefined;
		}}
		rows={3}
	/>
</div>

<style>
	.sa-editor {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
</style>