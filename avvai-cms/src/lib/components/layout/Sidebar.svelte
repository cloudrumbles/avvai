<script lang="ts">
	import Nav from '$lib/components/layout/Nav.svelte';
	import type { SidebarOverride } from '$lib/layoutContext';

	interface Props {
		override?: SidebarOverride | null;
	}

	let { override = null }: Props = $props();

	const resolvedStructureTitle = $derived(override?.structureTitle ?? 'Structure');
	const resolvedStructureCount = $derived(override?.structureCount);
	const resolvedStructureComponent = $derived(override?.structureComponent);
	const resolvedStructureProps = $derived(override?.structureProps ?? {});
</script>

<aside class="sidebar">
	<div class="panel-section nav-section">
		<Nav />
	</div>

	{#if resolvedStructureComponent}
		{@const Structure = resolvedStructureComponent}
		<div class="panel-section structure-section">
			<div class="section-header">
				<span class="section-title">{resolvedStructureTitle}</span>
				{#if typeof resolvedStructureCount === 'number'}
					<span class="section-count">{resolvedStructureCount}</span>
				{/if}
			</div>

			<div class="structure-content">
				<Structure {...resolvedStructureProps} />
			</div>
		</div>
	{/if}
</aside>

<style>
	.sidebar {
		width: 260px;
		background: var(--c-cream-dark);
		border-right: 1px solid rgba(92, 74, 61, 0.1);
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
		height: 100vh;
		overflow: hidden;
	}

	.panel-section {
		padding: var(--space-4);
	}

	.nav-section {
		border-bottom: 1px solid rgba(92, 74, 61, 0.1);
	}

	.structure-section {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-height: 0;
		overflow: hidden;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-3);
	}

	.section-title {
		font-family: var(--font-heading);
		font-size: 0.7rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--c-brown-muted);
	}

	.section-count {
		font-size: 0.7rem;
		font-weight: 600;
		color: var(--c-brown-muted);
		background: rgba(92, 74, 61, 0.08);
		padding: 0.1rem 0.4rem;
		border-radius: 8px;
	}

	.structure-content {
		flex: 1;
		overflow-y: auto;
		margin: 0 calc(-1 * var(--space-2));
		padding: 0 var(--space-2);
	}
</style>
