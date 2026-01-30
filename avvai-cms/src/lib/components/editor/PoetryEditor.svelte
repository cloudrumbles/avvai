<script lang="ts">
	import { PlusIcon, TrashIcon, ArrowLeftIcon, ArrowRightIcon } from '$lib/components/icons';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import type { PoetrySection, Verse } from '$lib/types/lesson';

	interface Props {
		section: PoetrySection;
	}

	let { section = $bindable() }: Props = $props();
	let activeVerseIndex = $state(0);
	let showPreview = $state(false);

	function addVerse() {
		section.verses = [...section.verses, { lines: [''] }];
		activeVerseIndex = section.verses.length - 1;
	}

	function removeVerse(index: number) {
		if (section.verses.length <= 1) return;
		section.verses = section.verses.filter((_, i) => i !== index);
		if (activeVerseIndex >= section.verses.length) {
			activeVerseIndex = section.verses.length - 1;
		}
	}

	function addLine(verseIndex: number) {
		section.verses[verseIndex].lines = [...section.verses[verseIndex].lines, ''];
	}

	function removeLine(verseIndex: number, lineIndex: number) {
		if (section.verses[verseIndex].lines.length <= 1) return;
		section.verses[verseIndex].lines = section.verses[verseIndex].lines.filter((_, i) => i !== lineIndex);
	}

	function getVersePreview(verse: Verse): string {
		return verse.lines.slice(0, 2).join(' ').substring(0, 40) + (verse.lines.join(' ').length > 40 ? '...' : '');
	}
</script>

