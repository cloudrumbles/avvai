<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		open: boolean;
		title: string;
		onclose: () => void;
		position?: 'bottom' | 'left' | 'right';
		children: Snippet;
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
		{@render children()}
	</div>
</div>

<style>
	.drawer-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(26, 14, 6, 0.4);
		z-index: 150;
	}

	@media (min-width: 769px) {
		.drawer-backdrop {
			display: none;
		}
	}

	.drawer {
		position: fixed;
		background: var(--cream);
		z-index: 200;
		display: flex;
		flex-direction: column;
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	/* Bottom drawer (default) */
	.drawer.bottom {
		bottom: 0;
		left: 0;
		right: 0;
		height: 85vh;
		max-height: 85vh;
		border-radius: 16px 16px 0 0;
		box-shadow: 0 -4px 24px rgba(26, 14, 6, 0.15);
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
		box-shadow: 4px 0 24px rgba(26, 14, 6, 0.15);
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
		box-shadow: -4px 0 24px rgba(26, 14, 6, 0.15);
		transform: translateX(100%);
	}

	.drawer.right.open {
		transform: translateX(0);
	}

	/* Desktop adjustments for bottom drawer */
	@media (min-width: 769px) {
		.drawer.bottom {
			left: auto;
			right: 24px;
			bottom: 24px;
			width: 400px;
			height: auto;
			max-height: 80vh;
			border-radius: 16px;
			transform: translateX(120%);
			box-shadow: 0 4px 24px rgba(26, 14, 6, 0.2);
		}

		.drawer.bottom.open {
			transform: translateX(0);
		}
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid var(--cream-mid);
		flex-shrink: 0;
	}

	.drawer-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 18px;
		font-weight: 700;
		color: var(--red);
		margin: 0;
	}

	.drawer.left .drawer-title {
		color: var(--red-deep);
	}

	.drawer-close {
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
	}

	.drawer-close:hover {
		background: var(--red-faint);
		color: var(--red);
	}

	.drawer-content {
		flex: 1;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
	}

	.drawer.bottom .drawer-content,
	.drawer.right .drawer-content {
		padding: 20px;
	}
</style>
