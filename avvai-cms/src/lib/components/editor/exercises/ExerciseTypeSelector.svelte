<script lang="ts">
	import {
		MultipleChoiceIcon,
		FillInBlankIcon,
		ShortAnswerIcon,
		LongAnswerIcon
	} from '$lib/components/icons';

	type ExerciseType = 'multiple_choice' | 'fill_in_blank' | 'short_answer' | 'long_answer';

	interface Props {
		onSelect: (type: ExerciseType) => void;
	}

	let { onSelect }: Props = $props();

	interface TypeOption {
		type: ExerciseType;
		label: string;
		description: string;
		color: string;
		bgColor: string;
	}

	const typeOptions: TypeOption[] = [
		{
			type: 'multiple_choice',
			label: 'Multiple Choice',
			description: 'Choose from options',
			color: '#2B6CB0',
			bgColor: '#EBF8FF'
		},
		{
			type: 'fill_in_blank',
			label: 'Fill in Blank',
			description: 'Complete the sentence',
			color: '#805AD5',
			bgColor: '#FAF5FF'
		},
		{
			type: 'short_answer',
			label: 'Short Answer',
			description: 'Brief written response',
			color: '#2F855A',
			bgColor: '#F0FFF4'
		},
		{
			type: 'long_answer',
			label: 'Long Answer',
			description: 'Extended written response',
			color: '#C99A2E',
			bgColor: '#FFF8E1'
		}
	];

	function handleSelect(type: ExerciseType) {
		onSelect(type);
	}

	function handleKeydown(event: KeyboardEvent, type: ExerciseType) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			onSelect(type);
		}
	}
</script>

<div class="type-selector-grid" role="group" aria-label="Select exercise type">
	{#each typeOptions as option (option.type)}
		<button
			class="type-button"
			style="--accent-color: {option.color}; --accent-bg: {option.bgColor}"
			onclick={() => handleSelect(option.type)}
			onkeydown={(e) => handleKeydown(e, option.type)}
			type="button"
		>
			<span class="icon">
				{#if option.type === 'multiple_choice'}
					<MultipleChoiceIcon size={24} color={option.color} />
				{:else if option.type === 'fill_in_blank'}
					<FillInBlankIcon size={24} color={option.color} />
				{:else if option.type === 'short_answer'}
					<ShortAnswerIcon size={24} color={option.color} />
				{:else}
					<LongAnswerIcon size={24} color={option.color} />
				{/if}
			</span>
			<span class="label">{option.label}</span>
			<span class="description">{option.description}</span>
		</button>
	{/each}
</div>

<style>
	.type-selector-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-3);
	}

	.type-button {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		padding: var(--space-5) var(--space-4);
		background: linear-gradient(135deg, var(--accent-bg) 0%, white 100%);
		border: 1px solid color-mix(in srgb, var(--accent-color) 20%, transparent);
		border-radius: var(--radius-md);
		cursor: pointer;
		transition: all var(--transition-base);
		position: relative;
		overflow: hidden;
	}

	.type-button::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			transparent 0%,
			color-mix(in srgb, var(--accent-color) 5%, transparent) 100%
		);
		opacity: 0;
		transition: opacity var(--transition-base);
	}

	.type-button:hover {
		transform: translateY(-3px);
		box-shadow:
			0 8px 20px color-mix(in srgb, var(--accent-color) 15%, transparent),
			0 4px 8px rgba(0, 0, 0, 0.04);
		border-color: color-mix(in srgb, var(--accent-color) 40%, transparent);
	}

	.type-button:hover::before {
		opacity: 1;
	}

	.type-button:focus {
		outline: none;
		box-shadow:
			0 0 0 2px white,
			0 0 0 4px var(--accent-color);
	}

	.type-button:focus:not(:focus-visible) {
		box-shadow: none;
	}

	.type-button:focus-visible {
		box-shadow:
			0 0 0 2px white,
			0 0 0 4px var(--accent-color);
	}

	.type-button:active {
		transform: translateY(-1px);
	}

	.icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: white;
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-sm);
		position: relative;
		z-index: 1;
	}

	.label {
		font-family: var(--font-heading);
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		position: relative;
		z-index: 1;
	}

	.description {
		font-size: 0.7rem;
		color: var(--c-brown-muted);
		opacity: 0;
		transform: translateY(-4px);
		transition: all var(--transition-base);
		position: relative;
		z-index: 1;
	}

	.type-button:hover .description,
	.type-button:focus .description {
		opacity: 1;
		transform: translateY(0);
	}
</style>
