<script lang="ts">
	import { showDictionary } from 'avvai-frontend/stores/dictionary';

	interface Props {
		text: string;
		onwordclick?: (word: string, event: MouseEvent) => void;
	}

	let { text, onwordclick }: Props = $props();

	function handleClick(token: string, event: MouseEvent) {
		const clean = token.replace(/^[\s\p{P}]+|[\s\p{P}]+$/gu, '');
		if (!clean) return;

		if (onwordclick) {
			onwordclick(clean, event);
		} else {
			const rect = (event.target as HTMLElement).getBoundingClientRect();
			showDictionary(clean, {
				x: rect.left + rect.width / 2,
				y: rect.top,
				bottom: rect.bottom
			});
		}
	}

	function handleKeydown(token: string, event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleClick(token, event as unknown as MouseEvent);
		}
	}
</script>

{#each text.split(/(\s+)/) as token, i (i)}
	{#if /^\s+$/.test(token)}
		{token}
	{:else}
		<span
			class="clickable-word"
			role="button"
			tabindex="-1"
			onclick={(e) => handleClick(token, e)}
			onkeydown={(e) => handleKeydown(token, e)}
		>{token}</span>
	{/if}
{/each}

<style>
	.clickable-word {
		cursor: pointer;
		border-radius: 3px;
		transition: background 0.1s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.clickable-word:hover {
		background: var(--red-faint);
	}

	.clickable-word:active {
		background: rgba(139, 26, 26, 0.14);
	}
</style>
