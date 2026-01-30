<script lang="ts">
	import { slide } from 'svelte/transition';
	import { ChevronIcon, ArrowUpIcon, ArrowDownIcon, TrashIcon } from '$lib/components/icons';
	import KolamPattern from '$lib/components/ui/KolamPattern.svelte';

	interface Props {
		id?: string;
		title: string;
		type: string;
		isFirst?: boolean;
		isLast?: boolean;
		onMoveUp?: () => void;
		onMoveDown?: () => void;
		onDelete?: () => void;
		children: import('svelte').Snippet;
		headerActions?: import('svelte').Snippet;
		expanded?: boolean;
	}

	let {
		id,
		title,
		type,
		isFirst = false,
		isLast = false,
		onMoveUp,
		onMoveDown,
		onDelete,
		children,
		headerActions,
		expanded = $bindable(true)
	}: Props = $props();

	// Color mapping for section types using the new palette
	const typeColors: Record<string, { bg: string; text: string; accent: string }> = {
		prose: { bg: '#EBF8FF', text: '#2B6CB0', accent: '#2B6CB0' },
		poetry: { bg: '#FAF5FF', text: '#805AD5', accent: '#805AD5' },
		vocabulary: { bg: '#F0FFF4', text: '#2F855A', accent: '#2F855A' },
		exercises: { bg: '#FEF3F0', text: '#B8533D', accent: '#B8533D' },
		media: { bg: '#FFFFF0', text: '#B7791F', accent: '#B7791F' },
		dialogue: { bg: '#FFF5F7', text: '#D53F8C', accent: '#D53F8C' }
	};

	let typeConfig = $derived(typeColors[type] || typeColors.prose);

	function toggleExpand() {
		expanded = !expanded;
	}
</script>

<div {id} class="editor-section" class:collapsed={!expanded}>
	<div class="accent-bar" style="--accent-color: {typeConfig.accent}">
		<div class="gradient-overlay"></div>
	</div>

	<div class="section-content">
		<div class="section-header">
			<div class="header-left">
				<button
					class="toggle-btn"
					onclick={toggleExpand}
					aria-label={expanded ? 'Collapse section' : 'Expand section'}
				>
					<ChevronIcon expanded={!expanded} size={14} color="var(--c-brown-light)" />
				</button>

				<div class="title-group">
					<span class="badge" style="--badge-bg: {typeConfig.bg}; --badge-text: {typeConfig.text}">
						{type}
					</span>
					<span class="section-title">{title || '(Untitled Section)'}</span>
				</div>
			</div>

			<div class="header-right">
				{#if headerActions}
					<div class="custom-actions">
						{@render headerActions()}
					</div>
				{/if}

				<div class="actions">
					<button
						class="action-btn"
						disabled={isFirst}
						onclick={onMoveUp}
						title="Move Up"
					>
						<ArrowUpIcon size={14} color={isFirst ? 'var(--c-brown-subtle)' : 'var(--c-brown)'} />
					</button>
					<button
						class="action-btn"
						disabled={isLast}
						onclick={onMoveDown}
						title="Move Down"
					>
						<ArrowDownIcon size={14} color={isLast ? 'var(--c-brown-subtle)' : 'var(--c-brown)'} />
					</button>
					<button
						class="action-btn delete"
						onclick={onDelete}
						title="Delete Section"
					>
						<TrashIcon size={14} color="var(--c-terracotta)" />
					</button>
				</div>
			</div>
		</div>

		{#if expanded}
			<div class="section-body" transition:slide={{ duration: 200 }}>
				{@render children()}
			</div>
		{/if}
	</div>

	<div class="corner-decoration">
		<KolamPattern variant="corner" size="sm" color="terracotta" />
	</div>
</div>

<style>
	.editor-section {
		display: flex;
		position: relative;
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.1);
		border-radius: var(--radius-md);
		margin-bottom: var(--space-4);
		overflow: hidden;
		transition: all var(--transition-base);
		box-shadow: var(--shadow-sm);
	}

	.editor-section:hover {
		box-shadow: var(--shadow-md);
		transform: translateY(-1px);
	}

	.editor-section.collapsed {
		background: var(--c-cream-warm);
	}

	.accent-bar {
		width: 6px;
		flex-shrink: 0;
		background: linear-gradient(
			180deg,
			var(--accent-color) 0%,
			color-mix(in srgb, var(--accent-color) 70%, var(--c-gold)) 50%,
			var(--c-gold) 100%
		);
		position: relative;
	}

	.gradient-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			90deg,
			rgba(255, 255, 255, 0.3) 0%,
			transparent 50%,
			rgba(0, 0, 0, 0.05) 100%
		);
	}

	.section-content {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--space-3) var(--space-4);
		background: linear-gradient(180deg, var(--c-cream) 0%, rgba(250, 246, 240, 0.5) 100%);
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
		user-select: none;
	}

	.editor-section.collapsed .section-header {
		border-bottom: none;
		background: transparent;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex: 1;
		min-width: 0;
	}

	.toggle-btn {
		background: rgba(92, 74, 61, 0.06);
		border: none;
		cursor: pointer;
		padding: var(--space-1);
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-sm);
		transition: all var(--transition-fast);
	}

	.toggle-btn:hover {
		background: rgba(92, 74, 61, 0.12);
	}

	.title-group {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		min-width: 0;
	}

	.badge {
		font-family: var(--font-heading);
		font-size: 0.65rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		padding: 0.2rem 0.5rem;
		border-radius: var(--radius-sm);
		background: var(--badge-bg);
		color: var(--badge-text);
		border: 1px solid color-mix(in srgb, var(--badge-text) 15%, transparent);
		white-space: nowrap;
	}

	.section-title {
		font-family: var(--font-heading);
		font-weight: 600;
		color: var(--c-brown-dark);
		font-size: 0.95rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex-shrink: 0;
	}

	.custom-actions {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.actions {
		display: flex;
		gap: var(--space-1);
		padding-left: var(--space-2);
		border-left: 1px solid rgba(92, 74, 61, 0.1);
	}

	.action-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid rgba(92, 74, 61, 0.15);
		border-radius: var(--radius-sm);
		background: white;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.action-btn:hover:not(:disabled) {
		background: var(--c-cream-dark);
		border-color: rgba(92, 74, 61, 0.25);
		transform: translateY(-1px);
		box-shadow: var(--shadow-sm);
	}

	.action-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	.action-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
		background: transparent;
		border-color: rgba(92, 74, 61, 0.08);
	}

	.action-btn.delete:hover {
		background: var(--c-exercises-bg);
		border-color: rgba(184, 83, 61, 0.3);
	}

	.section-body {
		padding: var(--space-6);
		background: rgba(255, 255, 255, 0.6);
	}

	.corner-decoration {
		position: absolute;
		top: var(--space-2);
		right: var(--space-2);
		opacity: 0.3;
		pointer-events: none;
	}

	.editor-section:hover .corner-decoration {
		opacity: 0.5;
	}
</style>
