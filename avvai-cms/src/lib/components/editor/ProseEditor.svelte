<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { PlusIcon, TrashIcon } from '$lib/components/icons';
	import KolamPattern from '$lib/components/ui/KolamPattern.svelte';
	import type { ProseSection } from '$lib/types/lesson';

	interface Props {
		section: ProseSection;
	}

	let { section = $bindable() }: Props = $props();

	function addParagraph() {
		section.paragraphs = [...section.paragraphs, ''];
	}

	function removeParagraph(index: number) {
		if (section.paragraphs.length > 1) {
			section.paragraphs = section.paragraphs.filter((_, i) => i !== index);
		}
	}
</script>

<div class="prose-editor">
	<div class="editor-header">
		<div class="header-accent">
			<KolamPattern variant="dot" size="sm" color="terracotta" />
		</div>
		<TextInput
			label="Section Title (Optional)"
			value={section.title ?? ''}
			placeholder="e.g., Introduction to the Text"
			oninput={(e) => {
				section.title = e.currentTarget.value || undefined;
			}}
		/>
	</div>

	<div class="paragraphs-container">
		<div class="section-label">
			<span class="label-text">Paragraphs</span>
			<span class="label-count">{section.paragraphs.length}</span>
		</div>

		<div class="paragraphs">
			{#each section.paragraphs as para, index (index)}
				<div class="paragraph-item">
					<div class="paragraph-number">
						<span class="number">{String(index + 1).padStart(2, '0')}</span>
					</div>
					<div class="paragraph-content">
						<Textarea
							value={para}
							oninput={(e) => {
								section.paragraphs[index] = e.currentTarget.value;
							}}
							placeholder="Enter paragraph text..."
							rows={4}
						/>
					</div>
					<button 
						class="remove-btn" 
						onclick={() => removeParagraph(index)}
						aria-label="Remove paragraph"
						title="Remove paragraph"
						disabled={section.paragraphs.length === 1}
					>
						<TrashIcon size={16} color="var(--c-terracotta)" />
					</button>
				</div>
			{/each}
		</div>
	</div>

	<div class="add-section">
		<Button 
			variant="outline" 
			size="sm"
			onclick={addParagraph}
		>
			<PlusIcon size={16} />
			<span>Add Paragraph</span>
		</Button>
	</div>
</div>

<style>
	.prose-editor {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.editor-header {
		display: flex;
		gap: var(--space-3);
		align-items: flex-start;
	}

	.header-accent {
		padding-top: 28px;
		opacity: 0.6;
	}

	.paragraphs-container {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.section-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.label-text {
		font-family: var(--font-heading);
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown);
	}

	.label-count {
		font-size: 0.7rem;
		color: var(--c-brown-muted);
		background: var(--c-cream-dark);
		padding: 0.1rem 0.4rem;
		border-radius: 10px;
	}

	.paragraphs {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.paragraph-item {
		display: flex;
		gap: var(--space-3);
		align-items: flex-start;
		padding: var(--space-4);
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.08);
		border-radius: var(--radius-md);
		transition: all var(--transition-base);
	}

	.paragraph-item:hover {
		border-color: rgba(92, 74, 61, 0.15);
		box-shadow: var(--shadow-sm);
	}

	.paragraph-number {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: linear-gradient(135deg, var(--c-cream) 0%, var(--c-cream-dark) 100%);
		border-radius: var(--radius-sm);
		flex-shrink: 0;
	}

	.number {
		font-family: var(--font-heading);
		font-size: 0.8rem;
		font-weight: 700;
		color: var(--c-terracotta);
	}

	.paragraph-content {
		flex: 1;
		min-width: 0;
	}

	.remove-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: 1px solid rgba(184, 83, 61, 0.15);
		background: white;
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--transition-fast);
		flex-shrink: 0;
	}

	.remove-btn:hover:not(:disabled) {
		background: var(--c-error-bg);
		border-color: var(--c-terracotta);
		transform: scale(1.05);
	}

	.remove-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.add-section {
		display: flex;
		justify-content: center;
		padding-top: var(--space-4);
		border-top: 1px dashed rgba(92, 74, 61, 0.15);
	}
</style>
