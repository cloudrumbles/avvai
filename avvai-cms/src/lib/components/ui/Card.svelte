<script lang="ts">
	interface Props {
		title?: string;
		padding?: 'none' | 'sm' | 'md' | 'lg';
		class?: string;
		children?: import('svelte').Snippet;
		actions?: import('svelte').Snippet;
	}

	let {
		title,
		padding = 'md',
		class: className = '',
		children,
		actions
	}: Props = $props();
</script>

<div class="card padding-{padding} {className}">
	{#if title || actions}
		<div class="card-header">
			{#if title}
				<h3 class="card-title">{title}</h3>
			{/if}
			{#if actions}
				<div class="card-actions">
					{@render actions()}
				</div>
			{/if}
		</div>
	{/if}

	<div class="card-content">
		{@render children?.()}
	</div>
</div>

<style>
	.card {
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.08);
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-sm);
		overflow: hidden;
		transition: all var(--transition-base);
		position: relative;
	}

	.card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		padding: 1px;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.8) 0%,
			rgba(250, 246, 240, 0.4) 100%
		);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask-composite: exclude;
		pointer-events: none;
	}

	.card:hover {
		box-shadow: var(--shadow-md);
		transform: translateY(-2px);
		border-color: rgba(92, 74, 61, 0.12);
	}

	/* Padding variants */
	.padding-none .card-content {
		padding: 0;
	}

	.padding-sm .card-content {
		padding: var(--space-3);
	}

	.padding-md .card-content {
		padding: var(--space-5);
	}

	.padding-lg .card-content {
		padding: var(--space-6);
	}

	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-5);
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
		background: linear-gradient(180deg, var(--c-cream) 0%, rgba(250, 246, 240, 0.3) 100%);
	}

	.card-title {
		margin: 0;
		font-family: var(--font-heading);
		font-size: 1.05rem;
		font-weight: 600;
		color: var(--c-brown-dark);
	}

	.card-actions {
		display: flex;
		gap: var(--space-2);
	}

	.card-content {
		background: rgba(255, 255, 255, 0.5);
	}
</style>
