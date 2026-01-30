<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import type { FillInBlankExercise } from '$lib/types/lesson';

	interface Props {
		exercise: FillInBlankExercise;
	}

	let { exercise = $bindable() }: Props = $props();

	function addAcceptedAnswer() {
		exercise.accepted_answers = [...exercise.accepted_answers, ''];
	}

	function removeAcceptedAnswer(index: number) {
		exercise.accepted_answers = exercise.accepted_answers.filter((_, i) => i !== index);
	}
</script>

<div class="fib-editor">
	<div class="sentence-builder">
		<label class="section-label">Sentence Construction</label>
		<div class="sentence-row">
			<div class="input-group">
				<span class="label-text">Text Before</span>
				<TextInput
					value={exercise.text_before}
					placeholder="Start of sentence..."
					oninput={(e) => {
						exercise.text_before = e.currentTarget.value;
					}}
				/>
			</div>
			
			<div class="blank-preview">
				[ BLANK ]
			</div>

			<div class="input-group">
				<span class="label-text">Text After (Optional)</span>
				<TextInput
					value={exercise.text_after ?? ''}
					placeholder="End of sentence..."
					oninput={(e) => {
						exercise.text_after = e.currentTarget.value || undefined;
					}}
				/>
			</div>
		</div>
	</div>

	<div class="answers-section">
		<label class="section-label">Accepted Answer(s)</label>
		<p class="help-text">Add all valid variations of the correct answer.</p>
		
		<div class="answers-list">
			{#each exercise.accepted_answers as answer, index}
				<div class="answer-row">
					<TextInput
						value={answer}
						placeholder="Correct answer..."
						oninput={(e) => {
							exercise.accepted_answers[index] = e.currentTarget.value;
						}}
					/>
					<button
						class="remove-btn"
						onclick={() => removeAcceptedAnswer(index)}
						disabled={exercise.accepted_answers.length === 1}
						title="Remove answer"
					>
						×
					</button>
				</div>
			{/each}
			<button class="add-link" onclick={addAcceptedAnswer}>+ Add Alternative Answer</button>
		</div>
	</div>

	<div class="hint-section">
		<TextInput
			label="Hint (Optional)"
			value={exercise.hint ?? ''}
			placeholder="Provide a clue..."
			oninput={(e) => {
				exercise.hint = e.currentTarget.value || undefined;
			}}
		/>
	</div>
</div>

<style>
	.fib-editor {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.section-label {
		display: block;
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown);
		margin-bottom: 0.5rem;
	}

	.sentence-row {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
	}

	.input-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.label-text {
		font-size: 0.75rem;
		color: var(--c-brown-light);
	}

	.blank-preview {
		align-self: center;
		padding: 0.5rem 1rem;
		background-color: rgba(92, 74, 61, 0.1);
		border-radius: 4px;
		font-family: monospace;
		font-weight: 600;
		color: var(--c-brown);
		margin-top: 1rem; /* Align with inputs roughly */
	}

	.help-text {
		font-size: 0.8rem;
		color: var(--c-brown-light);
		margin: -0.25rem 0 0.5rem;
	}

	.answers-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.answer-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	/* Make input grow */
	.answer-row :global(.text-input-wrapper) {
		flex: 1;
	}
	.answer-row :global(> div:first-child) {
		flex: 1;
	}

	.remove-btn {
		width: 2rem;
		height: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		color: #e53e3e;
		font-size: 1.25rem;
		cursor: pointer;
		opacity: 0.5;
	}

	.remove-btn:hover:not(:disabled) {
		opacity: 1;
		background-color: rgba(229, 62, 62, 0.1);
		border-radius: 4px;
	}

	.add-link {
		background: none;
		border: none;
		color: var(--c-terracotta);
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		align-self: flex-start;
		padding: 0;
	}

	.add-link:hover {
		text-decoration: underline;
	}
</style>