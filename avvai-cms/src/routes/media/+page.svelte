<script lang="ts">
	import { onMount } from 'svelte';
	import { 
		PlusIcon, 
		SearchIcon,
		FolderIcon,
		ImageIcon,
		FileAudioIcon,
		MoreVerticalIcon,
		TrashIcon,
		CheckCircleIcon,
		ClockIcon
	} from '$lib/components/icons';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';

	let files = $state<string[]>([]);
	let loading = $state(true);
	let uploading = $state(false);
	let error = $state('');
	let searchQuery = $state('');
	let selectedFolder = $state('all');
	let selectedFile = $state<string | null>(null);

	let fileInput: HTMLInputElement;

	onMount(async () => {
		await loadMedia();
	});

	async function loadMedia() {
		loading = true;
		try {
			const res = await fetch('/api/admin/media');
			if (!res.ok) throw new Error('Failed to fetch media');
			files = await res.json();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	}

	async function handleUpload() {
		if (!fileInput?.files?.length) return;

		uploading = true;
		error = '';

		const formData = new FormData();
		formData.append('file', fileInput.files[0]);

		try {
			const res = await fetch('/api/admin/media', {
				method: 'POST',
				body: formData
			});

			if (!res.ok) {
				const data = await res.json();
				throw new Error(data.error || 'Upload failed');
			}

			fileInput.value = '';
			await loadMedia();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			uploading = false;
		}
	}

	async function deleteFile(filename: string) {
		if (!confirm(`Delete "${filename}"?`)) return;

		try {
			const res = await fetch(`/api/admin/media/${encodeURIComponent(filename)}`, {
				method: 'DELETE'
			});

			if (!res.ok) throw new Error('Delete failed');
			files = files.filter((f) => f !== filename);
			if (selectedFile === filename) selectedFile = null;
		} catch (e) {
			alert('Delete failed');
		}
	}

	function getFileType(filename: string): 'image' | 'audio' | 'other' {
		if (filename.match(/\.(jpg|jpeg|png|gif|webp|svg)$/i)) return 'image';
		if (filename.match(/\.(mp3|wav|ogg|m4a|aac)$/i)) return 'audio';
		return 'other';
	}

	function getFolderForFile(filename: string): string {
		// Mock folder assignment based on filename
		const folders = ['lessons', 'poetry', 'nature', 'audio'];
		const index = filename.charCodeAt(0) % folders.length;
		return folders[index];
	}

	const folders = $derived([
		{ id: 'all', name: 'All Files', count: files.length },
		{ id: 'lessons', name: 'Lessons', count: files.filter(f => getFolderForFile(f) === 'lessons').length },
		{ id: 'poetry', name: 'Poetry', count: files.filter(f => getFolderForFile(f) === 'poetry').length },
		{ id: 'nature', name: 'Nature', count: files.filter(f => getFolderForFile(f) === 'nature').length },
		{ id: 'audio', name: 'Audio', count: files.filter(f => getFileType(f) === 'audio').length },
	]);

	const filteredFiles = $derived(
		files.filter(file => {
			const matchesSearch = file.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesFolder = selectedFolder === 'all' || 
				(selectedFolder === 'audio' ? getFileType(file) === 'audio' : getFolderForFile(file) === selectedFolder);
			return matchesSearch && matchesFolder;
		})
	);

	function formatFileSize(filename: string): string {
		// Mock file sizes
		const sizes = ['2.4 MB', '1.8 MB', '856 KB', '3.2 MB', '1.1 MB'];
		const index = filename.length % sizes.length;
		return sizes[index];
	}

	function copyUrl(filename: string) {
		const url = `/api/admin/media/${filename}`;
		navigator.clipboard.writeText(url);
		// Could show a toast here
	}
</script>

<div class="media-page">
	<div class="page-header">
		<div class="header-content">
			<h1 class="page-title">Media Library</h1>
			<p class="page-description">{files.length} files across {folders.length - 1} folders</p>
		</div>
		<div class="header-actions">
			<input 
				type="file" 
				bind:this={fileInput} 
				accept="image/*,audio/*,video/*" 
				onchange={handleUpload}
				style="display: none"
				id="file-upload"
			/>
			<Button size="lg" onclick={() => fileInput?.click()} disabled={uploading}>
				<PlusIcon size={18} />
				<span>{uploading ? 'Uploading...' : 'Upload Files'}</span>
			</Button>
		</div>
	</div>

	{#if error}
		<div class="error-banner">
			<span>{error}</span>
			<button onclick={() => error = ''}>Dismiss</button>
		</div>
	{/if}

	<div class="media-layout">
		<aside class="folders-sidebar">
			<Card padding="md">
				<div class="folders-list">
					<div class="folders-header">
						<FolderIcon size={16} color="var(--c-brown-muted)" />
						<span>Folders</span>
					</div>
					{#each folders as folder}
						<button 
							class="folder-item"
							class:active={selectedFolder === folder.id}
							onclick={() => selectedFolder = folder.id}
						>
							<span class="folder-name">{folder.name}</span>
							<span class="folder-count">{folder.id === 'all' ? files.length : folder.count}</span>
						</button>
					{/each}
					<button class="folder-item add-folder">
						<PlusIcon size={14} color="var(--c-brown-muted)" />
						<span>New Folder</span>
					</button>
				</div>
			</Card>
		</aside>

		<div class="content-area">
			<div class="content-header">
				<div class="search-box">
					<SearchIcon size={18} color="var(--c-brown-muted)" />
					<input 
						type="text" 
						placeholder="Search files..." 
						bind:value={searchQuery}
					/>
				</div>
				<span class="results-count">{filteredFiles.length} files</span>
			</div>

			{#if loading}
				<div class="loading-state">
					<div class="loader">
						<div class="loader-ring"></div>
						<div class="loader-ring"></div>
						<div class="loader-ring"></div>
					</div>
					<p class="loading-text">Loading media...</p>
				</div>
			{:else if filteredFiles.length === 0}
				<div class="empty-state">
					<div class="empty-content">
						{#if searchQuery || selectedFolder !== 'all'}
							<SearchIcon size={48} color="var(--c-brown-muted)" />
							<h2 class="empty-title">No files found</h2>
							<p class="empty-text">Try adjusting your search or folder selection</p>
							<Button variant="outline" onclick={() => { searchQuery = ''; selectedFolder = 'all'; }}>
								Clear Filters
							</Button>
						{:else}
							<div class="empty-illustration">
								<ImageIcon size={48} color="var(--c-terracotta)" />
							</div>
							<h2 class="empty-title">No Media Yet</h2>
							<p class="empty-text">Upload images, audio, or video files to use in your lessons</p>
							<Button onclick={() => fileInput?.click()}>
								<PlusIcon size={16} />
								<span>Upload First File</span>
							</Button>
						{/if}
					</div>
				</div>
			{:else}
				<div class="files-grid">
					{#each filteredFiles as file (file)}
						{@const fileType = getFileType(file)}
						{@const isSelected = selectedFile === file}
						<div 
							class="file-card"
							class:selected={isSelected}
							onclick={() => selectedFile = isSelected ? null : file}
						>
							<div class="file-preview">
								{#if fileType === 'image'}
									<img src="/api/admin/media/{file}" alt={file} loading="lazy" />
								{:else if fileType === 'audio'}
									<div class="audio-preview">
										<FileAudioIcon size={32} color="var(--c-terracotta)" />
										<span class="audio-label">Audio</span>
									</div>
								{:else}
									<div class="file-preview-placeholder">
										<span>📄</span>
									</div>
								{/if}
								{#if isSelected}
									<div class="selection-indicator">
										<CheckCircleIcon size={20} color="white" />
									</div>
								{/if}
							</div>
							<div class="file-info">
								<span class="file-name" title={file}>{file}</span>
								<div class="file-meta">
									<span class="file-size">{formatFileSize(file)}</span>
									<span class="file-folder">{getFolderForFile(file)}</span>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		{#if selectedFile}
			<aside class="details-sidebar">
				<Card padding="md">
					<div class="file-details">
						<div class="details-preview">
							{#if getFileType(selectedFile) === 'image'}
								<img src="/api/admin/media/{selectedFile}" alt={selectedFile} />
							{:else}
								<div class="details-preview-placeholder">
									{getFileType(selectedFile) === 'audio' ? '🎵' : '📄'}
								</div>
							{/if}
						</div>
						
						<div class="details-info">
							<h3 class="details-name">{selectedFile}</h3>
							<div class="details-meta">
								<div class="meta-row">
									<span class="meta-label">Type</span>
									<span class="meta-value">{getFileType(selectedFile).toUpperCase()}</span>
								</div>
								<div class="meta-row">
									<span class="meta-label">Size</span>
									<span class="meta-value">{formatFileSize(selectedFile)}</span>
								</div>
								<div class="meta-row">
									<span class="meta-label">Folder</span>
									<span class="meta-value">{getFolderForFile(selectedFile)}</span>
								</div>
								<div class="meta-row">
									<span class="meta-label">Used in</span>
									<span class="meta-value">2 lessons</span>
								</div>
							</div>
						</div>

						<div class="details-actions">
							<Button variant="secondary" fullWidth onclick={() => selectedFile && copyUrl(selectedFile)}>
								Copy URL
							</Button>
							<Button variant="ghost" fullWidth onclick={() => {}}>
								Replace
							</Button>
							<Button 
								variant="ghost" 
								fullWidth 
								onclick={() => selectedFile && deleteFile(selectedFile)}
								class="delete-btn"
							>
								<TrashIcon size={16} color="var(--c-terracotta)" />
								<span>Delete</span>
							</Button>
						</div>
					</div>
				</Card>
			</aside>
		{/if}
	</div>
</div>

<style>
	.media-page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: var(--space-4);
	}

	.header-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.page-description {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-muted);
		margin: 0;
	}

	.error-banner {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--c-error-bg);
		border: 1px solid rgba(184, 83, 61, 0.2);
		border-radius: var(--radius-md);
		color: var(--c-terracotta);
		font-size: 0.9rem;
	}

	.error-banner button {
		background: none;
		border: none;
		color: var(--c-terracotta);
		font-weight: 500;
		cursor: pointer;
	}

	.media-layout {
		display: grid;
		grid-template-columns: 200px 1fr auto;
		gap: 1.5rem;
		align-items: start;
	}

	@media (max-width: 1024px) {
		.media-layout {
			grid-template-columns: 1fr;
		}
		.folders-sidebar,
		.details-sidebar {
			display: none;
		}
	}

	/* Folders Sidebar */
	.folders-sidebar {
		position: sticky;
		top: 1rem;
	}

	.folders-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.folders-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--c-brown-muted);
		padding: 0.5rem 0.75rem;
		margin-bottom: 0.25rem;
	}

	.folder-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.625rem 0.75rem;
		border-radius: var(--radius-sm);
		font-size: 0.9rem;
		color: var(--c-brown);
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.folder-item:hover {
		background: var(--c-cream-dark);
	}

	.folder-item.active {
		background: var(--c-terracotta);
		color: white;
	}

	.folder-item.active .folder-count {
		background: rgba(255, 255, 255, 0.2);
		color: white;
	}

	.folder-name {
		font-weight: 500;
	}

	.folder-count {
		font-size: 0.75rem;
		padding: 0.15rem 0.5rem;
		background: var(--c-cream-dark);
		border-radius: var(--radius-full);
		color: var(--c-brown-muted);
	}

	.folder-item.add-folder {
		color: var(--c-brown-muted);
		margin-top: 0.5rem;
		border-top: 1px solid rgba(92, 74, 61, 0.08);
		border-radius: 0;
		padding-top: 0.75rem;
	}

	/* Content Area */
	.content-area {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.content-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex: 1;
		max-width: 400px;
		padding: 0.625rem 1rem;
		background: white;
		border: 1px solid rgba(92, 74, 61, 0.12);
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
	}

	.search-box:focus-within {
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(184, 83, 61, 0.08);
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.95rem;
		color: var(--c-brown);
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--c-brown-muted);
	}

	.results-count {
		font-size: 0.9rem;
		color: var(--c-brown-muted);
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 0;
		gap: 1rem;
	}

	.loader {
		position: relative;
		width: 48px;
		height: 48px;
	}

	.loader-ring {
		position: absolute;
		inset: 0;
		border: 2px solid transparent;
		border-top-color: var(--c-terracotta);
		border-radius: 50%;
		animation: spin 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
	}

	.loader-ring:nth-child(2) {
		inset: 6px;
		animation-delay: -0.15s;
		border-top-color: var(--c-gold);
	}

	.loader-ring:nth-child(3) {
		inset: 12px;
		animation-delay: -0.3s;
		border-top-color: var(--c-sage);
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.loading-text {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-light);
		font-style: italic;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 0;
		text-align: center;
	}

	.empty-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.empty-illustration {
		width: 96px;
		height: 96px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--c-exercises-bg);
		border-radius: var(--radius-lg);
		border: 1px solid rgba(184, 83, 61, 0.15);
	}

	.empty-title {
		font-family: var(--font-heading);
		font-size: 1.35rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.empty-text {
		font-family: var(--font-serif);
		color: var(--c-brown-muted);
		margin: 0;
		max-width: 400px;
	}

	.files-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 1rem;
	}

	.file-card {
		position: relative;
		background: white;
		border: 2px solid transparent;
		border-radius: var(--radius-md);
		overflow: hidden;
		cursor: pointer;
		transition: all var(--transition-fast);
		box-shadow: var(--shadow-sm);
	}

	.file-card:hover {
		transform: translateY(-2px);
		box-shadow: var(--shadow-md);
	}

	.file-card.selected {
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(184, 83, 61, 0.15);
	}

	.file-preview {
		aspect-ratio: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--c-cream-dark);
		overflow: hidden;
		position: relative;
	}

	.file-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.audio-preview {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.audio-label {
		font-size: 0.75rem;
		color: var(--c-brown-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.file-preview-placeholder {
		font-size: 2rem;
	}

	.selection-indicator {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 24px;
		height: 24px;
		background: var(--c-terracotta);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.file-info {
		padding: 0.75rem;
	}

	.file-name {
		display: block;
		font-size: 0.8rem;
		color: var(--c-brown);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		margin-bottom: 0.25rem;
	}

	.file-meta {
		display: flex;
		justify-content: space-between;
		font-size: 0.7rem;
		color: var(--c-brown-muted);
	}

	.file-folder {
		text-transform: capitalize;
	}

	/* Details Sidebar */
	.details-sidebar {
		position: sticky;
		top: 1rem;
		width: 260px;
	}

	.file-details {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.details-preview {
		aspect-ratio: 1;
		border-radius: var(--radius-md);
		overflow: hidden;
		background: var(--c-cream-dark);
	}

	.details-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.details-preview-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 3rem;
	}

	.details-name {
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 600;
		color: var(--c-brown-dark);
		margin: 0;
		word-break: break-word;
	}

	.details-meta {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.meta-row {
		display: flex;
		justify-content: space-between;
		font-size: 0.85rem;
	}

	.meta-label {
		color: var(--c-brown-muted);
	}

	.meta-value {
		color: var(--c-brown);
		font-weight: 500;
		text-transform: capitalize;
	}

	.details-actions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(92, 74, 61, 0.08);
	}

	:global(.details-actions .delete-btn) {
		color: var(--c-terracotta);
	}
</style>
