<script lang="ts">
	interface Option {
		label: string;
		value: string;
	}

	interface Props {
		label?: string;
		value?: string;
		options?: Option[];
		placeholder?: string;
		error?: string;
		disabled?: boolean;
		class?: string;
		onchange?: (e: Event & { currentTarget: HTMLSelectElement }) => void;
	}

	const selectId = $props.id();

	let {
		label,
		value = $bindable(''),
		options = [],
		placeholder = 'Select an option',
		error,
		disabled = false,
		class: className = '',
		onchange
	}: Props = $props();
</script>

<div class="select-group {className}">
	{#if label}
		<label class="label" for={selectId}>
			{label}
		</label>
	{/if}

	<div class="select-wrapper">
		<select
			id={selectId}
			class="select {error ? 'has-error' : ''}"
			bind:value
			{disabled}
			{onchange}
		>
			{#if placeholder}
				<option value="" disabled>{placeholder}</option>
			{/if}
			{#each options as option (option.value)}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
		<span class="chevron">▾</span>
	</div>

	{#if error}
		<span class="error-text">{error}</span>
	{/if}
</div>

<style>
	.select-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		width: 100%;
	}

	.label {
		font-family: var(--font-heading);
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--c-brown);
	}

	.select-wrapper {
		position: relative;
	}

	.select {
		width: 100%;
		appearance: none;
		padding: 0.75rem 2.5rem 0.75rem 1rem;
		border: 1px solid rgba(92, 74, 61, 0.2);
		border-radius: 8px;
		background-color: white;
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-ink);
		transition: all 0.2s ease;
	}

	.select:focus {
		outline: none;
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(196, 92, 62, 0.1);
	}

	.select:disabled {
		background-color: rgba(0, 0, 0, 0.05);
		cursor: not-allowed;
		color: var(--c-brown-light);
	}

	.select.has-error {
		border-color: #e53e3e;
	}

	.chevron {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
		color: var(--c-brown-light);
		font-size: 0.8rem;
		pointer-events: none;
	}

	.error-text {
		font-family: var(--font-serif);
		font-size: 0.85rem;
		color: #e53e3e;
	}
</style>
