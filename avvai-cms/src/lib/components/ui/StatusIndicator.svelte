<script lang="ts">
	import { CheckCircleIcon, WarningIcon } from '$lib/components/icons';

	interface Props {
		status: 'valid' | 'invalid' | 'loading' | 'neutral';
		size?: 'sm' | 'md' | 'lg';
		showLabel?: boolean;
		label?: string;
	}

	let {
		status,
		size = 'md',
		showLabel = true,
		label
	}: Props = $props();

	const config = {
		valid: {
			icon: CheckCircleIcon,
			color: 'var(--c-sage)',
			bgColor: 'var(--c-success-bg)',
			defaultLabel: 'Complete',
			animate: true
		},
		invalid: {
			icon: WarningIcon,
			color: 'var(--c-warning)',
			bgColor: 'var(--c-warning-bg)',
			defaultLabel: 'Draft',
			animate: false
		},
		loading: {
			icon: null,
			color: 'var(--c-brown-muted)',
			bgColor: 'var(--c-cream-dark)',
			defaultLabel: 'Loading',
			animate: true
		},
		neutral: {
			icon: null,
			color: 'var(--c-brown-muted)',
			bgColor: 'transparent',
			defaultLabel: '',
			animate: false
		}
	};

	let statusConfig = $derived(config[status]);
	let displayLabel = $derived(label || statusConfig.defaultLabel);
	let IconComponent = $derived(statusConfig.icon);
	let shouldAnimate = $derived(status === 'valid' || status === 'loading');
</script>

<div
	class="status-indicator status-{status} size-{size}"
	style="--status-color: {statusConfig.color}; --status-bg: {statusConfig.bgColor}"
>
	<div class="indicator-container">
		{#if status === 'loading'}
			<div class="spinner"></div>
		{:else if status === 'valid'}
			<div class="pulse-ring"></div>
			<div class="icon-wrapper">
				<CheckCircleIcon size={size === 'sm' ? 12 : size === 'lg' ? 20 : 16} color={statusConfig.color} />
			</div>
		{:else if status === 'invalid'}
			<div class="icon-wrapper">
				<WarningIcon size={size === 'sm' ? 12 : size === 'lg' ? 20 : 16} color={statusConfig.color} />
			</div>
		{:else}
			<div class="dot"></div>
		{/if}
	</div>

	{#if showLabel && displayLabel}
		<span class="status-label">{displayLabel}</span>
	{/if}
</div>

<style>
	.status-indicator {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: 0.35rem 0.75rem;
		border-radius: var(--radius-full);
		background: var(--status-bg);
		border: 1px solid color-mix(in srgb, var(--status-color) 15%, transparent);
		transition: all var(--transition-base);
	}

	.size-sm {
		padding: 0.25rem 0.5rem;
	}

	.size-sm .status-label {
		font-size: 0.65rem;
	}

	.size-lg {
		padding: 0.5rem 1rem;
	}

	.size-lg .status-label {
		font-size: 0.85rem;
	}

	.indicator-container {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 16px;
		height: 16px;
	}

	.size-sm .indicator-container {
		width: 14px;
		height: 14px;
	}

	.size-lg .indicator-container {
		width: 20px;
		height: 20px;
	}

	.icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		z-index: 1;
	}

	.pulse-ring {
		position: absolute;
		width: 100%;
		height: 100%;
		border-radius: 50%;
		background: var(--status-color);
		opacity: 0;
		animation: pulse-ring 2s cubic-bezier(0.215, 0.61, 0.355, 1) infinite;
	}

	@keyframes pulse-ring {
		0% {
			transform: scale(0.5);
			opacity: 0.5;
		}
		50% {
			opacity: 0.3;
		}
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}

	.spinner {
		width: 14px;
		height: 14px;
		border: 2px solid color-mix(in srgb, var(--status-color) 30%, transparent);
		border-top-color: var(--status-color);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--status-color);
	}

	.dot.pulsing {
		animation: pulse-dot 1.5s ease-in-out infinite;
	}

	@keyframes pulse-dot {
		0%, 100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.5;
			transform: scale(0.8);
		}
	}

	.status-label {
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
		color: var(--status-color);
		white-space: nowrap;
	}

	/* Status variations */
	.status-valid {
		box-shadow: 0 0 0 1px color-mix(in srgb, var(--c-sage) 20%, transparent);
	}

	.status-invalid {
		box-shadow: 0 0 0 1px color-mix(in srgb, var(--c-warning) 20%, transparent);
	}

	.status-loading {
		opacity: 0.8;
	}
</style>
