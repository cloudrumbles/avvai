<script lang="ts">
	import '$lib/styles/tokens.css';
	import '$lib/styles/primitives.css';
	import favicon from '$lib/assets/favicon.svg';
	import DictionaryPopup from '$lib/components/DictionaryPopup.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Catamaran:wght@400;500;600;700&family=Cormorant+Garamond:ital,wght@0,300;0,400;0,600;1,400&family=Eczar:wght@400;600;700&family=Tiro+Tamil:ital@0;1&display=swap" rel="stylesheet" />
</svelte:head>

{@render children()}
<DictionaryPopup />

<style>
	:global(html, body) {
		margin: 0;
		padding: 0;
		background: var(--color-bg);
		color: var(--color-text);
		font-family: var(--font-sans);
		line-height: var(--line-height-3);
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	:global(*) {
		box-sizing: border-box;
	}
</style>