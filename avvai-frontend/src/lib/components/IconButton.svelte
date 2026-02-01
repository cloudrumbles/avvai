<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		onclick?: (event: MouseEvent) => void;
		label: string;
		disabled?: boolean;
		expanded?: boolean;
		children?: Snippet;
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
	{@render children?.()}
</button>

<style>
	.icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--size-icon-btn);
		height: var(--size-icon-btn);
		border: none;
		border-radius: var(--radius-2);
		background: transparent;
		color: var(--color-text-subtle);
		cursor: pointer;
		transition: transform var(--duration-fast) var(--ease-standard),
			background var(--duration-fast) var(--ease-standard),
			color var(--duration-fast) var(--ease-standard);
		-webkit-tap-highlight-color: transparent;
	}

	.icon-btn:hover:not(:disabled) {
		background: var(--color-accent-tint);
		color: var(--color-accent);
	}

	.icon-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
</style>
