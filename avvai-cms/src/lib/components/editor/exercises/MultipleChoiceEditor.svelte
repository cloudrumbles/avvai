<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import type { MultipleChoiceExercise, ChoiceOption } from '$lib/types/lesson';

	interface Props {
		exercise: MultipleChoiceExercise;
	}

	let { exercise = $bindable() }: Props = $props();

	function addOption() {
		// Generate simple ID
		const id = Math.random().toString(36).substring(2, 9);
		exercise.options = [...exercise.options, { id, text: '', correct: false }];
	}

	function removeOption(index: number) {
		exercise.options = exercise.options.filter((_, i) => i !== index);
	}

	function setCorrect(index: number) {
		// Only one correct answer usually, but logic allows multiple if needed. 
		// For typical MCQ, we might want to unset others, but let's keep it flexible or radio-like behavior.
		// Let's implement radio behavior (only one correct)
		exercise.options = exercise.options.map((opt, i) => ({
			...opt,
			correct: i === index
		}));
	}
</script>

<div class="mcq-editor">
	<TextInput
		label="Question"
		value={exercise.question}
		placeholder="Enter the question..."
		oninput={(e) => {
			exercise.question = e.currentTarget.value;
		}}
	/>

	<div class="options-list">
		<label class="options-label">Options</label>
		{#each exercise.options as option, index}
			<div class="option-row" class:correct={option.correct}>
				<div class="correct-toggle">
					<input
						type="radio"
						name={`correct-${exercise.question.substring(0, 10)}`}
						checked={option.correct}
						onchange={() => setCorrect(index)}
						title="Mark as correct answer"
					/>
				</div>
				<div class="option-text">
					<TextInput
						value={option.text}
						placeholder={`Option ${index + 1}`}
						oninput={(e) => {
							option.text = e.currentTarget.value;
						}}
					/>
				</div>
				<button
					class="remove-btn"
					onclick={() => removeOption(index)}
					disabled={exercise.options.length <= 2}
					title="Remove option"
				>
					×
				</button>
			</div>
		{/each}
		
		<button class="add-btn" onclick={addOption}>+ Add Option</button>
	</div>
</div>

<style>
	.mcq-editor {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.options-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.options-label {
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--c-brown);
		margin-bottom: 0.25rem;
	}

	.option-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.25rem;
		border-radius: 6px;
		transition: background-color 0.2s;
	}

	.option-row.correct {
		background-color: rgba(72, 187, 120, 0.1);
	}

	.correct-toggle {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 1.5rem;
	}

	.correct-toggle input {
		width: 1.25rem;
		height: 1.25rem;
		accent-color: #48bb78;
		cursor: pointer;
	}

	.option-text {
		flex: 1;
	}

	.remove-btn {
		width: 1.75rem;
		height: 1.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		color: #e53e3e;
		font-size: 1.25rem;
		cursor: pointer;
		opacity: 0.5;
		transition: opacity 0.2s;
	}

	.remove-btn:hover:not(:disabled) {
		opacity: 1;
		background-color: rgba(229, 62, 62, 0.1);
		border-radius: 4px;
	}

	.remove-btn:disabled {
		opacity: 0.2;
		cursor: not-allowed;
	}

	.add-btn {
		margin-top: 0.5rem;
		background: none;
		border: 1px dashed rgba(92, 74, 61, 0.3);
		border-radius: 6px;
		padding: 0.5rem;
		color: var(--c-brown);
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-btn:hover {
		background-color: rgba(92, 74, 61, 0.05);
		border-color: var(--c-brown);
	}
</style>