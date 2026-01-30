<script lang="ts">
	import KolamPattern from './KolamPattern.svelte';

	interface Props {
		label?: string;
		value?: string;
		type?: string;
		placeholder?: string;
		error?: string;
		disabled?: boolean;
		multiline?: boolean;
		rows?: number;
		class?: string;
		oninput?: (e: Event & { currentTarget: HTMLInputElement | HTMLTextAreaElement }) => void;
	}

	let {
		label,
		value = $bindable(''),
		type = 'text',
		placeholder = '',
		error,
		disabled = false,
		multiline = false,
		rows = 4,
		class: className = '',
		oninput
	}: Props = $props();
</script>

<div class="input-group {className}" class:has-error={error}>
	{#if label}
		<label class="label">
			{label}
		</label>
	{/if}

	<div class="input-wrapper">
		{#if multiline}
			<textarea
				class="input"
				bind:value
				{placeholder}
				{disabled}
				{rows}
				{oninput}
			></textarea>
		{:else}
			<input
				class="input"
				{type}
				bind:value
				{placeholder}
				{disabled}
				{oninput}
			/>
		{/if}
		<div class="focus-accent"></div>
		<div class="corner-accent">
			<KolamPattern variant="corner" size="sm" color="terracotta" />
		</div>
	</div>

	{#if error}
		<span class="error-text">{error}</span>
	{/if}
</div>

<style>
	.input-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		width: 100%;
	}

	.label {
		font-family: var(--font-heading);
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown);
		letter-spacing: 0.02em;
	}

	.input-wrapper {
		position: relative;
	}

	.input {
		width: 100%;
		padding: 0.85rem 1rem;
		border: 1px solid rgba(92, 74, 61, 0.18);
		border-radius: var(--radius-sm);
		background: white;
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-dark);
		transition: all var(--transition-base);
	}

	textarea.input {
		resize: vertical;
		min-height: 100px;
	}

	.input::placeholder {
		color: var(--c-brown-muted);
	}

	.input:hover {
		border-color: rgba(92, 74, 61, 0.25);
	}

	.input:focus {
		outline: none;
		border-color: var(--c-terracotta);
	}

	.input:focus ~ .focus-accent {
		opacity: 1;
	}

	.input:disabled {
		background: var(--c-cream-dark);
		cursor: not-allowed;
		opacity: 0.7;
	}

	.focus-accent {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 2px;
		background: linear-gradient(90deg, var(--c-terracotta) 0%, var(--c-gold) 100%);
		border-radius: 0 0 var(--radius-sm) var(--radius-sm);
		opacity: 0;
		transition: opacity var(--transition-base);
		pointer-events: none;
	}

	.corner-accent {
		position: absolute;
		top: var(--space-2);
		right: var(--space-2);
		opacity: 0;
		transition: opacity var(--transition-base);
		pointer-events: none;
	}

	.input:focus ~ .corner-accent {
		opacity: 0.3;
	}

	.input-group.has-error .input {
		border-color: var(--c-terracotta);
		background: var(--c-error-bg);
	}

	.input-group.has-error .focus-accent {
		background: var(--c-terracotta);
		opacity: 1;
	}

	.error-text {
		font-size: 0.8rem;
		color: var(--c-terracotta);
		font-family: var(--font-body);
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}

	.error-text::before {
		content: '•';
		font-weight: bold;
	}
</style>
