<script lang="ts">
	import { onMount } from 'svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';

	interface CacheEntry {
		key: string;
		entry: {
			word: string;
			definition?: string;
			examples?: string[];
		};
	}

	let entries = $state<CacheEntry[]>([]);
	let loading = $state(true);
	let error = $state('');

	// Form state for new/edit
	let showForm = $state(false);
	let editingKey = $state<string | null>(null);
	let formKey = $state('');
	let formWord = $state('');
	let formDefinition = $state('');
	let saving = $state(false);

	onMount(async () => {
		await loadEntries();
	});

	async function loadEntries() {
		loading = true;
		try {
			const res = await fetch('/api/admin/dictionary-cache');
			if (!res.ok) throw new Error('Failed to fetch entries');
			entries = await res.json();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	}

	function startNew() {
		editingKey = null;
		formKey = '';
		formWord = '';
		formDefinition = '';
		showForm = true;
	}

	function startEdit(entry: CacheEntry) {
		editingKey = entry.key;
		formKey = entry.key;
		formWord = entry.entry.word;
		formDefinition = entry.entry.definition ?? '';
		showForm = true;
	}

	function cancelEdit() {
		editingKey = null;
		formKey = '';
		formWord = '';
		formDefinition = '';
		showForm = false;
	}

	async function saveEntry() {
		if (!formKey.trim() || !formWord.trim()) {
			alert('Key and Word are required');
			return;
		}

		saving = true;
		error = '';

		try {
			const res = await fetch('/api/admin/dictionary-cache?action=upsert', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					key: formKey.trim(),
					entry: {
						word: formWord.trim(),
						definition: formDefinition.trim() || '',
						examples: []
					}
				})
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Save failed');
			}

			cancelEdit();
			await loadEntries();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			saving = false;
		}
	}

	async function deleteEntry(key: string) {
		if (!confirm(`Delete entry "${key}"?`)) return;

		try {
			const res = await fetch('/api/admin/dictionary-cache', {
				method: 'DELETE',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ key })
			});

			if (!res.ok) throw new Error('Delete failed');
			entries = entries.filter((e) => e.key !== key);
		} catch (e) {
			alert('Delete failed');
		}
	}
</script>

<div class="dictionary-page">
	<div class="page-header">
		<div>
			<h1 class="page-title">Dictionary Cache</h1>
			<p class="page-description">Manage cached dictionary entries.</p>
		</div>
		<Button onclick={startNew}>New Entry</Button>
	</div>

	{#if showForm}
		<Card title={editingKey ? 'Edit Entry' : 'New Entry'}>
			<div class="entry-form">
				{#if error}
					<p class="error">{error}</p>
				{/if}

				<TextInput
					label="Key"
					bind:value={formKey}
					placeholder="Cache key"
					disabled={editingKey !== null}
				/>
				<TextInput label="Word" bind:value={formWord} placeholder="Tamil word" />
				<Textarea
					label="Definition"
					bind:value={formDefinition}
					placeholder="Definition/meaning"
					rows={3}
				/>

				<div class="form-actions">
					<Button variant="outline" onclick={cancelEdit}>Cancel</Button>
					<Button onclick={saveEntry} disabled={saving}>
						{saving ? 'Saving...' : 'Save'}
					</Button>
				</div>
			</div>
		</Card>
	{/if}

	<Card title="Cached Entries">
		{#if loading}
			<p class="loading">Loading entries...</p>
		{:else if entries.length === 0}
			<p class="empty">No cached entries yet.</p>
		{:else}
			<table class="entries-table">
				<thead>
					<tr>
						<th>Key</th>
						<th>Word</th>
						<th>Definition</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each entries as entry (entry.key)}
						<tr>
							<td class="key-cell">
								<code>{entry.key}</code>
							</td>
							<td class="word-cell">{entry.entry.word}</td>
							<td class="def-cell">{entry.entry.definition ?? '-'}</td>
							<td class="actions-cell">
								<Button variant="ghost" size="sm" onclick={() => startEdit(entry)}>Edit</Button>
								<Button variant="ghost" size="sm" onclick={() => deleteEntry(entry.key)}>
									Delete
								</Button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	</Card>
</div>

<style>
	.dictionary-page {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 1rem;
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

	.entry-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(92, 74, 61, 0.1);
	}

	.loading,
	.empty {
		font-family: var(--font-serif);
		color: var(--c-brown-light);
		margin: 0;
	}

	.error {
		font-family: var(--font-serif);
		color: #e53e3e;
		margin: 0;
		padding: 0.75rem 1rem;
		background-color: rgba(229, 62, 62, 0.1);
		border-radius: 8px;
	}

	.entries-table {
		width: 100%;
		border-collapse: collapse;
	}

	.entries-table th,
	.entries-table td {
		padding: 0.75rem 1rem;
		text-align: left;
		border-bottom: 1px solid rgba(92, 74, 61, 0.1);
	}

	.entries-table th {
		font-family: var(--font-heading);
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown-light);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background-color: rgba(245, 242, 235, 0.5);
	}

	.entries-table tbody tr:hover {
		background-color: rgba(245, 242, 235, 0.5);
	}

	.key-cell code {
		font-family: monospace;
		font-size: 0.85rem;
		background-color: rgba(92, 74, 61, 0.1);
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
	}

	.word-cell {
		font-family: var(--font-tamil);
		font-weight: 500;
	}

	.def-cell {
		font-family: var(--font-serif);
		color: var(--c-brown-light);
		max-width: 300px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.actions-cell {
		display: flex;
		gap: 0.5rem;
	}
</style>
