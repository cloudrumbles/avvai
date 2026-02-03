import { showDictionary, showDictionaryPanel } from '$lib/stores/dictionary.svelte';
import type { PopupAnchor } from '$lib/stores/dictionary.svelte';

const PANEL_BREAKPOINT = 1024;

export function cleanDictionaryToken(token: string) {
	return token.replace(/^[\s\p{P}]+|[\s\p{P}]+$/gu, '');
}

export function anchorFromElement(element: HTMLElement | null): PopupAnchor | null {
	if (!element) return null;
	const rect = element.getBoundingClientRect();
	return {
		x: rect.left + rect.width / 2,
		y: rect.top,
		bottom: rect.bottom
	};
}

export function showDictionaryAtElement(word: string, element: HTMLElement | null) {
	const anchor = anchorFromElement(element);
	if (!anchor) return;
	showDictionary(word, anchor);
}

export function showDictionaryFromEvent(word: string, event: Event) {
	const element = event.currentTarget instanceof HTMLElement ? event.currentTarget : null;
	const anchor = anchorFromElement(element);
	if (!anchor) return;

	if (window.innerWidth >= PANEL_BREAKPOINT) {
		showDictionaryPanel(word);
	} else {
		showDictionary(word, anchor);
	}
}
