<script lang="ts">
	import { lookup, type DictionaryEntry } from 'avvai-frontend/services/dictionary';
	import { getDictionaryState, hideDictionary } from 'avvai-frontend/stores/dictionary';

	const popup = getDictionaryState();

	let entry: DictionaryEntry | null = $state(null);
	let loading = $state(false);
	let popupEl: HTMLElement | undefined = $state(undefined);
	let style = $state('');

	const PAD = 8;

	function reposition() {
		if (!popupEl) return;
		const pw = popupEl.offsetWidth;
		const ph = popupEl.offsetHeight;
		const vw = window.innerWidth;
		const vh = window.innerHeight;

		let x = popup.anchor.x - pw / 2;
		let y = popup.anchor.y - ph - PAD;

		if (y < PAD) {
			y = popup.anchor.bottom + PAD;
		}

		x = Math.max(PAD, Math.min(x, vw - pw - PAD));
		y = Math.max(PAD, Math.min(y, vh - ph - PAD));

		style = `left:${x}px;top:${y}px;`;
	}

	$effect(() => {
		if (!popup.visible || !popup.word) return;

		loading = true;
		entry = null;
		requestAnimationFrame(reposition);

		lookup(popup.word).then((result) => {
			entry = result;
			loading = false;
			requestAnimationFrame(reposition);
		});
	});
</script>

{#if popup.visible}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="popup-overlay" onclick={hideDictionary} onkeydown={(e) => e.key === 'Escape' && hideDictionary()}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="popup"
			bind:this={popupEl}
			{style}
			onclick={(e) => e.stopPropagation()}
		>
			<p class="popup-word">{popup.word}</p>
			{#if loading}
				<p class="popup-body">Looking up…</p>
			{:else if entry}
				<p class="popup-body">{entry.definition}</p>
				{#if entry.examples?.length}
					{#each entry.examples as ex, ei (ei)}
						<p class="popup-example">{ex}</p>
					{/each}
				{/if}
			{:else}
				<p class="popup-body">No definition found.</p>
			{/if}
		</div>
	</div>
{/if}

<style>
	.popup-overlay {
		position: fixed;
		inset: 0;
		z-index: 100;
	}

	.popup {
		position: fixed;
		width: 280px;
		background: var(--color-bg);
		border: var(--border-strong);
		border-radius: var(--radius-3);
		padding: var(--space-3) var(--space-4);
		box-shadow: var(--shadow-2);
		z-index: 101;
	}

	.popup-word {
		font-family: var(--font-sans);
		font-size: var(--font-size-3);
		font-weight: 700;
		color: var(--color-accent-strong);
		margin: 0 0 var(--space-1);
	}

	.popup-body {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text);
		margin: 0;
		line-height: var(--line-height-3);
	}

	.popup-example {
		font-family: var(--font-sans);
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
		margin: var(--space-2) 0 0;
		line-height: var(--line-height-2);
		font-style: italic;
	}
</style>