<script lang="ts">
	let fontSize = $state(24);

	const fonts = [
		{ name: 'Noto Serif Tamil', family: "'Noto Serif Tamil', serif", category: 'serif' },
		{ name: 'Noto Sans Tamil', family: "'Noto Sans Tamil', sans-serif", category: 'sans-serif' },
		{ name: 'Tiro Tamil', family: "'Tiro Tamil', serif", category: 'serif' },
		{ name: 'Catamaran', family: "var(--font-sans)", category: 'sans-serif' },
		{ name: 'Anek Tamil', family: "'Anek Tamil', sans-serif", category: 'sans-serif' },
		{ name: 'Hind Madurai', family: "'Hind Madurai', sans-serif", category: 'sans-serif' },
		{ name: 'Mukta Malar', family: "'Mukta Malar', sans-serif", category: 'sans-serif' },
		{ name: 'Pavanam', family: "'Pavanam', sans-serif", category: 'sans-serif' },
		{ name: 'Meera Inimai', family: "'Meera Inimai', sans-serif", category: 'sans-serif' },
		{ name: 'Baloo Thambi 2', family: "'Baloo Thambi 2', cursive", category: 'display' },
		{ name: 'Kavivanar', family: "'Kavivanar', cursive", category: 'display' },
		{ name: 'Arima', family: "'Arima', system-ui", category: 'display' },
		{ name: 'Ponnala', family: "'Ponnala', serif", category: 'serif' }
	];

	const sampleText =
		'திருக்குறள், சுருக்கமாக குறள், ஒரு தொன்மையான தமிழ் மொழி அற இலக்கியமாகும். சங்க இலக்கிய வகைப்பாட்டில் பதினெண்கீழ்க்கணக்கு எனப்படும் பதினெட்டு நூல்களின் திரட்டில் இருக்கும் இந்நூல் குறள் வெண்பா என்னும் பாவகையினாலான 1,330 ஈரடிச் செய்யுள்களைக் கொண்டது.';

	const googleFontsUrl = $derived(
		'https://fonts.googleapis.com/css2?' +
			fonts.map((f) => `family=${f.name.replace(/ /g, '+')}`).join('&') +
			'&display=swap'
	);
</script>

<svelte:head>
	<title>எழுத்துரு காட்சியகம் — Avvai</title>
	<link rel="stylesheet" href={googleFontsUrl} />
</svelte:head>

<div class="page">
	<header class="page-header">
		<a href="/" class="back-link">← முகப்பு</a>
		<h1 class="page-title">எழுத்துரு காட்சியகம்</h1>
		<p class="page-subtitle">Font Gallery</p>
	</header>

	<div class="controls">
		<label class="slider-label" for="font-size-slider">
			எழுத்து அளவு / Font size: <strong>{fontSize}px</strong>
		</label>
		<input
			id="font-size-slider"
			type="range"
			min="16"
			max="48"
			step="2"
			bind:value={fontSize}
			class="font-slider"
		/>
	</div>

	<div class="grid">
		{#each fonts as font (font.name)}
			<article class="card">
				<div class="card-header">
					<h2 class="card-font-name" style:font-family={font.family}>{font.name}</h2>
					<span class="card-category">{font.category}</span>
				</div>
				<p class="card-sample" style:font-family={font.family} style:font-size="{fontSize}px">
					{sampleText}
				</p>
			</article>
		{/each}
	</div>
</div>

<style>
	:global(html),
	:global(body) {
		overflow: auto;
		height: auto;
	}

	.page {
		background: var(--color-bg);
		min-height: 100dvh;
		padding: var(--space-7) var(--space-6) 64px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: var(--space-7);
	}

	.back-link {
		display: inline-block;
		font-family: 'Noto Sans Tamil', sans-serif;
		font-size: var(--font-size-2);
		color: var(--color-accent);
		text-decoration: none;
		margin-bottom: var(--space-4);
		padding: var(--space-1-5) var(--space-3);
		border-radius: var(--radius-1-5);
		transition: background 0.15s ease;
	}

	.back-link:hover {
		background: var(--color-accent-tint);
	}

	.page-title {
		font-family: 'Noto Serif Tamil', serif;
		font-size: var(--font-size-8);
		color: var(--color-accent-strong);
		margin: 0 0 var(--space-1);
		font-weight: 700;
	}

	.page-subtitle {
		font-family: 'Noto Sans Tamil', sans-serif;
		font-size: var(--font-size-2);
		color: var(--color-text-subtle);
		margin: 0;
		text-transform: uppercase;
		letter-spacing: 1.5px;
	}

	.controls {
		background: var(--color-bg);
		border: 1.5px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-5) var(--space-6);
		margin-bottom: var(--space-7);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.slider-label {
		font-family: 'Noto Sans Tamil', sans-serif;
		font-size: var(--font-size-2);
		color: var(--color-text-muted);
	}

	.slider-label strong {
		color: var(--color-accent);
		font-weight: 600;
	}

	.font-slider {
		-webkit-appearance: none;
		appearance: none;
		width: 100%;
		height: 6px;
		border-radius: var(--radius-0-75);
		background: var(--color-bg-soft);
		outline: none;
	}

	.font-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--color-accent);
		cursor: pointer;
		border: 3px solid var(--color-bg);
		box-shadow: var(--shadow-1);
		transition: transform 0.15s ease;
	}

	.font-slider::-webkit-slider-thumb:hover {
		transform: scale(1.15);
	}

	.font-slider::-moz-range-thumb {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--color-accent);
		cursor: pointer;
		border: 3px solid var(--color-bg);
		box-shadow: var(--shadow-1);
		transition: transform 0.15s ease;
	}

	.font-slider::-moz-range-thumb:hover {
		transform: scale(1.15);
	}

	.font-slider::-moz-range-track {
		height: 6px;
		border-radius: var(--radius-0-75);
		background: var(--color-bg-soft);
	}

	.grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-6);
	}

	@media (min-width: 768px) {
		.grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1200px) {
		.grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.card {
		background: var(--color-bg);
		border: 1.5px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-6);
		box-shadow: var(--shadow-1);
		transition: box-shadow 0.2s ease;
	}

	.card:hover {
		box-shadow: var(--shadow-2);
	}

	.card-header {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: var(--space-3);
		margin-bottom: var(--space-4);
		padding-bottom: var(--space-3);
		border-bottom: 1px solid var(--color-bg-soft);
	}

	.card-font-name {
		font-size: var(--font-size-4);
		color: var(--color-text);
		margin: 0;
		font-weight: 600;
	}

	.card-category {
		font-family: 'Noto Sans Tamil', sans-serif;
		font-size: var(--font-size-0-75);
		color: var(--color-text-subtle);
		text-transform: uppercase;
		letter-spacing: 1px;
		white-space: nowrap;
		padding: var(--space-0-5) var(--space-2);
		background: var(--color-bg-soft);
		border-radius: var(--radius-1);
		font-weight: 500;
	}

	.card-sample {
		color: var(--color-text);
		line-height: var(--line-height-1-8);
		margin: 0;
	}
</style>