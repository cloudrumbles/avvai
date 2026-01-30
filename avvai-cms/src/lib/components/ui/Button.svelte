<script lang="ts">
	interface Props {
		type?: 'button' | 'submit' | 'reset';
		variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'sage';
		size?: 'sm' | 'md' | 'lg';
		disabled?: boolean;
		fullWidth?: boolean;
		class?: string;
		children?: import('svelte').Snippet;
		onclick?: (e: MouseEvent) => void;
	}

	let {
		type = 'button',
		variant = 'primary',
		size = 'md',
		disabled = false,
		fullWidth = false,
		class: className = '',
		children,
		onclick
	}: Props = $props();
</script>

<button
	{type}
	class="btn variant-{variant} size-{size} {fullWidth ? 'full-width' : ''} {className}"
	{disabled}
	{onclick}
>
	{@render children?.()}
</button>

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		font-family: var(--font-heading);
		font-weight: 600;
		letter-spacing: 0.02em;
		transition: all var(--transition-base);
		gap: var(--space-2);
		position: relative;
		overflow: hidden;
	}

	.btn::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, transparent 50%);
		opacity: 0;
		transition: opacity var(--transition-fast);
	}

	.btn:hover::before {
		opacity: 1;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none !important;
	}

	.btn:disabled::before {
		display: none;
	}

	.full-width {
		width: 100%;
	}

	/* Sizes */
	.size-sm {
		padding: var(--space-2) var(--space-3);
		font-size: 0.8rem;
	}

	.size-md {
		padding: 0.65rem var(--space-5);
		font-size: 0.9rem;
	}

	.size-lg {
		padding: var(--space-3) var(--space-6);
		font-size: 1rem;
	}

	/* Primary - Terracotta */
	.variant-primary {
		background: linear-gradient(135deg, var(--c-terracotta) 0%, var(--c-terracotta-dark) 100%);
		color: white;
		box-shadow:
			0 1px 2px rgba(184, 83, 61, 0.2),
			0 2px 4px rgba(184, 83, 61, 0.1);
	}

	.variant-primary:hover:not(:disabled) {
		background: linear-gradient(135deg, var(--c-terracotta-light) 0%, var(--c-terracotta) 100%);
		transform: translateY(-2px);
		box-shadow:
			0 4px 12px rgba(184, 83, 61, 0.25),
			0 2px 4px rgba(184, 83, 61, 0.15);
	}

	.variant-primary:active:not(:disabled) {
		transform: translateY(0);
		box-shadow:
			0 1px 2px rgba(184, 83, 61, 0.2),
			0 2px 4px rgba(184, 83, 61, 0.1);
	}

	/* Secondary - Brown */
	.variant-secondary {
		background: linear-gradient(135deg, var(--c-brown) 0%, var(--c-brown-dark) 100%);
		color: var(--c-cream);
		box-shadow:
			0 1px 2px rgba(61, 43, 31, 0.15),
			0 2px 4px rgba(61, 43, 31, 0.08);
	}

	.variant-secondary:hover:not(:disabled) {
		background: linear-gradient(135deg, var(--c-brown-light) 0%, var(--c-brown) 100%);
		transform: translateY(-2px);
		box-shadow:
			0 4px 12px rgba(61, 43, 31, 0.2),
			0 2px 4px rgba(61, 43, 31, 0.1);
	}

	.variant-secondary:active:not(:disabled) {
		transform: translateY(0);
	}

	/* Sage - Secondary Action */
	.variant-sage {
		background: linear-gradient(135deg, var(--c-sage) 0%, var(--c-sage-dark) 100%);
		color: white;
		box-shadow:
			0 1px 2px rgba(95, 122, 97, 0.2),
			0 2px 4px rgba(95, 122, 97, 0.1);
	}

	.variant-sage:hover:not(:disabled) {
		background: linear-gradient(135deg, var(--c-sage-light) 0%, var(--c-sage) 100%);
		transform: translateY(-2px);
		box-shadow:
			0 4px 12px rgba(95, 122, 97, 0.25),
			0 2px 4px rgba(95, 122, 97, 0.15);
	}

	.variant-sage:active:not(:disabled) {
		transform: translateY(0);
	}

	/* Outline */
	.variant-outline {
		background: transparent;
		border: 1.5px solid var(--c-brown);
		color: var(--c-brown);
	}

	.variant-outline:hover:not(:disabled) {
		background: var(--c-brown);
		color: var(--c-cream);
		transform: translateY(-2px);
	}

	.variant-outline:active:not(:disabled) {
		transform: translateY(0);
	}

	/* Ghost */
	.variant-ghost {
		background: transparent;
		color: var(--c-brown);
	}

	.variant-ghost:hover:not(:disabled) {
		background: rgba(92, 74, 61, 0.08);
		transform: translateY(-1px);
	}

	.variant-ghost:active:not(:disabled) {
		transform: translateY(0);
	}
</style>
