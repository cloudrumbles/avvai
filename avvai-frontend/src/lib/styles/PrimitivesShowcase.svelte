<script lang="ts">
	export let section: 'buttons' | 'inputs' | 'cards' | 'interactive' = 'buttons';
</script>

{#if section === 'buttons'}
	<div class="stack">
		<div class="row">
			<button class="btn btn-primary">Primary</button>
			<button class="btn btn-secondary">Secondary</button>
			<button class="btn btn-ghost">Ghost</button>
		</div>
		<div class="row">
			<button class="btn btn-primary" disabled>Primary Disabled</button>
			<button class="btn btn-secondary" disabled>Secondary Disabled</button>
		</div>
	</div>
{:else if section === 'inputs'}
	<div class="stack">
		<label class="stack">
			<span class="label">Default</span>
			<input class="input" placeholder="Type here" />
		</label>
		<label class="stack">
			<span class="label">Filled</span>
			<input class="input" value="Avvai" />
		</label>
	</div>
{:else if section === 'cards'}
	<div class="row">
		<div class="card">
			<div class="card-title">Base Card</div>
			<p class="card-body">Warm parchment tones and soft edges.</p>
			<span class="badge">Default</span>
		</div>
		<div class="card accent">
			<div class="card-title">Accent Card</div>
			<p class="card-body">Terracotta wash with subtle warmth.</p>
			<span class="badge alt">Accent</span>
		</div>
	</div>
{:else}
	<div class="row">
		<button class="btn-icon" aria-label="Bookmark">
			★
		</button>
		<span class="interactive-text">Hover me</span>
	</div>
{/if}

<style>
	.stack {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-3);
	}

	.label {
		font-size: var(--font-size-2);
		color: var(--color-text-subtle);
		letter-spacing: var(--letter-wide);
		text-transform: uppercase;
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--font-size-4);
		margin-bottom: var(--space-2);
	}

	.card-body {
		margin: 0 0 var(--space-3);
		color: var(--color-text-muted);
	}
</style>
