<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import type { DialogueSection } from '$lib/types/lesson';

	interface Props {
		section: DialogueSection;
	}

	let { section = $bindable() }: Props = $props();

	function addLine() {
		section.lines = [...section.lines, { text: '' }];
	}

	function removeLine(index: number) {
		section.lines = section.lines.filter((_, i) => i !== index);
	}

	// Ensure scene object exists
	if (!section.scene) {
		section.scene = {};
	}
</script>

<div class="dialogue-editor">
	<div class="meta-section">
		<TextInput
			label="Title (optional)"
			value={section.title ?? ''}
			placeholder="Scene Title"
			oninput={(e) => {
				section.title = e.currentTarget.value || undefined;
			}}
		/>

		<div class="scene-info">
			<TextInput
				label="Location"
				value={section.scene?.location ?? ''}
				oninput={(e) => {
					if (!section.scene) section.scene = {};
					section.scene.location = e.currentTarget.value || undefined;
				}}
			/>
			<TextInput
				label="Time"
				value={section.scene?.time ?? ''}
				oninput={(e) => {
					if (!section.scene) section.scene = {};
					section.scene.time = e.currentTarget.value || undefined;
				}}
			/>
		</div>
	</div>

	<div class="lines-list">
		{#each section.lines as line, index}
			<div class="dialogue-line" class:direction={line.direction}>
				<div class="line-header">
					<div class="char-input">
						<TextInput
							placeholder={line.direction ? "Direction Type" : "Character Name"}
							value={line.character ?? ''}
							oninput={(e) => {
								line.character = e.currentTarget.value || undefined;
							}}
						/>
					</div>
					
					<label class="direction-toggle">
						<input 
							type="checkbox" 
							checked={line.direction ?? false} 
							onchange={(e) => {
								line.direction = e.currentTarget.checked;
							}}
						/>
						Is Stage Direction?
					</label>

					<button 
						class="remove-btn" 
						onclick={() => removeLine(index)}
						title="Remove line"
						disabled={section.lines.length === 1}
					>
						×
					</button>
				</div>

				<Textarea
					placeholder="Dialogue text..."
					value={line.text}
					oninput={(e) => {
						line.text = e.currentTarget.value;
					}}
					rows={2}
				/>
			</div>
		{/each}
	</div>

	<Button 
		size="sm" 
		variant="ghost" 
		onclick={addLine}
	>
		+ Add Line
	</Button>
</div>

<style>
	.dialogue-editor {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.meta-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px dashed rgba(92, 74, 61, 0.2);
	}

	.scene-info {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.lines-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.dialogue-line {
		background-color: rgba(245, 242, 235, 0.3);
		border: 1px solid rgba(92, 74, 61, 0.1);
		border-radius: 8px;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		transition: background-color 0.2s ease;
	}

	.dialogue-line.direction {
		background-color: rgba(92, 74, 61, 0.05);
		border-style: dashed;
	}

	.line-header {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.char-input {
		width: 200px;
	}

	.direction-toggle {
		font-size: 0.85rem;
		color: var(--c-brown);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		margin-left: auto;
	}

	.remove-btn {
		width: 1.75rem;
		height: 1.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid rgba(92, 74, 61, 0.1);
		background: white;
		border-radius: 4px;
		color: #e53e3e;
		font-size: 1.25rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.remove-btn:hover:not(:disabled) {
		background-color: #FFF5F5;
		border-color: #e53e3e;
	}

	.remove-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}
</style>