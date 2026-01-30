<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import type { LongAnswerExercise } from '$lib/types/lesson';

	interface Props {
		exercise: LongAnswerExercise;
	}

	let { exercise = $bindable() }: Props = $props();
</script>

<div class="la-editor">
	<TextInput
		label="Question"
		value={exercise.question}
		placeholder="Enter the essay question..."
		oninput={(e) => {
			exercise.question = e.currentTarget.value;
		}}
	/>

	<div class="meta-row">
		<label class="meta-label">
			Minimum Words:
			<input 
				type="number" 
				class="number-input" 
				value={exercise.min_words ?? ''} 
				oninput={(e) => exercise.min_words = e.currentTarget.valueAsNumber || undefined}
				placeholder="e.g. 100"
			/>
		</label>
	</div>

	<Textarea
		label="Model Answer / Key Points (for grader)"
		value={exercise.model_answer ?? ''}
		placeholder="Outline key points or provide a sample answer..."
		oninput={(e) => {
			exercise.model_answer = e.currentTarget.value || undefined;
		}}
		rows={6}
	/>
</div>

<style>
	.la-editor {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.meta-row {
		display: flex;
		gap: 1rem;
	}

	.meta-label {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--c-brown);
	}

	.number-input {
		padding: 0.5rem;
		border: 1px solid rgba(92, 74, 61, 0.2);
		border-radius: 6px;
		font-family: var(--font-sans);
		width: 120px;
	}
</style>