<div class="poetry-editor">
	<div class="editor-layout">
		<aside class="verse-nav">
			<Card padding="md">
				<div class="nav-header">
					<span class="nav-title">Verses</span>
					<span class="nav-count">{section.verses.length}</span>
				</div>
				<div class="verse-list">
					{#each section.verses as verse, index}
						<button
							class="verse-nav-item"
							class:active={activeVerseIndex === index}
							onclick={() => activeVerseIndex = index}
						>
							<div class="verse-nav-number">
								{verse.number || index + 1}
							</div>
							<div class="verse-nav-preview">
								{getVersePreview(verse) || 'Empty verse'}
							</div>
						</button>
					{/each}
					<button class="verse-nav-item add-verse" onclick={addVerse}>
						<PlusIcon size={14} />
						<span>Add Verse</span>
					</button>
				</div>
			</Card>
		</aside>

		<div class="editor-main">
			{#if section.verses[activeVerseIndex]}
				{@const verse = section.verses[activeVerseIndex]}
				{@const vIndex = activeVerseIndex}
				
				<Card padding="lg">
					<div class="verse-editor">
						<div class="verse-editor-header">
							<div class="verse-number-block">
								<span class="verse-label">Verse {vIndex + 1}</span>
								<div class="number-input-group">
									<label for="verse-number-{vIndex}">Number:</label>
									<input 
										id="verse-number-{vIndex}"
										type="number" 
										class="number-input" 
										value={verse.number ?? ''} 
										oninput={(e) => verse.number = e.currentTarget.valueAsNumber || undefined}
										placeholder="{(vIndex + 1).toString()}"
									/>
								</div>
							</div>
							<div class="verse-actions">
								<button 
									class="preview-toggle"
									class:active={showPreview}
									onclick={() => showPreview = !showPreview}
								>
									{showPreview ? 'Edit' : 'Preview'}
								</button>
								<button 
									class="remove-verse-btn"
									onclick={() => removeVerse(vIndex)}
									disabled={section.verses.length === 1}
									title="Delete this verse"
								>
									<TrashIcon size={14} />
								</button>
							</div>
						</div>

						{#if showPreview}
							<div class="verse-preview">
								<div class="preview-poem">
									{#each verse.lines as line}
										{#if line.trim()}
											<p class="preview-line">{line}</p>
										{/if}
									{/each}
								</div>
								{#if verse.translation}
									<div class="preview-translation">
										<p>{verse.translation}</p>
									</div>
								{/if}
							</div>
						{:else}
							<div class="lines-editor">
								<span class="field-label">Poem Lines</span>
								<div class="lines-list">
									{#each verse.lines as line, lIndex}
										<div class="line-row">
											<span class="line-number">{lIndex + 1}</span>
											<input
												type="text"
												class="line-input"
												value={line}
												placeholder="Enter line {lIndex + 1}..."
												oninput={(e) => {
													verse.lines[lIndex] = e.currentTarget.value;
												}}
											/>
											<button 
												class="remove-line-btn"
												onclick={() => removeLine(vIndex, lIndex)}
												disabled={verse.lines.length === 1}
												title="Remove line"
											>
												×
											</button>
										</div>
									{/each}
								</div>
								<Button 
									size="sm" 
									variant="ghost" 
									onclick={() => addLine(vIndex)}
								>
									<PlusIcon size={14} />
									Add Line
								</Button>
							</div>

							<div class="translation-editor">
								<label class="field-label" for="verse-translation-{vIndex}">
									English Translation
								</label>
								<textarea
									id="verse-translation-{vIndex}"
									class="translation-input"
									value={verse.translation ?? ''}
									placeholder="Enter English translation of this verse..."
									oninput={(e) => {
										verse.translation = e.currentTarget.value || undefined;
									}}
									rows={3}
								></textarea>
							</div>
						{/if}
					</div>
				</Card>

				<div class="verse-navigation-footer">
					<button 
						class="nav-prev"
						disabled={vIndex === 0}
						onclick={() => activeVerseIndex = vIndex - 1}
					>
							<ArrowLeftIcon size={16} />
							Previous Verse
					</button>
					<span class="verse-indicator">
						Verse {vIndex + 1} of {section.verses.length}
					</span>
					<button 
						class="nav-next"
						disabled={vIndex === section.verses.length - 1}
						onclick={() => activeVerseIndex = vIndex + 1}
					>
							Next Verse
							<ArrowRightIcon size={16} />
					</button>
				</div>
			{/if}
		</div>
	</div>

	<div class="section-title-editor">
		<label class="field-label" for="section-title">Section Title (optional)</label>
		<input
			id="section-title"
			type="text"
			class="title-input"
			value={section.title ?? ''}
			placeholder="e.g., Kuruntogai 40"
			oninput={(e) => {
				section.title = e.currentTarget.value || undefined;
			}}
		/>
	</div>
</div>

<style>
	.poetry-editor {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.editor-layout {
		display: grid;
		grid-template-columns: 200px 1fr;
		gap: 1.5rem;
		align-items: start;
	}

	@media (max-width: 768px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}
		.verse-nav {
			order: 2;
		}
	}

	/* Verse Navigation Sidebar */
	.verse-nav {
		position: sticky;
		top: 1rem;
		align-self: start;
		max-height: calc(100vh - 2rem);
		overflow-y: auto;
		overflow-x: hidden;
	}

	.verse-nav::-webkit-scrollbar {
		width: 4px;
	}

	.verse-nav::-webkit-scrollbar-track {
		background: transparent;
	}

	.verse-nav::-webkit-scrollbar-thumb {
		background: var(--c-brown-subtle);
		border-radius: 2px;
	}

	.nav-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
	}

	.nav-title {
		font-family: var(--font-heading);
		font-weight: 600;
		font-size: 0.9rem;
		color: var(--c-brown);
	}

	.nav-count {
		font-size: 0.75rem;
		padding: 0.15rem 0.5rem;
		background: var(--c-cream-dark);
		border-radius: var(--radius-full);
		color: var(--c-brown-muted);
	}

	.verse-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.verse-nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.625rem 0.75rem;
		border-radius: var(--radius-sm);
		background: transparent;
		border: none;
		cursor: pointer;
		text-align: left;
		transition: all var(--transition-fast);
	}

	.verse-nav-item:hover {
		background: var(--c-cream-dark);
	}

	.verse-nav-item.active {
		background: var(--c-poetry-bg);
		border-left: 3px solid var(--c-poetry);
	}

	.verse-nav-number {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--c-cream-dark);
		border-radius: var(--radius-sm);
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown);
		flex-shrink: 0;
	}

	.verse-nav-item.active .verse-nav-number {
		background: var(--c-poetry);
		color: white;
	}

	.verse-nav-preview {
		font-size: 0.8rem;
		color: var(--c-brown-light);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.verse-nav-item.add-verse {
		color: var(--c-terracotta);
		font-weight: 500;
		margin-top: 0.5rem;
		border-top: 1px solid rgba(92, 74, 61, 0.08);
		border-radius: 0;
		padding-top: 0.75rem;
	}

	/* Main Editor */
	.editor-main {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.verse-editor {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.verse-editor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(92, 74, 61, 0.08);
	}

	.verse-number-block {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.verse-label {
		font-family: var(--font-heading);
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--c-brown-dark);
	}

	.number-input-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		color: var(--c-brown-muted);
	}

	.number-input {
		width: 4rem;
		padding: 0.375rem 0.5rem;
		border: 1px solid rgba(92, 74, 61, 0.2);
		border-radius: var(--radius-sm);
		font-size: 0.9rem;
		text-align: center;
	}

	.number-input:focus {
		outline: none;
		border-color: var(--c-terracotta);
	}

	.verse-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.preview-toggle {
		padding: 0.5rem 1rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--c-brown);
		background: var(--c-cream-dark);
		border: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.preview-toggle:hover {
		background: var(--c-brown-subtle);
	}

	.preview-toggle.active {
		background: var(--c-poetry);
		color: white;
	}

	.remove-verse-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: transparent;
		border-radius: var(--radius-sm);
		color: var(--c-terracotta);
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.remove-verse-btn:hover:not(:disabled) {
		background: var(--c-error-bg);
	}

	.remove-verse-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	/* Preview Mode */
	.verse-preview {
		background: var(--c-cream);
		border-radius: var(--radius-md);
		padding: 2rem;
	}

	.preview-poem {
		font-family: var(--font-tamil);
		font-size: 1.25rem;
		line-height: 2;
		color: var(--c-brown-dark);
		margin-bottom: 1.5rem;
	}

	.preview-line {
		margin: 0;
		padding: 0.25rem 0;
	}

	.preview-translation {
		padding-top: 1rem;
		border-top: 1px solid rgba(92, 74, 61, 0.1);
		font-family: var(--font-serif);
		font-style: italic;
		color: var(--c-brown-light);
	}

	.preview-translation p {
		margin: 0;
	}

	/* Lines Editor */
	.lines-editor {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.field-label {
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-brown);
	}

	.lines-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.line-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.line-number {
		width: 24px;
		font-size: 0.75rem;
		color: var(--c-brown-muted);
		text-align: center;
		flex-shrink: 0;
	}

	.line-input {
		flex: 1;
		padding: 0.625rem 0.875rem;
		border: 1px solid rgba(92, 74, 61, 0.15);
		border-radius: var(--radius-sm);
		font-family: var(--font-tamil);
		font-size: 1rem;
		color: var(--c-brown-dark);
		transition: all var(--transition-fast);
	}

	.line-input:focus {
		outline: none;
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(184, 83, 61, 0.08);
	}

	.line-input::placeholder {
		color: var(--c-brown-subtle);
	}

	.remove-line-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid rgba(92, 74, 61, 0.1);
		background: white;
		border-radius: var(--radius-sm);
		color: var(--c-terracotta);
		cursor: pointer;
		font-size: 1.1rem;
		flex-shrink: 0;
		transition: all var(--transition-fast);
	}

	.remove-line-btn:hover:not(:disabled) {
		background: var(--c-error-bg);
		border-color: var(--c-terracotta);
	}

	.remove-line-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	/* Translation Editor */
	.translation-editor {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.translation-input {
		width: 100%;
		padding: 0.875rem;
		border: 1px solid rgba(92, 74, 61, 0.15);
		border-radius: var(--radius-md);
		font-family: var(--font-serif);
		font-size: 0.95rem;
		color: var(--c-brown);
		resize: vertical;
		min-height: 80px;
		transition: all var(--transition-fast);
	}

	.translation-input:focus {
		outline: none;
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(184, 83, 61, 0.08);
	}

	/* Navigation Footer */
	.verse-navigation-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--c-cream-dark);
		border-radius: var(--radius-md);
	}

	.nav-prev,
	.nav-next {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		font-size: 0.85rem;
		color: var(--c-terracotta);
		background: transparent;
		border: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.nav-prev:hover:not(:disabled),
	.nav-next:hover:not(:disabled) {
		background: white;
	}

	.nav-prev:disabled,
	.nav-next:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.verse-indicator {
		font-size: 0.85rem;
		color: var(--c-brown-muted);
		font-weight: 500;
	}

	/* Section Title */
	.section-title-editor {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.title-input {
		width: 100%;
		max-width: 400px;
		padding: 0.625rem 0.875rem;
		border: 1px solid rgba(92, 74, 61, 0.15);
		border-radius: var(--radius-md);
		font-family: var(--font-heading);
		font-size: 1rem;
		color: var(--c-brown-dark);
		transition: all var(--transition-fast);
	}

	.title-input:focus {
		outline: none;
		border-color: var(--c-terracotta);
		box-shadow: 0 0 0 3px rgba(184, 83, 61, 0.08);
	}

	.title-input::placeholder {
		color: var(--c-brown-subtle);
	}
</style>
