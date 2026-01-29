<script lang="ts">
	import type {
		ExercisesSection as ExercisesSectionData,
		Exercise,
		ExerciseContent
	} from '$lib/types/lesson';

	interface Props {
		data: ExercisesSectionData;
	}

	let { data }: Props = $props();

	// State for tracking answers
	let mcqAnswers: Record<string, string> = $state({});
	let mcqChecked: Record<string, boolean> = $state({});

	let fillBlankAnswers: Record<string, string> = $state({});
	let fillBlankChecked: Record<string, boolean> = $state({});
	let fillBlankCorrect: Record<string, boolean> = $state({});

	let shortAnswers: Record<string, string> = $state({});
	let shortAnswerRevealed: Record<string, boolean> = $state({});

	let longAnswers: Record<string, string> = $state({});
	let longAnswerRevealed: Record<string, boolean> = $state({});

	function selectMcqOption(exerciseId: string, optionId: string) {
		if (mcqChecked[exerciseId]) return;
		mcqAnswers[exerciseId] = optionId;
		mcqChecked[exerciseId] = true;
	}

	function checkFillBlank(exercise: Exercise) {
		if (exercise.content.exercise_type !== 'fill_in_blank') return;
		const userAnswer = (fillBlankAnswers[exercise.id] ?? '').trim().toLowerCase();
		const accepted = exercise.content.accepted_answers.map((a) => a.toLowerCase());
		fillBlankCorrect[exercise.id] = accepted.includes(userAnswer);
		fillBlankChecked[exercise.id] = true;
	}

	function retryFillBlank(exerciseId: string) {
		fillBlankChecked[exerciseId] = false;
		fillBlankCorrect[exerciseId] = false;
	}

	function toggleShortAnswer(exerciseId: string) {
		shortAnswerRevealed[exerciseId] = !shortAnswerRevealed[exerciseId];
	}

	function toggleLongAnswer(exerciseId: string) {
		longAnswerRevealed[exerciseId] = !longAnswerRevealed[exerciseId];
	}
</script>

