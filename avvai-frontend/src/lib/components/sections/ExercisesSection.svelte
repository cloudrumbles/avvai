<script lang="ts">
	import type {
		ExercisesSection as ExercisesSectionData,
		Exercise,
		ExerciseContent
	} from '$lib/types/lesson';
	import ClickableText from '$lib/components/ClickableText.svelte';

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

	function autoResize(event: Event) {
		const textarea = event.target as HTMLTextAreaElement;
		textarea.style.height = 'auto';
		textarea.style.height = textarea.scrollHeight + 'px';
	}

	// Tamil IME using Google Input Tools API
	let currentAbortController: AbortController | null = null;

	async function transliterate(text: string, signal?: AbortSignal): Promise<string[]> {
		if (!text.trim()) return [];
		const url = new URL('https://inputtools.google.com/request');
		url.searchParams.set('text', text);
		url.searchParams.set('itc', 'ta-t-i0-und'); // Tamil transliteration
		url.searchParams.set('num', '5');
		url.searchParams.set('cp', '0');
		url.searchParams.set('cs', '1');
		url.searchParams.set('ie', 'utf-8');
		url.searchParams.set('oe', 'utf-8');
		url.searchParams.set('app', 'demopage');

		try {
			const res = await fetch(url.toString(), { signal });
			const data = await res.json();
			if (data[0] === 'SUCCESS' && data[1]?.[0]?.[1]) {
				return data[1][0][1] as string[];
			}
		} catch (e) {
			// Ignore abort errors, silently fail others
			if (e instanceof Error && e.name === 'AbortError') return [];
		}
		return [];
	}

	function tamilIme(node: HTMLInputElement | HTMLTextAreaElement) {
		let suggestions: string[] = [];
		let selectedIndex = 0;
		let wordStart = 0;
		let wordEnd = 0;
		let dropdown: HTMLDivElement | null = null;
		let debounceTimer: ReturnType<typeof setTimeout> | null = null;
		let isComposing = false;
		let isLoading = false;

		function getWordAtCursor(): { word: string; start: number; end: number } {
			const value = node.value;
			const cursor = node.selectionStart ?? value.length;
			let start = cursor;
			let end = cursor;
			// Find word boundaries (Latin letters only for transliteration)
			while (start > 0 && /[a-zA-Z]/.test(value[start - 1])) start--;
			while (end < value.length && /[a-zA-Z]/.test(value[end])) end++;
			return { word: value.slice(start, end), start, end };
		}

		function createDropdown() {
			if (dropdown) return;
			dropdown = document.createElement('div');
			dropdown.className = 'tamil-ime-dropdown';
			document.body.appendChild(dropdown);

			// Handle click for option selection (works for both touch and mouse)
			dropdown.addEventListener('click', handleDropdownClick);
			// Prevent blur on desktop
			dropdown.addEventListener('mousedown', handleDropdownMousedown);
		}

		function positionDropdown() {
			if (!dropdown) return;

			dropdown.style.position = 'fixed';
			dropdown.style.left = '0';
			dropdown.style.right = '0';
			dropdown.style.width = '100%';

			const viewport = window.visualViewport;
			if (viewport) {
				// Position at bottom of visual viewport (above keyboard)
				const top = viewport.offsetTop + viewport.height - 48;
				dropdown.style.top = `${top}px`;
				dropdown.style.bottom = 'auto';
			} else {
				// Fallback: pin to bottom
				dropdown.style.bottom = '0';
				dropdown.style.top = 'auto';
			}
		}

		function renderDropdown() {
			if (!dropdown) return;
			if (suggestions.length === 0 && !isLoading) {
				dropdown.style.display = 'none';
				return;
			}
			dropdown.style.display = 'flex';

			if (isLoading && suggestions.length === 0) {
				dropdown.innerHTML = '<div class="tamil-ime-loading">...</div>';
			} else {
				dropdown.innerHTML = suggestions
					.map(
						(s, i) =>
							`<div class="tamil-ime-option${i === selectedIndex ? ' selected' : ''}" data-index="${i}"><span class="tamil-ime-key">${i + 1}</span>${s}</div>`
					)
					.join('');
			}
			positionDropdown();
		}

		function hideDropdown() {
			if (dropdown) {
				dropdown.style.display = 'none';
			}
			suggestions = [];
			selectedIndex = 0;
			isLoading = false;
		}

		function selectSuggestion(index: number, addSpace = false) {
			const selected = suggestions[index];
			if (!selected) return;
			const value = node.value;
			const suffix = addSpace ? ' ' : '';
			const newValue = value.slice(0, wordStart) + selected + suffix + value.slice(wordEnd);
			node.value = newValue;
			const newCursor = wordStart + selected.length + suffix.length;
			node.setSelectionRange(newCursor, newCursor);
			node.dispatchEvent(new Event('input', { bubbles: true }));
			hideDropdown();
		}

		async function handleInput() {
			// Skip if in composition mode (mobile keyboard predictive text)
			if (isComposing) return;

			const { word, start, end } = getWordAtCursor();
			if (!word || word.length < 1) {
				hideDropdown();
				return;
			}
			wordStart = start;
			wordEnd = end;

			// Cancel any pending request
			if (currentAbortController) {
				currentAbortController.abort();
			}

			if (debounceTimer) clearTimeout(debounceTimer);
			debounceTimer = setTimeout(async () => {
				isLoading = true;
				createDropdown();
				renderDropdown();

				currentAbortController = new AbortController();
				const results = await transliterate(word, currentAbortController.signal);
				currentAbortController = null;
				isLoading = false;

				if (results.length > 0) {
					suggestions = results;
					selectedIndex = 0;
					renderDropdown();
				} else {
					hideDropdown();
				}
			}, 150);
		}

		function handleKeydown(e: KeyboardEvent) {
			// Number keys 1-5 to select suggestions
			if (suggestions.length > 0 && /^[1-5]$/.test(e.key)) {
				const index = parseInt(e.key, 10) - 1;
				if (index < suggestions.length) {
					e.preventDefault();
					selectSuggestion(index);
					return;
				}
			}

			if (suggestions.length === 0) return;

			if (e.key === 'ArrowDown') {
				e.preventDefault();
				selectedIndex = (selectedIndex + 1) % suggestions.length;
				renderDropdown();
			} else if (e.key === 'ArrowUp') {
				e.preventDefault();
				selectedIndex = (selectedIndex - 1 + suggestions.length) % suggestions.length;
				renderDropdown();
			} else if (e.key === 'Enter' || e.key === 'Tab') {
				if (suggestions.length > 0) {
					e.preventDefault();
					selectSuggestion(selectedIndex);
				}
			} else if (e.key === 'Escape') {
				hideDropdown();
			} else if (e.key === ' ') {
				// Space commits the selected suggestion and adds space
				if (suggestions.length > 0) {
					e.preventDefault();
					selectSuggestion(selectedIndex, true);
				}
			}
		}

		// Click handler for option selection (works for both touch tap and mouse click)
		function handleDropdownClick(e: MouseEvent) {
			const target = e.target as HTMLElement;
			const option = target.closest('.tamil-ime-option') as HTMLElement | null;
			if (option) {
				const index = parseInt(option.dataset.index ?? '0', 10);
				selectSuggestion(index);
				node.focus();
			}
		}

		// Mousedown to prevent blur on desktop
		function handleDropdownMousedown(e: MouseEvent) {
			e.preventDefault();
		}

		function handleBlur() {
			// Delay to allow click events to fire before hiding
			setTimeout(() => {
				hideDropdown();
			}, 150);
		}

		// Composition events for mobile keyboard predictive text
		function handleCompositionStart() {
			isComposing = true;
		}

		function handleCompositionEnd() {
			isComposing = false;
			// Trigger input handling after composition ends
			handleInput();
		}

		// Reposition dropdown when viewport changes (mobile keyboard open/close)
		function handleViewportResize() {
			if (dropdown && dropdown.style.display !== 'none') {
				positionDropdown();
			}
		}

		node.addEventListener('input', handleInput);
		node.addEventListener('keydown', handleKeydown as EventListener);
		node.addEventListener('blur', handleBlur);
		node.addEventListener('compositionstart', handleCompositionStart);
		node.addEventListener('compositionend', handleCompositionEnd);

		// Listen for viewport changes (mobile keyboard)
		window.visualViewport?.addEventListener('resize', handleViewportResize);
		window.visualViewport?.addEventListener('scroll', handleViewportResize);

		return {
			destroy() {
				if (debounceTimer) clearTimeout(debounceTimer);
				if (currentAbortController) currentAbortController.abort();
				node.removeEventListener('input', handleInput);
				node.removeEventListener('keydown', handleKeydown as EventListener);
				node.removeEventListener('blur', handleBlur);
				node.removeEventListener('compositionstart', handleCompositionStart);
				node.removeEventListener('compositionend', handleCompositionEnd);
				window.visualViewport?.removeEventListener('resize', handleViewportResize);
				window.visualViewport?.removeEventListener('scroll', handleViewportResize);
				if (dropdown) {
					dropdown.removeEventListener('click', handleDropdownClick);
					dropdown.removeEventListener('mousedown', handleDropdownMousedown);
					dropdown.remove();
					dropdown = null;
				}
			}
		};
	}

	// Parse hint like "(option1, option2, option3)" into dropdown options
	function parseHintOptions(hint?: string): string[] | null {
		if (!hint) return null;
		const match = hint.match(/^\s*\((.+)\)\s*$/);
		if (!match) return null;
		return match[1].split(',').map(s => s.trim()).filter(s => s.length > 0);
	}
