<script lang="ts">
	import { lookup, type DictionaryEntry } from '$lib/services/dictionary';

	interface Props {
		word: string;
		anchor: { x: number; y: number; bottom: number };
		visible: boolean;
		onclose: () => void;
	}

	let { word, anchor, visible, onclose }: Props = $props();

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

		let x = anchor.x - pw / 2;
		let y = anchor.y - ph - PAD;

		if (y < PAD) {
			y = anchor.bottom + PAD;
		}

		x = Math.max(PAD, Math.min(x, vw - pw - PAD));
		y = Math.max(PAD, Math.min(y, vh - ph - PAD));

		style = `left:${x}px;top:${y}px;`;
	}

	$effect(() => {
		if (!visible || !word) return;

		loading = true;
		entry = null;
		requestAnimationFrame(reposition);

		lookup(word).then((result) => {
			entry = result;
			loading = false;
			requestAnimationFrame(reposition);
		});
	});
</script>

{#if visible}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="popup-overlay" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="popup"
			bind:this={popupEl}
			{style}
			onclick={(e) => e.stopPropagation()}
		>
			<p class="popup-word">{word}</p>
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
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		padding: 12px 16px;
		box-shadow: 0 8px 32px rgba(26, 14, 6, 0.18);
		z-index: 101;
	}

	.popup-word {
		font-family: 'Catamaran', sans-serif;
		font-size: 16px;
		font-weight: 700;
		color: var(--red-deep);
		margin: 0 0 4px;
	}

	.popup-body {
		font-family: 'Catamaran', sans-serif;
		font-size: 14px;
		color: var(--ink);
		margin: 0;
		line-height: 1.5;
	}

	.popup-example {
		font-family: 'Catamaran', sans-serif;
		font-size: 13px;
		color: var(--ink-soft);
		margin: 6px 0 0;
		line-height: 1.4;
		font-style: italic;
	}
</style>
