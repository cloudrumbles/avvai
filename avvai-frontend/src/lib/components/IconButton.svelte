<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		onclick?: (event: MouseEvent) => void;
		label: string;
		disabled?: boolean;
		expanded?: boolean;
		children: Snippet;
	}

	let { onclick, label, disabled = false, expanded, children }: Props = $props();
</script>

<button
	class="icon-btn"
	{onclick}
	{disabled}
	aria-label={label}
	aria-expanded={expanded}
>
	{@render children()}
</button>

<style>
	.icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: none;
		border-radius: 8px;
		background: transparent;
		color: var(--stone);
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.icon-btn:hover:not(:disabled) {
		background: var(--red-faint);
		color: var(--red);
	}

	.icon-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
</style>
