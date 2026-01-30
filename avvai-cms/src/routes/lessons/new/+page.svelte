<script lang="ts">
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import type { Lesson, SourceMetadata } from '$lib/types/lesson';

	let id = $state('');
	let title = $state('');
	let description = $state('');
	let sourceTitle = $state('');
	let sourceAuthor = $state('');
	let sourcePeriod = $state('');

	let saving = $state(false);
	let error = $state('');

	async function handleSubmit() {
		if (!id.trim() || !title.trim()) {
			error = 'ID and Title are required';
			return;
		}

		saving = true;
		error = '';

		const source: SourceMetadata | undefined =
			sourceTitle || sourceAuthor || sourcePeriod
				? {
						title: sourceTitle,
						author: sourceAuthor || undefined,
						period: sourcePeriod || undefined
					}
				: undefined;

		const lesson: Lesson = {
			id: id.trim(),
			title: title.trim(),
			description: description.trim(),
			source,
			sections: []
		};

		try {
			const res = await fetch('/api/admin/lessons', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ lesson })
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Failed to create lesson');
			}

			window.location.href = `/lessons/${id}`;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			saving = false;
		}
	}
</script>

<div class="new-lesson-page">
	<div class="page-header">
		<div>
			<h1 class="page-title">New Lesson</h1>
			<p class="page-description">Create a new lesson. You can add sections after creation.</p>
		</div>
	</div>

	<Card title="Lesson Details">
		<form class="form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
			{#if error}
				<p class="error">{error}</p>
			{/if}

			<div class="form-grid">
				<TextInput label="Lesson ID" bind:value={id} placeholder="e.g., lesson-001" />
				<TextInput label="Title" bind:value={title} placeholder="Enter lesson title" />
			</div>

			<Textarea
				label="Description"
				bind:value={description}
				placeholder="Brief description of the lesson"
				rows={3}
			/>

			<h3 class="section-title">Source Metadata (optional)</h3>

			<div class="form-grid">
				<TextInput label="Source Title" bind:value={sourceTitle} placeholder="Original work title" />
				<TextInput label="Author" bind:value={sourceAuthor} placeholder="Author name" />
				<TextInput label="Period" bind:value={sourcePeriod} placeholder="e.g., Sangam Era" />
			</div>

			<div class="form-actions">
				<Button variant="outline" onclick={() => (window.location.href = '/lessons')}>
					Cancel
				</Button>
				<Button type="submit" disabled={saving}>
					{saving ? 'Creating...' : 'Create Lesson'}
				</Button>
			</div>
		</form>
	</Card>
</div>

<style>
	.new-lesson-page {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 700;
		color: var(--c-brown);
		margin: 0;
	}

	.page-description {
		font-family: var(--font-serif);
		font-size: 1.1rem;
		color: var(--c-brown-light);
		margin: 0.25rem 0 0;
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.section-title {
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 600;
		color: var(--c-brown);
		margin: 0.5rem 0 0;
		padding-top: 1rem;
		border-top: 1px solid rgba(92, 74, 61, 0.1);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(92, 74, 61, 0.1);
	}

	.error {
		font-family: var(--font-serif);
		color: #e53e3e;
		margin: 0;
		padding: 0.75rem 1rem;
		background-color: rgba(229, 62, 62, 0.1);
		border-radius: 8px;
	}
</style>
