<script lang="ts">
	import { slide } from 'svelte/transition';
	import {
		ChevronIcon,
		DragHandleIcon,
		DuplicateIcon,
		TrashIcon,
		CheckCircleIcon,
		WarningIcon,
		MultipleChoiceIcon,
		FillInBlankIcon,
		ShortAnswerIcon,
		LongAnswerIcon
	} from '$lib/components/icons';

	type ExerciseType = 'multiple_choice' | 'fill_in_blank' | 'short_answer' | 'long_answer';

	interface Props {
		exerciseType: ExerciseType;
		title: string;
		isValid: boolean;
		collapsed?: boolean;
		onDelete?: () => void;
		onDuplicate?: () => void;
		children: import('svelte').Snippet;
	}

	let {
		exerciseType,
		title,
		isValid,
		collapsed = $bindable(false),
		onDelete,
		onDuplicate,
		children
	}: Props = $props();

	// Color mapping for exercise types using the new palette
	const typeColors: Record<ExerciseType, string> = {
		multiple_choice: '#2B6CB0',
		fill_in_blank: '#805AD5',
		short_answer: '#2F855A',
		long_answer: '#C99A2E'
	};

	// Label mapping for exercise types
	const typeLabels: Record<ExerciseType, string> = {
		multiple_choice: 'Multiple Choice',
		fill_in_blank: 'Fill in Blank',
		short_answer: 'Short Answer',
		long_answer: 'Long Answer'
	};

	let accentColor = $derived(typeColors[exerciseType]);
	let typeLabel = $derived(typeLabels[exerciseType]);

	function toggleCollapse() {
		collapsed = !collapsed;
	}
</script>

