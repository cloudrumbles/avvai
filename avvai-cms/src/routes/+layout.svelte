<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import '$lib/styles/theme.css';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import { setLayoutContext } from '$lib/layoutContext';
	import type { SidebarOverride } from '$lib/layoutContext';

	let { children } = $props();

	let override = $state<SidebarOverride | null>(null);

	function setSidebarOverride(next: SidebarOverride) {
		override = next;
	}

	function clearSidebarOverride() {
		override = null;
	}

	setLayoutContext({ setSidebarOverride, clearSidebarOverride });
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>avvai cms</title>
</svelte:head>

<div class="texture-overlay"></div>
<div class="cms-shell">
	<Sidebar {override} />
	<main>
		<div class="content-wrapper" class:full-width={override?.fullWidth}>
			{@render children()}
		</div>
	</main>
</div>

<style>
	.cms-shell {
		display: flex;
		min-height: 100vh;
		width: 100%;
	}

	main {
		flex: 1;
		background-color: var(--c-cream);
		overflow-y: auto;
		position: relative;
		display: flex;
		flex-direction: column;
	}

	.content-wrapper {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2.5rem 3rem 3rem;
		width: 100%;
	}

	.content-wrapper.full-width {
		max-width: none;
		padding: 0;
	}
</style>
