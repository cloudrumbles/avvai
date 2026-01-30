<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import type { VocabularySection } from '$lib/types/lesson';

	interface Props {
		section: VocabularySection;
	}

	let { section = $bindable() }: Props = $props();

	function addEntry() {
		section.entries = [...section.entries, { word: '', meaning: '' }];
	}

	function removeEntry(index: number) {
		section.entries = section.entries.filter((_, i) => i !== index);
	}
</script>

<div class="vocab-editor">
	<TextInput
		label="Title (optional)"
		value={section.title ?? ''}
		placeholder="e.g. Key Terms"
		oninput={(e) => {
			section.title = e.currentTarget.value || undefined;
		}}
	/>

	<div class="entries-grid">
		<div class="grid-header">
			<span>Word / Phrase</span>
			<span>Meaning / Definition</span>
			<span></span>
		</div>
		
		{#each section.entries as entry, index}
			<div class="entry-row">
				<div class="input-cell">
					<TextInput
						value={entry.word}
						placeholder="Word"
						oninput={(e) => {
							entry.word = e.currentTarget.value;
						}}
					/>
				</div>
				<div class="input-cell">
					<TextInput
						value={entry.meaning}
						placeholder="Meaning"
						oninput={(e) => {
							entry.meaning = e.currentTarget.value;
						}}
					/>
				</div>
				<button 
					class="remove-btn" 
					onclick={() => removeEntry(index)}
					title="Remove entry"
					disabled={section.entries.length === 1}
				>
					×
				</button>
			</div>
		{/each}
	</div>

	<Button 
		size="sm" 
		variant="ghost" 
		onclick={addEntry}
	>
		+ Add Vocabulary Entry
	</Button>
</div>

<style>
	.vocab-editor {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.entries-grid {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.grid-header {
		display: grid;
		grid-template-columns: 1fr 1fr 2rem;
		gap: 1rem;
		padding: 0 0.5rem;
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown-light);
		margin-bottom: 0.25rem;
	}

	.entry-row {
		display: grid;
		grid-template-columns: 1fr 1fr 2rem;
		gap: 1rem;
		align-items: flex-start;
	}

	.remove-btn {
		height: 2.5rem; /* Match standard input height */
		width: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		color: #e53e3e;
		font-size: 1.5rem;
		cursor: pointer;
		opacity: 0.5;
		transition: all 0.2s;
		margin-top: 1.5rem; /* Align with input field, accounting for label space if any, though here we use placeholder */
	}

	/* Adjust margin since we are using inputs without labels in the grid (using placeholders) */
	/* But TextInput component likely includes label spacing even if label is empty, 
	   so let's check structure. Based on usage, TextInput has internal label. */
	
	.entry-row .remove-btn {
		margin-top: 0;
		height: 100%;
		align-self: center;
	}

	.remove-btn:hover:not(:disabled) {
		opacity: 1;
		background-color: rgba(229, 62, 62, 0.1);
		border-radius: 4px;
	}

	.remove-btn:disabled {
		opacity: 0.2;
		cursor: not-allowed;
	}
</style>