<div class="exercise-card" style="--accent-color: {accentColor}" class:collapsed>
	<div class="accent-bar"></div>

	<div class="card-content">
		<div class="card-header">
			<div class="header-left">
				<div class="drag-handle" title="Drag to reorder">
					<DragHandleIcon size={16} color="var(--c-brown-light)" />
				</div>

				<button
					class="toggle-btn"
					onclick={toggleCollapse}
					aria-label={collapsed ? 'Expand' : 'Collapse'}
				>
					<ChevronIcon expanded={!collapsed} size={14} color="var(--c-brown-light)" />
				</button>

				<div class="title-group">
					<div class="type-icon" style="color: {accentColor}">
						{#if exerciseType === 'multiple_choice'}
							<MultipleChoiceIcon size={18} color={accentColor} />
						{:else if exerciseType === 'fill_in_blank'}
							<FillInBlankIcon size={18} color={accentColor} />
						{:else if exerciseType === 'short_answer'}
							<ShortAnswerIcon size={18} color={accentColor} />
						{:else}
							<LongAnswerIcon size={18} color={accentColor} />
						{/if}
					</div>
					<span class="card-title">{title || typeLabel}</span>
				</div>
			</div>

			<div class="header-right">
				<div class="validation-badge" class:valid={isValid} class:invalid={!isValid}>
					<div class="status-icon">
						{#if isValid}
							<div class="pulse-ring"></div>
							<CheckCircleIcon size={14} color="var(--c-sage)" />
						{:else}
							<WarningIcon size={14} color="var(--c-warning)" />
						{/if}
					</div>
					<span class="badge-text">{isValid ? 'Complete' : 'Draft'}</span>
				</div>

				<div class="actions">
					<button class="action-btn duplicate" onclick={onDuplicate} title="Duplicate Exercise">
						<DuplicateIcon size={14} color="var(--c-brown)" />
					</button>
					<button class="action-btn delete" onclick={onDelete} title="Delete Exercise">
						<TrashIcon size={14} color="var(--c-terracotta)" />
					</button>
				</div>
			</div>
		</div>

		{#if !collapsed}
			<div class="card-body" transition:slide={{ duration: 200 }}>
				{@render children()}
			</div>
		{/if}
	</div>
</div>

<style>
	.exercise-card {
		display: flex;
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.1);
		border-radius: var(--radius-md);
		overflow: hidden;
		transition: all var(--transition-base);
		box-shadow: var(--shadow-sm);
	}

	.exercise-card:hover {
		box-shadow: var(--shadow-md);
		transform: translateY(-2px);
		border-color: rgba(92, 74, 61, 0.15);
	}

	.exercise-card.collapsed {
		background: var(--c-cream-warm);
	}

	.exercise-card.collapsed:hover {
		transform: translateY(-1px);
	}

	.accent-bar {
		width: 5px;
		background: linear-gradient(
			180deg,
			var(--accent-color) 0%,
			color-mix(in srgb, var(--accent-color) 60%, var(--c-gold)) 100%
		);
		flex-shrink: 0;
		position: relative;
	}

	.accent-bar::after {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			90deg,
			rgba(255, 255, 255, 0.2) 0%,
			transparent 50%,
			rgba(0, 0, 0, 0.05) 100%
		);
	}

	.card-content {
		flex: 1;
		min-width: 0;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--space-3) var(--space-4);
		background: linear-gradient(180deg, var(--c-cream) 0%, rgba(250, 246, 240, 0.5) 100%);
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
		user-select: none;
	}

	.exercise-card.collapsed .card-header {
		border-bottom: none;
		background: transparent;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex: 1;
		min-width: 0;
	}

	.drag-handle {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		cursor: grab;
		border-radius: var(--radius-sm);
		transition: all var(--transition-fast);
	}

	.drag-handle:hover {
		background: rgba(92, 74, 61, 0.08);
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	.toggle-btn {
		background: rgba(92, 74, 61, 0.06);
		border: none;
		cursor: pointer;
		padding: var(--space-1);
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-sm);
		transition: all var(--transition-fast);
	}

	.toggle-btn:hover {
		background: rgba(92, 74, 61, 0.12);
	}

	.title-group {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		min-width: 0;
	}

	.type-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: color-mix(in srgb, var(--accent-color) 10%, white);
		border-radius: var(--radius-sm);
		flex-shrink: 0;
	}

	.card-title {
		font-weight: 600;
		color: var(--c-brown-dark);
		font-size: 0.9rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex-shrink: 0;
	}

	.validation-badge {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
		padding: 0.25rem 0.5rem;
		border-radius: var(--radius-full);
		transition: all var(--transition-base);
	}

	.validation-badge.valid {
		background: var(--c-success-bg);
		color: var(--c-sage);
		border: 1px solid rgba(47, 133, 90, 0.15);
	}

	.validation-badge.invalid {
		background: var(--c-warning-bg);
		color: var(--c-warning);
		border: 1px solid rgba(201, 154, 46, 0.15);
	}

	.status-icon {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.pulse-ring {
		position: absolute;
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--c-sage);
		opacity: 0;
		animation: pulse-ring 2s ease-out infinite;
	}

	@keyframes pulse-ring {
		0% {
			transform: scale(0.5);
			opacity: 0.5;
		}
		100% {
			transform: scale(1.5);
			opacity: 0;
		}
	}

	.badge-text {
		font-size: 0.65rem;
	}

	.actions {
		display: flex;
		gap: var(--space-1);
		padding-left: var(--space-2);
		border-left: 1px solid rgba(92, 74, 61, 0.1);
	}

	.action-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid rgba(92, 74, 61, 0.15);
		border-radius: var(--radius-sm);
		background: white;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.action-btn:hover {
		background: var(--c-cream-dark);
		border-color: rgba(92, 74, 61, 0.25);
		transform: translateY(-1px);
		box-shadow: var(--shadow-sm);
	}

	.action-btn:active {
		transform: translateY(0);
	}

	.action-btn.delete:hover {
		background: var(--c-exercises-bg);
		border-color: rgba(184, 83, 61, 0.3);
	}

	.card-body {
		padding: var(--space-5);
		background: rgba(255, 255, 255, 0.6);
	}
</style>
