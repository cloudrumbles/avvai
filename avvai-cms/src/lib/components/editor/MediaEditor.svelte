<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import type { MediaSection } from '$lib/types/lesson';

	interface Props {
		section: MediaSection;
	}

	let { section = $bindable() }: Props = $props();

	const mediaTypeOptions = [
		{ label: 'Image', value: 'image' },
		{ label: 'Audio', value: 'audio' },
		{ label: 'Video', value: 'video' }
	];
</script>

<div class="media-editor">
	<div class="form-row">
		<Select
			label="Media Type"
			value={section.media_type}
			options={mediaTypeOptions}
			onchange={(e) => {
				section.media_type = e.currentTarget.value as 'image' | 'audio' | 'video';
			}}
		/>
		
		<div class="url-input">
			<TextInput
				label="URL"
				value={section.url}
				placeholder="/images/..."
				oninput={(e) => {
					section.url = e.currentTarget.value;
				}}
			/>
		</div>
	</div>

	<TextInput
		label="Caption"
		value={section.caption ?? ''}
		placeholder="Media caption..."
		oninput={(e) => {
			section.caption = e.currentTarget.value || undefined;
		}}
	/>

	{#if section.media_type === 'image' && section.url}
		<div class="preview">
			<span class="preview-label">Preview:</span>
			<img src={section.url} alt={section.caption || 'Preview'} onerror={(e) => ((e.currentTarget as HTMLElement).style.display = 'none')} />
		</div>
	{/if}
</div>

<style>
	.media-editor {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 2fr;
		gap: 1rem;
	}

	.preview {
		margin-top: 0.5rem;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.03);
		border-radius: 8px;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.preview-label {
		font-size: 0.75rem;
		text-transform: uppercase;
		color: var(--c-brown-light);
		font-weight: 600;
	}

	.preview img {
		max-width: 100%;
		max-height: 200px;
		object-fit: contain;
		border-radius: 4px;
		border: 1px solid rgba(0, 0, 0, 0.1);
	}
</style>