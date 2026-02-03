// Global dictionary popup state using Svelte 5 runes
// This module can be imported anywhere and the state is reactive across components

import type { DictionaryEntry } from '$lib/services/dictionary';

export interface PopupAnchor {
	x: number;
	y: number;
	bottom: number;
}

let word = $state('');
let anchor = $state<PopupAnchor>({ x: 0, y: 0, bottom: 0 });
let visible = $state(false);

let panelWord = $state('');
let panelEntry = $state<DictionaryEntry | null>(null);
let panelLoading = $state(false);
let panelVisible = $state(false);

export function showDictionary(newWord: string, newAnchor: PopupAnchor) {
	word = newWord;
	anchor = newAnchor;
	visible = true;
}

export function hideDictionary() {
	visible = false;
	word = '';
}

export function showDictionaryPanel(newWord: string) {
	panelWord = newWord;
	panelVisible = true;
}

export function hideDictionaryPanel() {
	panelVisible = false;
	panelWord = '';
	panelEntry = null;
}

export function setPanelEntry(entry: DictionaryEntry | null) {
	panelEntry = entry;
}

export function setPanelLoading(loading: boolean) {
	panelLoading = loading;
}

export function getDictionaryState() {
	return {
		get word() {
			return word;
		},
		get anchor() {
			return anchor;
		},
		get visible() {
			return visible;
		}
	};
}

export function getDictionaryPanelState() {
	return {
		get word() {
			return panelWord;
		},
		get entry() {
			return panelEntry;
		},
		get loading() {
			return panelLoading;
		},
		get visible() {
			return panelVisible;
		}
	};
}