<section class="exercises-section">
	{#each data.exercise_groups as group, groupIndex (groupIndex)}
		<div class="exercise-group">
			<div class="group-header">
				<span class="group-label">{group.instructions}</span>
			</div>

			<div class="exercises-list">
				{#each group.exercises as exercise, exerciseIndex (exercise.id)}
					<div class="exercise-item">
						{#if exercise.content.exercise_type === 'multiple_choice'}
							{@const content = exercise.content}
							{@const isChecked = mcqChecked[exercise.id]}
							{@const selectedId = mcqAnswers[exercise.id]}
							<div class="mcq">
								<p class="question">
									<span class="q-number">{exerciseIndex + 1}.</span>
									{content.question}
								</p>
								<div class="options">
									{#each content.options as option (option.id)}
										{@const isSelected = selectedId === option.id}
										{@const isCorrect = option.correct}
										<button
											type="button"
											class="option"
											class:selected={isSelected}
											class:correct={isChecked && isCorrect}
											class:incorrect={isChecked && isSelected && !isCorrect}
											class:dimmed={isChecked && !isCorrect && !isSelected}
											disabled={isChecked}
											onclick={() => selectMcqOption(exercise.id, option.id)}
										>
											<span class="option-letter">{option.id})</span>
											<span class="option-text">{option.text}</span>
											{#if isChecked && isCorrect}
												<span class="option-icon">✓</span>
											{:else if isChecked && isSelected && !isCorrect}
												<span class="option-icon">✗</span>
											{/if}
										</button>
									{/each}
								</div>
							</div>

						{:else if exercise.content.exercise_type === 'fill_in_blank'}
							{@const content = exercise.content}
							{@const isChecked = fillBlankChecked[exercise.id]}
							{@const isCorrect = fillBlankCorrect[exercise.id]}
							<div class="fill-blank">
								<p class="question">
									<span class="q-number">{exerciseIndex + 1}.</span>
									<span class="fill-sentence">
										{content.text_before}
										<span class="input-wrapper" class:correct={isChecked && isCorrect} class:incorrect={isChecked && !isCorrect}>
											<input
												type="text"
												class="fill-input"
												placeholder="______"
												bind:value={fillBlankAnswers[exercise.id]}
												disabled={isChecked && isCorrect}
											/>
											{#if isChecked}
												<span class="input-icon">{isCorrect ? '✓' : '✗'}</span>
											{/if}
										</span>
										{#if content.text_after}
											{content.text_after}
										{/if}
									</span>
								</p>
								{#if content.hint}
									<p class="hint">{content.hint}</p>
								{/if}
								<div class="actions">
									{#if !isChecked}
										<button type="button" class="btn primary" onclick={() => checkFillBlank(exercise)}>
											Check
										</button>
									{:else if !isCorrect}
										<button type="button" class="btn" onclick={() => retryFillBlank(exercise.id)}>
											Try again
										</button>
									{/if}
								</div>
							</div>

						{:else if exercise.content.exercise_type === 'short_answer'}
							{@const content = exercise.content}
							{@const isRevealed = shortAnswerRevealed[exercise.id]}
							<div class="short-answer">
								<p class="question">
									<span class="q-number">{exerciseIndex + 1}.</span>
									{content.question}
								</p>
								<textarea
									class="answer-input"
									placeholder="உங்கள் பதிலை இங்கே எழுதுங்கள்..."
									bind:value={shortAnswers[exercise.id]}
								></textarea>
								{#if content.model_answer}
									<div class="actions">
										<button type="button" class="btn" onclick={() => toggleShortAnswer(exercise.id)}>
											{isRevealed ? 'Hide answer' : 'Show answer'}
										</button>
									</div>
									{#if isRevealed}
										<div class="model-answer">
											<span class="model-label">Answer:</span>
											{content.model_answer}
										</div>
									{/if}
								{/if}
							</div>

						{:else if exercise.content.exercise_type === 'long_answer'}
							{@const content = exercise.content}
							{@const isRevealed = longAnswerRevealed[exercise.id]}
							<div class="long-answer">
								<p class="question">
									<span class="q-number">{exerciseIndex + 1}.</span>
									{content.question}
								</p>
								{#if content.min_words}
									<p class="word-hint">குறைந்தது {content.min_words} சொற்கள்</p>
								{/if}
								<textarea
									class="answer-input large"
									placeholder="உங்கள் பதிலை இங்கே எழுதுங்கள்..."
									bind:value={longAnswers[exercise.id]}
								></textarea>
								{#if content.model_answer}
									<div class="actions">
										<button type="button" class="btn" onclick={() => toggleLongAnswer(exercise.id)}>
											{isRevealed ? 'Hide answer' : 'Show answer'}
										</button>
									</div>
									{#if isRevealed}
										<div class="model-answer">
											<span class="model-label">Answer:</span>
											{content.model_answer}
										</div>
									{/if}
								{/if}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	{/each}
</section>

<style>
	.exercises-section {
		font-family: 'Catamaran', sans-serif;
	}

	/* ── Group ── */
	.exercise-group {
		margin-bottom: 2.5em;
	}

	.exercise-group:last-child {
		margin-bottom: 0;
	}

	.group-header {
		margin-bottom: 1.25em;
	}

	.group-label {
		font-size: 0.95em;
		font-weight: 600;
		color: var(--red);
	}

	/* ── Exercise list ── */
	.exercises-list {
		display: flex;
		flex-direction: column;
		gap: 1.75em;
	}

	.exercise-item {
		/* container for each question */
	}

	/* ── Question text ── */
	.question {
		margin: 0 0 1em;
		padding: 0.75em 1em;
		line-height: 1.6;
		color: var(--ink);
		font-weight: 500;
		background: white;
		border-radius: 8px;
		border-left: 3px solid var(--red);
		box-shadow: 0 2px 8px rgba(26, 14, 6, 0.06);
	}

	.q-number {
		font-weight: 700;
		color: var(--red);
		margin-right: 0.4em;
	}

	/* ── MCQ Options ── */
	.options {
		display: flex;
		flex-direction: column;
		gap: 0.625em;
	}

	.option {
		display: flex;
		align-items: center;
		gap: 0.75em;
		width: 100%;
		padding: 0.75em 1em;
		background: transparent;
		border: 1px solid var(--cream-mid);
		border-radius: 6px;
		font-family: inherit;
		font-size: 0.95em;
		color: var(--ink-soft);
		text-align: left;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.option:hover:not(:disabled) {
		border-color: var(--stone);
		color: var(--ink);
	}

	.option:disabled {
		cursor: default;
	}

	.option.selected {
		border-color: var(--ink);
		color: var(--ink);
		background: rgba(26, 14, 6, 0.03);
	}

	.option.correct {
		background: var(--green-faint);
		border-color: var(--green);
		color: var(--green);
	}

	.option.incorrect {
		background: var(--red-faint);
		border-color: var(--red);
		color: var(--red);
	}

	.option.dimmed {
		opacity: 0.4;
	}

	.option-letter {
		font-weight: 700;
		min-width: 1.5em;
		color: var(--stone);
	}

	.option-text {
		flex: 1;
	}

	.option-icon {
		font-weight: 700;
		font-size: 1.1em;
	}

	.option.correct .option-icon {
		color: var(--green);
	}

	.option.incorrect .option-icon {
		color: var(--red);
	}

	/* ── Fill in blank ── */
	.fill-blank {
		/* container */
	}

	.fill-sentence {
		line-height: 2.2;
	}

	.input-wrapper {
		display: inline-flex;
		align-items: center;
		gap: 0.3em;
		position: relative;
	}

	.fill-input {
		width: 7em;
		padding: 0.2em 0.4em;
		border: none;
		border-bottom: 2px solid var(--cream-mid);
		background: transparent;
		font-family: inherit;
		font-size: inherit;
		color: var(--ink);
		text-align: center;
		transition: border-color 0.15s ease;
	}

	.fill-input:focus {
		outline: none;
		border-color: var(--red);
	}

	.fill-input::placeholder {
		color: var(--stone);
		opacity: 0.5;
	}

	.input-wrapper.correct .fill-input {
		border-color: var(--green);
		color: var(--green);
	}

	.input-wrapper.incorrect .fill-input {
		border-color: var(--red);
		color: var(--red);
	}

	.input-icon {
		font-weight: 700;
		font-size: 0.9em;
	}

	.input-wrapper.correct .input-icon {
		color: var(--green);
	}

	.input-wrapper.incorrect .input-icon {
		color: var(--red);
	}

	.hint {
		font-size: 0.85em;
		color: var(--stone);
		margin: 0.5em 0 0;
	}

	/* ── Buttons ── */
	.actions {
		margin-top: 0.75em;
		display: flex;
		gap: 0.5em;
	}

	.btn {
		padding: 0.5em 1.25em;
		border: 1.5px solid var(--cream-mid);
		border-radius: 2em;
		background: transparent;
		font-family: inherit;
		font-size: 0.85em;
		font-weight: 600;
		color: var(--stone);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn:hover {
		border-color: var(--ink);
		color: var(--ink);
	}

	.btn.primary {
		border-color: var(--red);
		background: var(--red);
		color: var(--cream);
	}

	.btn.primary:hover {
		background: var(--red-deep);
		border-color: var(--red-deep);
	}

	/* ── Short/Long answer ── */
	.answer-input {
		width: 100%;
		min-height: 5em;
		padding: 0.75em;
		border: 1.5px solid var(--cream-mid);
		border-radius: 0.5em;
		background: transparent;
		font-family: inherit;
		font-size: 0.95em;
		color: var(--ink);
		resize: vertical;
		transition: border-color 0.15s ease;
	}

	.answer-input:focus {
		outline: none;
		border-color: var(--red);
	}

	.answer-input.large {
		min-height: 8em;
	}

	.answer-input::placeholder {
		color: var(--stone);
		opacity: 0.6;
	}

	.word-hint {
		font-size: 0.85em;
		color: var(--stone);
		margin: 0 0 0.5em;
	}

	/* ── Model answer ── */
	.model-answer {
		margin-top: 0.75em;
		padding: 0.75em 1em;
		background: var(--green-faint);
		border-radius: 0.5em;
		color: var(--ink);
		line-height: 1.5;
	}

	.model-label {
		font-weight: 600;
		color: var(--green);
		margin-right: 0.3em;
	}
</style>
