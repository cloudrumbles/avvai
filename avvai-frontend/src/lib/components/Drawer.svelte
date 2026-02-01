<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		open: boolean;
		title: string;
		onclose: () => void;
		position?: 'bottom' | 'left' | 'right';
		children?: Snippet;
	}

	let { open, title, onclose, position = 'bottom', children }: Props = $props();
</script>

{#if open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="drawer-backdrop" onclick={onclose}></div>
{/if}

<div class="drawer {position}" class:open>
	<div class="drawer-header">
		<h2 class="drawer-title">{title}</h2>
		<button class="drawer-close" onclick={onclose} aria-label="Close {title}">
			<svg width="20" height="20" viewBox="0 0 20 20" fill="none">
				<path d="M6 6L14 14M14 6L6 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
			</svg>
		</button>
	</div>
	<div class="drawer-content">
		{@render children?.()}
	</div>
</div>

<style>
	.drawer-backdrop {
		position: fixed;
		inset: 0;
		background: var(--overlay-ink-strong);
		z-index: 150;
	}

	@media (min-width: 769px) {
		.drawer-backdrop {
			display: none;
		}
	}

	.drawer {
		position: fixed;
		background: var(--color-bg);
		z-index: 200;
		display: flex;
		flex-direction: column;
		transition: transform var(--duration-slow) var(--ease-standard);
	}

	/* Bottom drawer (default) */
	.drawer.bottom {
		bottom: 0;
		left: 0;
		right: 0;
		height: 85vh;
		max-height: 85vh;
		border-radius: var(--radius-4) var(--radius-4) 0 0;
		box-shadow: var(--shadow-drawer-bottom);
		transform: translateY(100%);
	}

	.drawer.bottom.open {
		transform: translateY(0);
	}

	/* Left drawer */
	.drawer.left {
		top: 0;
		bottom: 0;
		left: 0;
		width: 280px;
		box-shadow: var(--shadow-drawer-side);
		transform: translateX(-100%);
	}

	.drawer.left.open {
		transform: translateX(0);
	}

	/* Right drawer */
	.drawer.right {
		top: 0;
		bottom: 0;
		right: 0;
		width: 400px;
		box-shadow: var(--shadow-drawer-side-right);
		transform: translateX(100%);
	}

	.drawer.right.open {
		transform: translateX(0);
	}

	/* Desktop adjustments for bottom drawer */
	@media (min-width: 769px) {
		.drawer.bottom {
			left: auto;
			right: var(--space-6);
			bottom: var(--space-6);
			width: 400px;
			height: auto;
			max-height: 80vh;
			border-radius: var(--radius-4);
			transform: translateX(120%);
			box-shadow: var(--shadow-drawer-desktop);
		}

		.drawer.bottom.open {
			transform: translateX(0);
		}
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-5);
		border-bottom: var(--border-1);
		flex-shrink: 0;
	}

	.drawer-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-4);
		font-weight: 700;
		color: var(--color-accent);
		margin: 0;
	}

	.drawer.left .drawer-title {
		color: var(--color-accent-strong);
	}

	.drawer-close {
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
	}

	.drawer-close:hover {
		background: var(--color-accent-tint);
		color: var(--color-accent);
	}

	.drawer-content {
		flex: 1;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
	}

	.drawer.bottom .drawer-content,
	.drawer.right .drawer-content {
		padding: var(--space-5);
	}
</style>
