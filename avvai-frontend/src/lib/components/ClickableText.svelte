<script lang="ts">
	import { cleanDictionaryToken, showDictionaryFromEvent } from '$lib/actions/dictionary-lookup';

	interface Props {
		text: string;
		onwordclick?: (word: string, event: MouseEvent) => void;
	}

	let { text, onwordclick }: Props = $props();

	let parts = $derived.by(() => {
		const tokens = text.split(/(\s+)/);
		let wordIndex = 0;
		return tokens.map((token) => {
			const isWhitespace = /^\s+$/.test(token);
			return {
				token,
				isWhitespace,
				wordIndex: isWhitespace ? -1 : wordIndex++
			};
		});
	});

	let wordCount = $derived.by(() => {
		let count = 0;
		for (const part of parts) {
			if (!part.isWhitespace) count += 1;
		}
		return count;
	});

	let activeIndex = $state(0);
	let wordElements = $state<(HTMLElement | undefined)[]>([]);
	let currentIndex = $derived.by(() => {
		if (wordCount === 0) return 0;
		return Math.max(0, Math.min(activeIndex, wordCount - 1));
	});

	function focusWord(index: number) {
		if (wordCount === 0) return;
		const clamped = Math.max(0, Math.min(index, wordCount - 1));
		activeIndex = clamped;
		const el = wordElements[clamped];
		if (el) el.focus();
	}

	function storeWordElement(index: number) {
		return (node: HTMLElement) => {
			wordElements[index] = node;
			return () => {
				if (wordElements[index] === node) {
					wordElements[index] = undefined;
				}
			};
		};
	}

	function handleClick(token: string, event: MouseEvent) {
		const clean = cleanDictionaryToken(token);
		if (!clean) return;

		if (onwordclick) {
			onwordclick(clean, event);
		} else {
			showDictionaryFromEvent(clean, event);
		}
	}

	function handleKeydown(token: string, wordIndex: number, event: KeyboardEvent) {
		switch (event.key) {
			case 'ArrowLeft':
			case 'ArrowUp':
				event.preventDefault();
				focusWord(wordIndex - 1);
				break;
			case 'ArrowRight':
			case 'ArrowDown':
				event.preventDefault();
				focusWord(wordIndex + 1);
				break;
			case 'Home':
				event.preventDefault();
				focusWord(0);
				break;
			case 'End':
				event.preventDefault();
				focusWord(wordCount - 1);
				break;
			case 'Enter':
			case ' ':
			case 'Spacebar':
				event.preventDefault();
				handleClick(token, event as unknown as MouseEvent);
				break;
		}
	}
</script>

{#each parts as part, i (i)}
	{#if part.isWhitespace}
		{part.token}
	{:else}
		<span
			class="clickable-word interactive-text"
			role="button"
			tabindex={part.wordIndex === currentIndex ? 0 : -1}
			{@attach storeWordElement(part.wordIndex)}
			onclick={(e) => {
				activeIndex = part.wordIndex;
				handleClick(part.token, e);
			}}
			onfocus={() => {
				activeIndex = part.wordIndex;
			}}
			onkeydown={(e) => handleKeydown(part.token, part.wordIndex, e)}
		>{part.token}</span>
	{/if}
{/each}