</script>

<section class="exercises-section">
	{#each data.exercise_groups as group, groupIndex (groupIndex)}
		<div class="exercise-group">
			<div class="group-header">
				<span class="group-label"><ClickableText text={group.instructions} /></span>
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
									<ClickableText text={content.question} />
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
											<span class="option-text"><ClickableText text={option.text} /></span>
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
							{@const dropdownOptions = content.options ?? parseHintOptions(content.hint)}
							<div class="fill-blank">
								<p class="question">
									<span class="q-number">{exerciseIndex + 1}.</span>
									<span class="fill-sentence">
										<ClickableText text={content.text_before} />
										<span class="input-wrapper" class:correct={isChecked && isCorrect} class:incorrect={isChecked && !isCorrect}>
											{#if dropdownOptions && dropdownOptions.length > 0}
												<select
													class="fill-select"
													bind:value={fillBlankAnswers[exercise.id]}
													disabled={isChecked && isCorrect}
												>
													<option value="" disabled selected>தேர்வு செய்க</option>
												{#each dropdownOptions as option (option)}
													<option value={option}>{option}</option>
												{/each}
												</select>
											{:else}
							<input
								type="text"
								class="fill-input"
								id={`exercise-fib-${exercise.id}`}
								placeholder="______"
								bind:value={fillBlankAnswers[exercise.id]}
								disabled={isChecked && isCorrect}
								use:tamilIme
							/>
											{/if}
											{#if isChecked}
												<span class="input-icon">{isCorrect ? '✓' : '✗'}</span>
											{/if}
										</span>
										{#if content.text_after}
											<ClickableText text={content.text_after} />
										{/if}
									</span>
								</p>
								{#if content.hint && !parseHintOptions(content.hint)}
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
									<ClickableText text={content.question} />
								</p>
							<textarea
								class="answer-input"
								id={`exercise-short-${exercise.id}`}
								placeholder="உங்கள் பதிலை இங்கே எழுதுங்கள்..."
								bind:value={shortAnswers[exercise.id]}
								oninput={autoResize}
								use:tamilIme
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
											<ClickableText text={content.model_answer} />
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
									<ClickableText text={content.question} />
								</p>
								{#if content.min_words}
									<p class="word-hint">குறைந்தது {content.min_words} சொற்கள்</p>
								{/if}
							<textarea
								class="answer-input large"
								id={`exercise-long-${exercise.id}`}
								placeholder="உங்கள் பதிலை இங்கே எழுதுங்கள்..."
								bind:value={longAnswers[exercise.id]}
								oninput={autoResize}
								use:tamilIme
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
											<ClickableText text={content.model_answer} />
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
		font-family: var(--font-sans);
	}

	/* ── Group ── */
	.exercise-group {
		margin-bottom: var(--space-8);
	}

	.exercise-group:last-child {
		margin-bottom: 0;
	}

	.group-header {
		margin-bottom: var(--space-5);
	}

	.group-label {
		font-size: var(--font-size-2-5);
		font-weight: 600;
		color: var(--color-accent);
	}

	/* ── Exercise list ── */
	.exercises-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-7);
	}


	/* ── Question text ── */
	.question {
		margin: 0 0 var(--space-4);
		padding: var(--space-3) var(--space-4);
		line-height: var(--line-height-1-6);
		color: var(--color-text);
		font-weight: 500;
		background: white;
		border-radius: var(--radius-2);
		border-left: 3px solid var(--color-accent);
		box-shadow: var(--shadow-1);
	}

	.q-number {
		font-weight: 700;
		color: var(--color-accent);
		margin-right: var(--space-1);
	}

	/* ── MCQ Options ── */
	.options {
		display: flex;
		flex-direction: column;
		gap: var(--space-2-5);
	}

	.option {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		width: 100%;
		padding: var(--space-3) var(--space-4);
		background: transparent;
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-1-5);
		font-family: inherit;
		font-size: var(--font-size-2-5);
		color: var(--color-text-muted);
		text-align: left;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.option:hover:not(:disabled) {
		border-color: var(--color-text-subtle);
		color: var(--color-text);
	}

	.option:disabled {
		cursor: default;
	}

	.option.selected {
		border-color: var(--color-text);
		color: var(--color-text);
		background: var(--overlay-ink-soft);
	}

	.option.correct {
		background: var(--color-success-tint);
		border-color: var(--color-success);
		color: var(--color-success);
	}

	.option.incorrect {
		background: var(--color-accent-tint);
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.option.dimmed {
		opacity: 0.4;
	}

	.option-letter {
		font-weight: 700;
		min-width: var(--space-6);
		color: var(--color-text-subtle);
	}

	.option-text {
		flex: 1;
	}

	.option-icon {
		font-weight: 700;
		font-size: var(--font-size-4);
	}

	.option.correct .option-icon {
		color: var(--color-success);
	}

	.option.incorrect .option-icon {
		color: var(--color-accent);
	}

	/* ── Fill in blank ── */

	.fill-sentence {
		line-height: var(--line-height-2-2);
	}

	.input-wrapper {
		display: inline-flex;
		align-items: center;
		position: relative;
		background: white;
		border: 1.5px solid var(--color-bg-soft);
		border-radius: var(--radius-1-5);
		padding: var(--space-1) var(--space-2);
		transition: border-color 0.15s ease;
	}

	.input-wrapper:focus-within {
		border-color: var(--color-accent);
	}

	.input-wrapper.correct {
		border-color: var(--color-success);
		background: var(--color-success-tint);
	}

	.input-wrapper.incorrect {
		border-color: var(--color-accent);
		background: var(--color-accent-tint);
	}

	.fill-input,
	.fill-select {
		padding: var(--space-0) var(--space-0-5);
		border: none;
		background: transparent;
		font-family: inherit;
		font-size: inherit;
		color: var(--color-text);
		min-width: 5em;
	}

	.fill-input {
		text-align: center;
	}

	.fill-select {
		cursor: pointer;
	}

	.fill-input:focus,
	.fill-select:focus {
		outline: none;
	}

	.fill-input::placeholder {
		color: var(--color-text-subtle);
		opacity: 0.5;
	}

	.input-wrapper.correct .fill-input,
	.input-wrapper.correct .fill-select {
		color: var(--color-success);
	}

	.input-wrapper.incorrect .fill-input,
	.input-wrapper.incorrect .fill-select {
		color: var(--color-accent);
	}

	.input-icon {
		font-weight: 700;
		font-size: var(--font-size-2);
		margin-left: var(--space-1);
	}

	.input-wrapper.correct .input-icon {
		color: var(--color-success);
	}

	.input-wrapper.incorrect .input-icon {
		color: var(--color-accent);
	}

	.hint {
		font-size: var(--font-size-2);
		color: var(--color-text-subtle);
		margin: var(--space-2) 0 0;
	}

	/* ── Buttons ── */
	.actions {
		margin-top: var(--space-3);
		display: flex;
		gap: var(--space-2);
	}

	.btn {
		padding: var(--space-2) var(--space-5);
		border: 1.5px solid var(--color-bg-soft);
		border-radius: var(--radius-pill);
		background: transparent;
		font-family: inherit;
		font-size: var(--font-size-2);
		font-weight: 600;
		color: var(--color-text-subtle);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn:hover {
		border-color: var(--color-text);
		color: var(--color-text);
	}

	.btn.primary {
		border-color: var(--color-accent);
		background: var(--color-accent);
		color: var(--color-bg);
	}

	.btn.primary:hover {
		background: var(--color-accent-strong);
		border-color: var(--color-accent-strong);
	}

	/* ── Short/Long answer ── */
	.answer-input {
		width: 100%;
		min-height: 5em;
		padding: var(--space-3);
		border: 1.5px solid var(--color-bg-soft);
		border-radius: var(--radius-2);
		background: transparent;
		font-family: inherit;
		font-size: var(--font-size-2-5);
		color: var(--color-text);
		resize: none;
		overflow: hidden;
		transition: border-color 0.15s ease;
	}

	.answer-input:focus {
		outline: none;
		border-color: var(--color-accent);
	}

	.answer-input.large {
		min-height: 8em;
	}

	.answer-input::placeholder {
		color: var(--color-text-subtle);
		opacity: 0.6;
	}

	.word-hint {
		font-size: var(--font-size-2);
		color: var(--color-text-subtle);
		margin: 0 0 var(--space-2);
	}

	/* ── Model answer ── */
	.model-answer {
		margin-top: var(--space-3);
		padding: var(--space-3) var(--space-4);
		background: var(--color-success-tint);
		border-radius: var(--radius-2);
		color: var(--color-text);
		line-height: var(--line-height-3);
	}

	.model-label {
		font-weight: 600;
		color: var(--color-success);
		margin-right: var(--space-1);
	}

	/* Tamil IME dropdown - horizontal strip above keyboard */
	:global(.tamil-ime-dropdown) {
		position: fixed;
		z-index: 9999;
		background: #f5f5f5;
		border-top: 1px solid #e0e0e0;
		display: none;
		flex-direction: row;
		overflow-x: auto;
		overflow-y: hidden;
		height: 48px;
		font-family: var(--font-tamil, 'Mukta Malar', sans-serif);
		-webkit-user-select: none;
		user-select: none;
		-webkit-overflow-scrolling: touch;
	}

	:global(.tamil-ime-option) {
		flex: 1;
		padding: 12px 16px;
		cursor: pointer;
		font-size: 1.1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f5f5f5;
		color: #333333;
		border-right: 1px solid #e0e0e0;
		-webkit-tap-highlight-color: transparent;
		-webkit-touch-callout: none;
		-webkit-user-select: none;
		user-select: none;
		touch-action: manipulation;
		white-space: nowrap;
	}

	:global(.tamil-ime-option:last-child) {
		border-right: none;
	}

	:global(.tamil-ime-option.selected) {
		background: var(--color-accent, #8b2500);
		color: #ffffff;
	}

	:global(.tamil-ime-option:active) {
		background: #e8e8e8;
	}

	:global(.tamil-ime-option.selected:active) {
		background: var(--color-accent-strong, #6b1d00);
	}

	:global(.tamil-ime-key) {
		display: none;
	}

	:global(.tamil-ime-loading) {
		flex: 1;
		padding: 12px 16px;
		color: #888888;
		font-size: 0.9rem;
		text-align: center;
		background: #f5f5f5;
	}
</style>
