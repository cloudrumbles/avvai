<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import ExerciseCard from './exercises/ExerciseCard.svelte';
	import ExerciseTypeSelector from './exercises/ExerciseTypeSelector.svelte';
	import MultipleChoiceEditor from './exercises/MultipleChoiceEditor.svelte';
	import FillInBlankEditor from './exercises/FillInBlankEditor.svelte';
	import ShortAnswerEditor from './exercises/ShortAnswerEditor.svelte';
	import LongAnswerEditor from './exercises/LongAnswerEditor.svelte';
	import { PlusIcon, TrashIcon } from '$lib/components/icons';
	import KolamPattern from '$lib/components/ui/KolamPattern.svelte';

	import type {
		ExerciseGroup,
		Exercise,
		ExerciseContent,
		MultipleChoiceExercise,
		FillInBlankExercise,
		ShortAnswerExercise,
		LongAnswerExercise
	} from '$lib/types/lesson';

	type ExerciseType = 'multiple_choice' | 'fill_in_blank' | 'short_answer' | 'long_answer';

	interface Props {
		group: ExerciseGroup;
		index: number;
		onDelete: () => void;
	}

	let { group = $bindable(), index, onDelete }: Props = $props();

	function generateId(): string {
		return `ex-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
	}

	function addExercise(type: ExerciseType) {
		const newId = generateId();

		let newExercise: Exercise;

		switch (type) {
			case 'multiple_choice':
				newExercise = {
					id: newId,
					content: {
						exercise_type: 'multiple_choice',
						question: '',
						options: [
							{ id: 'a', text: '', correct: false },
							{ id: 'b', text: '', correct: false }
						]
					}
				};
				break;
			case 'fill_in_blank':
				newExercise = {
					id: newId,
					content: {
						exercise_type: 'fill_in_blank',
						text_before: '',
						accepted_answers: ['']
					}
				};
				break;
			case 'short_answer':
				newExercise = {
					id: newId,
					content: {
						exercise_type: 'short_answer',
						question: ''
					}
				};
				break;
			case 'long_answer':
				newExercise = {
					id: newId,
					content: {
						exercise_type: 'long_answer',
						question: ''
					}
				};
				break;
		}

		if (group.exercises.length === 0) {
			group.group_type = type;
		}

		group.exercises = [...group.exercises, newExercise];
	}

	function removeExercise(exIndex: number) {
		group.exercises = group.exercises.filter((_, i) => i !== exIndex);
	}

	function duplicateExercise(exIndex: number) {
		const original = group.exercises[exIndex];
		const duplicated: Exercise = {
			id: generateId(),
			content: JSON.parse(JSON.stringify(original.content))
		};
		const newExercises = [...group.exercises];
		newExercises.splice(exIndex + 1, 0, duplicated);
		group.exercises = newExercises;
	}

	function getExerciseTitle(content: ExerciseContent): string {
		switch (content.exercise_type) {
			case 'multiple_choice': {
				const mcq = content as { exercise_type: 'multiple_choice' } & MultipleChoiceExercise;
				return mcq.question ? truncate(mcq.question, 50) : '';
			}
			case 'fill_in_blank': {
				const fib = content as { exercise_type: 'fill_in_blank' } & FillInBlankExercise;
				return fib.text_before ? truncate(fib.text_before, 50) : '';
			}
			case 'short_answer': {
				const sa = content as { exercise_type: 'short_answer' } & ShortAnswerExercise;
				return sa.question ? truncate(sa.question, 50) : '';
			}
			case 'long_answer': {
				const la = content as { exercise_type: 'long_answer' } & LongAnswerExercise;
				return la.question ? truncate(la.question, 50) : '';
			}
		}
	}

	function truncate(text: string, maxLength: number): string {
		if (text.length <= maxLength) return text;
		return text.slice(0, maxLength) + '...';
	}

	function isExerciseValid(content: ExerciseContent): boolean {
		switch (content.exercise_type) {
			case 'multiple_choice': {
				const mcq = content as { exercise_type: 'multiple_choice' } & MultipleChoiceExercise;
				const hasQuestion = mcq.question.trim().length > 0;
				const hasAtLeastTwoOptions = mcq.options.length >= 2;
				const hasOneCorrect = mcq.options.some((opt) => opt.correct);
				return hasQuestion && hasAtLeastTwoOptions && hasOneCorrect;
			}
			case 'fill_in_blank': {
				const fib = content as { exercise_type: 'fill_in_blank' } & FillInBlankExercise;
				const hasTextBefore = fib.text_before.trim().length > 0;
				const hasAtLeastOneAnswer = fib.accepted_answers.some((ans) => ans.trim().length > 0);
				return hasTextBefore && hasAtLeastOneAnswer;
			}
			case 'short_answer': {
				const sa = content as { exercise_type: 'short_answer' } & ShortAnswerExercise;
				return sa.question.trim().length > 0;
			}
			case 'long_answer': {
				const la = content as { exercise_type: 'long_answer' } & LongAnswerExercise;
				return la.question.trim().length > 0;
			}
		}
	}

	let exerciseCount = $derived(group.exercises.length);
</script>

<div class="exercise-group">
	<div class="group-header">
		<div class="header-left">
			<div class="group-indicator">
				<span class="group-number">{String(index + 1).padStart(2, '0')}</span>
			</div>
			<div class="group-info">
				<h4 class="group-title">Exercise Group</h4>
				<span class="exercise-count">{exerciseCount} exercise{exerciseCount !== 1 ? 's' : ''}</span>
			</div>
		</div>
		<button class="delete-group-btn" onclick={onDelete}>
			<TrashIcon size={14} color="var(--c-terracotta)" />
			<span>Delete</span>
		</button>
	</div>

	<div class="group-settings">
		<TextInput
			label="Instructions for Students"
			value={group.instructions}
			placeholder="e.g. Choose the correct answer based on the passage..."
			oninput={(e) => {
				group.instructions = e.currentTarget.value;
			}}
		/>
	</div>

	<div class="exercises-list">
		{#each group.exercises as exercise, exIndex (exercise.id)}
			<ExerciseCard
				exerciseType={exercise.content.exercise_type}
				title={getExerciseTitle(exercise.content)}
				isValid={isExerciseValid(exercise.content)}
				onDelete={() => removeExercise(exIndex)}
				onDuplicate={() => duplicateExercise(exIndex)}
			>
				{#if exercise.content.exercise_type === 'multiple_choice'}
					<MultipleChoiceEditor bind:exercise={exercise.content} />
				{:else if exercise.content.exercise_type === 'fill_in_blank'}
					<FillInBlankEditor bind:exercise={exercise.content} />
				{:else if exercise.content.exercise_type === 'short_answer'}
					<ShortAnswerEditor bind:exercise={exercise.content} />
				{:else if exercise.content.exercise_type === 'long_answer'}
					<LongAnswerEditor bind:exercise={exercise.content} />
				{/if}
			</ExerciseCard>
		{/each}

		<div class="add-exercise-section">
			<div class="add-header">
				<div class="add-icon">
					<PlusIcon size={16} color="var(--c-terracotta)" />
				</div>
				<span class="add-label">Add Exercise</span>
			</div>
			<ExerciseTypeSelector onSelect={addExercise} />
		</div>
	</div>

	<div class="corner-decoration">
		<KolamPattern variant="corner" size="md" color="terracotta" />
	</div>
</div>

<style>
	.exercise-group {
		position: relative;
		background: linear-gradient(135deg, var(--c-cream) 0%, white 100%);
		border: 1px solid rgba(92, 74, 61, 0.1);
		border-radius: var(--radius-md);
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		box-shadow: var(--shadow-sm);
		overflow: hidden;
	}

	.exercise-group:hover {
		box-shadow: var(--shadow-md);
		border-color: rgba(92, 74, 61, 0.15);
	}

	.group-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: var(--space-4);
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.group-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: linear-gradient(135deg, var(--c-terracotta) 0%, var(--c-terracotta-dark) 100%);
		border-radius: var(--radius-md);
		box-shadow: 0 2px 8px rgba(184, 83, 61, 0.2);
	}

	.group-number {
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 700;
		color: white;
	}

	.group-info {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.group-title {
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.exercise-count {
		font-size: 0.75rem;
		color: var(--c-brown-muted);
		font-weight: 500;
	}

	.delete-group-btn {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: 0.8rem;
		color: var(--c-terracotta);
		background: var(--c-exercises-bg);
		border: 1px solid rgba(184, 83, 61, 0.15);
		cursor: pointer;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-sm);
		font-family: var(--font-heading);
		font-weight: 600;
		transition: all var(--transition-fast);
	}

	.delete-group-btn:hover {
		background: var(--c-terracotta);
		color: white;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(184, 83, 61, 0.25);
	}

	.group-settings {
		max-width: 600px;
	}

	.exercises-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-5);
	}

	.add-exercise-section {
		margin-top: var(--space-2);
		padding-top: var(--space-5);
		border-top: 1px dashed rgba(92, 74, 61, 0.15);
	}

	.add-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-4);
	}

	.add-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: var(--c-exercises-bg);
		border-radius: var(--radius-sm);
	}

	.add-label {
		font-family: var(--font-heading);
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.corner-decoration {
		position: absolute;
		top: var(--space-3);
		right: var(--space-3);
		opacity: 0.2;
		pointer-events: none;
	}

	.exercise-group:hover .corner-decoration {
		opacity: 0.35;
	}
</style>
