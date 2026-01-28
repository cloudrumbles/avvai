<script lang="ts">
	import Reader from '$lib/components/Reader.svelte';
	import sampleText from '$lib/content/thirukkural.txt?raw';

	const readerTitle = 'திருக்குறள்';
</script>

<svelte:head>
	<title>avvai</title>
	<meta name="description" content="tamil language learning" />
</svelte:head>

<div class="page">
	<Reader text={sampleText} title={readerTitle} />
</div>

<style>
	:global(html, body) {
		margin: 0;
		padding: 0;
		height: 100%;
		overflow: hidden;
		background: #faf3e6;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	.page {
		height: 100dvh;
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
