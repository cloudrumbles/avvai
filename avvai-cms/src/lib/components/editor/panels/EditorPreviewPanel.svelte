<script lang="ts">
	import { scale } from 'svelte/transition';
	import LessonPreview from '$lib/components/preview/LessonPreview.svelte';
	import type { Lesson } from '$lib/types/lesson';

	interface Props {
		lesson: Lesson;
		isDirty: boolean;
		saving: boolean;
		onSave: () => void;
	}

	let { lesson, isDirty, saving, onSave }: Props = $props();
</script>

<div class="preview-panel">
	<header class="panel-header">
		<span class="panel-title">Preview</span>
		<button
			type="button"
			class="save-btn"
			class:saving
			disabled={saving || !isDirty}
			onclick={onSave}
		>
			{#if saving}
				<span class="spinner"></span>
				Saving...
			{:else}
				Save
			{/if}
		</button>
	</header>

	<div class="preview-content">
		<LessonPreview {lesson} compact />
	</div>
</div>

<style>
	.preview-panel {
		height: 100vh;
		display: flex;
		flex-direction: column;
		background: white;
		border-left: 1px solid rgba(92, 74, 61, 0.1);
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		background: linear-gradient(135deg, var(--c-cream) 0%, var(--c-cream-dark) 100%);
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
	}

	.panel-title {
		font-family: var(--font-display);
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--c-brown-dark);
	}

	.save-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		background: var(--c-terracotta);
		color: white;
		border: none;
		border-radius: var(--radius-sm);
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.save-btn:hover:not(:disabled) {
		background: var(--c-terracotta-dark);
	}

	.save-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.save-btn.saving {
		padding-left: var(--space-3);
	}

	.spinner {
		width: 12px;
		height: 12px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.preview-content {
		flex: 1;
		overflow-y: auto;
	}
</style>
