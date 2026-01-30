<script lang="ts">
	interface Props {
		variant?: 'neutral' | 'info' | 'success' | 'warning' | 'error' | 'terracotta' | 'sage' | 'gold';
		children?: import('svelte').Snippet;
	}

	let { variant = 'neutral', children }: Props = $props();
</script>

<span class="badge variant-{variant}">
	{@render children?.()}
</span>

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		padding: 0.3rem 0.75rem;
		border-radius: var(--radius-full);
		font-family: var(--font-heading);
		font-size: 0.7rem;
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: uppercase;
		white-space: nowrap;
		border: 1px solid transparent;
		transition: all var(--transition-fast);
	}

	.variant-neutral {
		background: rgba(92, 74, 61, 0.08);
		color: var(--c-brown);
		border-color: rgba(92, 74, 61, 0.12);
	}

	.variant-info {
		background: var(--c-info-bg);
		color: var(--c-info);
		border-color: rgba(43, 108, 176, 0.15);
	}

	.variant-success {
		background: var(--c-success-bg);
		color: var(--c-success);
		border-color: rgba(47, 133, 90, 0.15);
	}

	.variant-warning {
		background: var(--c-warning-bg);
		color: var(--c-warning);
		border-color: rgba(201, 154, 46, 0.15);
	}

	.variant-error {
		background: var(--c-error-bg);
		color: var(--c-error);
		border-color: rgba(184, 83, 61, 0.15);
	}

	/* Editorial palette variants */
	.variant-terracotta {
		background: var(--c-exercises-bg);
		color: var(--c-terracotta);
		border-color: rgba(184, 83, 61, 0.2);
	}

	.variant-sage {
		background: rgba(95, 122, 97, 0.1);
		color: var(--c-sage);
		border-color: rgba(95, 122, 97, 0.2);
	}

	.variant-gold {
		background: rgba(201, 154, 46, 0.1);
		color: var(--c-gold);
		border-color: rgba(201, 154, 46, 0.2);
	}
</style>
