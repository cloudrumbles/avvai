// Global dictionary popup state using Svelte 5 runes
// This module can be imported anywhere and the state is reactive across components

export interface PopupAnchor {
	x: number;
	y: number;
	bottom: number;
}

let word = $state('');
let anchor = $state<PopupAnchor>({ x: 0, y: 0, bottom: 0 });
let visible = $state(false);

export function showDictionary(newWord: string, newAnchor: PopupAnchor) {
	word = newWord;
	anchor = newAnchor;
	visible = true;
}

export function hideDictionary() {
	visible = false;
	word = '';
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
