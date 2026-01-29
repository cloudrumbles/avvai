<script lang="ts">
	let mounted = $state(false);

	const landscapes = [
		{
			name: 'Kurinji',
			theme: 'Midnight • Union',
			color: '#1a1f3c',
			accent: '#4a5b8c',
			text: 'white',
			poem: 'Dark as the mountain roots, our love is deeper than the earth.',
			details: ['JACKFRUIT, BAMBOO', 'TIGER, ELEPHANT', 'THE YOUTH', 'COOL, DEWY']
		},
		{
			name: 'Mullai',
			theme: 'Evening • Waiting',
			color: '#2d3b2a',
			accent: '#5e7054',
			text: '#e8ecd6',
			poem: 'The jasmine buds open at dusk. I wait for his chariot\'s sound.',
			details: ['JASMINE, KONRAI', 'DEER, HARE', 'THE DARK ONE', 'RAINY']
		},
		{
			name: 'Marutham',
			theme: 'Morning • Domesticity',
			color: '#e3dcd2',
			accent: '#8c4b31',
			text: '#2c1810',
			poem: 'Red soil of the river banks, where the crane sleeps in the paddy.',
			details: ['LOTUS, MANGO', 'BUFFALO, CRANE', 'THE KING', 'ALL SEASONS']
		},
		{
			name: 'Neithal',
			theme: 'Sunset • Pining',
			color: '#8e9aaf',
			accent: '#2b4f60',
			text: '#f0f4f8',
			poem: 'The seagulls cry over the salt-pans. The waves erase his footprints.',
			details: ['WATER LILY, PUNNAI', 'SHARK, SEAGULL', 'THE SEA', 'ALL SEASONS']
		},
		{
			name: 'Paalai',
			theme: 'Noon • Separation',
			color: '#2a231f',
			accent: '#8c5e45',
			text: '#d4a373',
			poem: 'The path is harsh with sharp stones. Even the shadow dies in this heat.',
			details: ['CACTUS, PALMYRA', 'WOLF, LIZARD', 'THE MOTHER', 'SUMMER']
		}
	];

	$effect(() => {
		mounted = true;
	});
</script>

<svelte:head>
	<title>avvai — Learn Tamil</title>
	<meta name="description" content="Learn Tamil through classical literature" />
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,600;1,400&family=Eczar:wght@400;600&family=Noto+Serif+Tamil:wght@400;500;600&display=swap" rel="stylesheet" />
</svelte:head>

<div class="page" class:mounted>
	<div class="texture-overlay"></div>

	<header class="site-header">
		<div class="era-label">Sangam Era</div>
		<div class="era-dates">600 BCE — 300 CE</div>
	</header>

	<section id="hero" class="hero-section">
		<div class="hero-bg">
			<svg viewBox="0 0 1000 1000" preserveAspectRatio="xMidYMid slice">
				<path d="M100,100 Q400,50 700,300 T900,800" stroke="currentColor" stroke-width="1" fill="none" class="draw-path"/>
				<path d="M-50,500 Q300,400 600,600 T1100,500" stroke="currentColor" stroke-width="1" fill="none" class="draw-path"/>
				<circle cx="800" cy="200" r="50" stroke="currentColor" stroke-width="1" fill="none" class="draw-circle"/>
			</svg>
		</div>

		<div class="hero-content">
			<div class="brahmi-chars">
				<svg width="32" height="32" viewBox="0 0 50 50" class="brahmi-char">
					<path d="M10 40 C 10 10, 40 10, 40 40" />
					<line x1="25" y1="10" x2="25" y2="35" />
				</svg>
				<svg width="32" height="32" viewBox="0 0 50 50" class="brahmi-char">
					<circle cx="25" cy="25" r="15" />
					<line x1="10" y1="25" x2="40" y2="25" />
				</svg>
				<svg width="32" height="32" viewBox="0 0 50 50" class="brahmi-char">
					<path d="M10 10 L 25 40 L 40 10" />
				</svg>
			</div>
			
			<h1 class="hero-title">
				<span>Thinai</span>
			</h1>
			<p class="hero-subtitle">
				The earth, the soil, and the season of the heart.
			</p>
			<div class="scroll-hint">
				<span>Scroll to explore</span>
			</div>
		</div>
	</section>

	<section id="landscapes" class="landscapes-section">
		<div class="landscapes-intro">
			<h2>The Five Landscapes</h2>
			<p>Ancient Tamil poets mapped human emotion onto the natural world.</p>
		</div>

		<div class="landscapes-list">
			{#each landscapes as landscape, i (landscape.name)}
				<article 
					class="landscape-card"
					style="background-color: {landscape.color}; color: {landscape.text};"
				>
					<div class="landscape-visual" style="background-color: {landscape.accent}">
						{#if i === 0}
							<svg viewBox="0 0 100 100" preserveAspectRatio="none">
								<path d="M0 100 L 30 40 L 60 100 L 80 60 L 100 100 Z" fill="rgba(255,255,255,0.1)"/>
							</svg>
						{:else if i === 1}
							<svg viewBox="0 0 100 100" preserveAspectRatio="none">
								<path d="M0 50 Q 25 20 50 50 T 100 50" fill="none" stroke="rgba(255,255,255,0.2)" stroke-width="2"/>
								<circle cx="80" cy="30" r="15" fill="rgba(255,255,255,0.1)"/>
							</svg>
						{:else if i === 2}
							<svg viewBox="0 0 100 100" preserveAspectRatio="none">
								<line x1="0" y1="80" x2="100" y2="80" stroke="rgba(0,0,0,0.1)" stroke-width="2"/>
								<line x1="0" y1="90" x2="100" y2="90" stroke="rgba(0,0,0,0.1)" stroke-width="2"/>
							</svg>
						{:else if i === 3}
							<svg viewBox="0 0 100 100" preserveAspectRatio="none">
								<path d="M0 60 Q 25 40 50 60 T 100 60 V 100 H 0 Z" fill="rgba(255,255,255,0.1)"/>
							</svg>
						{:else}
							<svg viewBox="0 0 100 100" preserveAspectRatio="none">
								<path d="M10 10 L 30 30 L 20 60 L 50 80 M 60 10 L 80 40 L 70 90" stroke="rgba(255,255,255,0.2)" stroke-width="2" fill="none"/>
							</svg>
						{/if}
					</div>
					
					<div class="landscape-content">
						<div class="landscape-meta" style="color: {landscape.accent}">
							{landscape.theme}
						</div>
						<h3 class="landscape-name">{landscape.name}</h3>
						<p class="landscape-poem">{landscape.poem}</p>
						<div class="landscape-details">
							<span>{landscape.details[0]}</span>
							<span>{landscape.details[1]}</span>
							<span>{landscape.details[3]}</span>
						</div>
					</div>
				</article>
			{/each}
		</div>
	</section>

	<section id="poetry" class="poetry-section">
		<div class="poetry-content">
			<div class="palm-leaf">
				<div class="palm-leaf-inner">
					<div class="poetry-symbol">
						<svg width="48" height="48" viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="3">
							<path d="M20 50 Q 50 20 80 50" />
							<path d="M20 60 Q 50 90 80 60" />
							<circle cx="50" cy="50" r="5" fill="currentColor"/>
						</svg>
					</div>

					<blockquote class="poetry-verses">
						<p class="verse">What is my mother to yours? What is my father to yours? And how did we two ever meet?</p>
						<div class="verse-divider"></div>
						<p class="verse secondary">Like rain falling on red earth, our loving hearts have mingled into one.</p>
					</blockquote>

					<cite class="verse-attribution">
						Kurunthogai 40
					</cite>
				</div>
			</div>
		</div>
	</section>

	<section class="cta-section">
		<div class="cta-content">
			<h2 class="cta-headline">
				Your roots are calling.
			</h2>
			<p class="cta-subheadline">
				Learn Tamil to deepen your connection to the language, the poetry, and the land that shaped a civilization.
			</p>
			<a href="/home" class="cta-button">
				<span class="cta-tamil">தொடங்கு</span>
				<span class="cta-divider"></span>
				<span class="cta-english">Deepen Your Roots</span>
			</a>
		</div>
	</section>

	<footer class="site-footer">
		<div class="footer-grid"></div>
		<div class="footer-content">
			<div class="footer-brand">
				<h3>avvai</h3>
				<p>Learn Tamil through the poetry of the Sangam era.</p>
			</div>
			<nav class="footer-nav">
				<a href="#hero">Thinai</a>
				<a href="#poetry">Poetry</a>
				<a href="/home">Begin</a>
			</nav>
		</div>
		<div class="footer-copyright">
			<p>600 BCE — Present</p>
		</div>
	</footer>
</div>

<style>
	:global(:root) {
		--font-serif: 'Cormorant Garamond', serif;
		--font-heading: 'Eczar', serif;
		--font-tamil: 'Noto Serif Tamil', serif;
	}

	.page {
		background: #f3f0e9;
		color: #1a1a1a;
		font-family: var(--font-serif);
	}

	.texture-overlay {
		position: fixed;
		inset: 0;
		pointer-events: none;
		z-index: 100;
		mix-blend-mode: multiply;
		background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 400 400' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)' opacity='0.08'/%3E%3C/svg%3E");
	}

	.site-header {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		padding: 1rem 1.25rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		z-index: 40;
		mix-blend-mode: exclusion;
		color: rgba(255, 255, 255, 0.9);
		font-family: var(--font-heading);
	}

	.era-label {
		font-size: 0.75rem;
		letter-spacing: 0.2em;
		text-transform: uppercase;
	}

	.era-dates {
		font-size: 0.65rem;
		letter-spacing: 0.05em;
		opacity: 0.7;
	}

	.hero-section {
		min-height: 100dvh;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		background: #1c1c1c;
		color: #e0e0e0;
		overflow: hidden;
		padding: 1.5rem;
	}

	.hero-bg {
		position: absolute;
		inset: 0;
		opacity: 0.1;
	}

	.hero-bg svg {
		width: 100%;
		height: 100%;
	}

	.draw-path,
	.draw-circle {
		stroke-dasharray: 1000;
		stroke-dashoffset: 1000;
		animation: drawLine 3s ease-in-out infinite alternate;
	}

	@keyframes drawLine {
		to { stroke-dashoffset: 0; }
	}

	.hero-content {
		text-align: center;
		position: relative;
		z-index: 10;
	}

	.brahmi-chars {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		opacity: 0;
		transform: translateY(20px);
		transition: opacity 0.8s ease, transform 0.8s ease;
	}

	.mounted .brahmi-chars {
		opacity: 0.7;
		transform: translateY(0);
	}

	.brahmi-char {
		stroke: #a8a8a8;
		fill: none;
		stroke-width: 2;
		stroke-linecap: round;
	}

	.hero-title {
		font-family: var(--font-heading);
		font-size: clamp(3rem, 15vw, 6rem);
		font-weight: 700;
		color: #d4c5b0;
		margin: 0 0 1rem;
		line-height: 1;
		letter-spacing: -0.02em;
	}

	.hero-title span {
		display: inline-block;
		opacity: 0;
		transform: translateY(50px);
		transition: opacity 1.5s cubic-bezier(0.22, 1, 0.36, 1), 
					transform 1.5s cubic-bezier(0.22, 1, 0.36, 1);
	}

	.mounted .hero-title span {
		opacity: 1;
		transform: translateY(0);
	}

	.hero-subtitle {
		font-size: clamp(1rem, 4vw, 1.25rem);
		font-weight: 300;
		font-style: italic;
		letter-spacing: 0.02em;
		color: #a8a8a8;
		margin: 0 0 2rem;
		opacity: 0;
		transform: translateY(20px);
		transition: opacity 1.5s ease 0.7s, transform 1.5s ease 0.7s;
	}

	.mounted .hero-subtitle {
		opacity: 1;
		transform: translateY(0);
	}

	.scroll-hint {
		opacity: 0;
		animation: fadeInUp 1s ease 1.5s forwards, bounce 2s ease-in-out 2.5s infinite;
	}

	@keyframes fadeInUp {
		to { opacity: 1; transform: translateY(-5px); }
	}

	@keyframes bounce {
		0%, 100% { transform: translateY(-5px); }
		50% { transform: translateY(-15px); }
	}

	.scroll-hint span {
		font-size: 0.65rem;
		letter-spacing: 0.3em;
		text-transform: uppercase;
		border-bottom: 1px solid rgba(255, 255, 255, 0.2);
		padding-bottom: 0.25rem;
	}

	.landscapes-section {
		padding: 4rem 1.25rem;
		background: #f3f0e9;
	}

	.landscapes-intro {
		text-align: center;
		margin-bottom: 3rem;
	}

	.landscapes-intro h2 {
		font-family: var(--font-heading);
		font-size: clamp(1.75rem, 6vw, 2.5rem);
		color: #1a1a1a;
		margin: 0 0 0.75rem;
	}

	.landscapes-intro p {
		font-size: 1rem;
		color: #666;
		margin: 0;
		font-style: italic;
	}

	.landscapes-list {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		max-width: 600px;
		margin: 0 auto;
	}

	.landscape-card {
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
	}

	.landscape-visual {
		height: 120px;
		position: relative;
		opacity: 0.8;
	}

	.landscape-visual svg {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}

	.landscape-content {
		padding: 1.5rem;
	}

	.landscape-meta {
		font-family: var(--font-heading);
		font-size: 0.7rem;
		font-weight: 600;
		letter-spacing: 0.15em;
		text-transform: uppercase;
		margin-bottom: 0.5rem;
		opacity: 0.9;
	}

	.landscape-name {
		font-family: var(--font-heading);
		font-size: clamp(1.75rem, 6vw, 2.25rem);
		margin: 0 0 0.75rem;
		line-height: 1;
	}

	.landscape-poem {
		font-size: 1rem;
		line-height: 1.6;
		font-style: italic;
		opacity: 0.85;
		margin: 0 0 1rem;
	}

	.landscape-details {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		font-family: var(--font-heading);
		font-size: 0.65rem;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		opacity: 0.6;
	}

	.landscape-details span {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.poetry-section {
		padding: 4rem 1.25rem;
		background: linear-gradient(to bottom, #2a231f 0%, #e8e4d9 20%);
	}

	.poetry-content {
		max-width: 600px;
		margin: 0 auto;
	}

	.palm-leaf {
		background: #d4cbb8;
		padding: 2rem 1.5rem;
		border-radius: 8px;
		position: relative;
		transform: rotate(-0.5deg);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.12);
	}

	.palm-leaf::before {
		content: '';
		position: absolute;
		inset: 0;
		border-top: 2px solid #b8ae9a;
		border-bottom: 2px solid #b8ae9a;
		border-radius: 8px;
		pointer-events: none;
	}

	.palm-leaf-inner {
		transform: rotate(0.5deg);
		text-align: center;
	}

	.poetry-symbol {
		color: #5c5c5c;
		margin-bottom: 1.5rem;
		display: flex;
		justify-content: center;
	}

	.poetry-verses {
		margin: 0;
		padding: 0;
		border: none;
	}

	.verse {
		font-size: clamp(1.1rem, 4vw, 1.5rem);
		line-height: 1.7;
		margin: 0;
		font-weight: 400;
		color: #2c2c2c;
	}

	.verse.secondary {
		font-size: clamp(1rem, 3.5vw, 1.25rem);
		opacity: 0.8;
	}

	.verse-divider {
		width: 60%;
		height: 1px;
		background: #8c8c8c;
		opacity: 0.3;
		margin: 1.5rem auto;
	}

	.verse-attribution {
		display: block;
		margin-top: 1.5rem;
		font-family: var(--font-heading);
		font-size: 0.8rem;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		opacity: 0.6;
		font-style: normal;
		color: #5c5c5c;
	}

	.cta-section {
		padding: 4rem 1.5rem;
		background: #1a1a1a;
		color: #e0e0e0;
		text-align: center;
	}

	.cta-content {
		max-width: 500px;
		margin: 0 auto;
	}

	.cta-headline {
		font-family: var(--font-heading);
		font-size: clamp(1.75rem, 6vw, 2.25rem);
		margin: 0 0 1rem;
		line-height: 1.2;
		color: #d4c5b0;
	}

	.cta-subheadline {
		font-size: 1rem;
		line-height: 1.7;
		color: #a8a8a8;
		margin: 0 0 2rem;
		font-weight: 300;
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.75rem;
		background: #c45c3e;
		color: white;
		text-decoration: none;
		border-radius: 6px;
		font-family: var(--font-heading);
		font-size: 0.9rem;
		transition: background 0.3s, transform 0.3s, box-shadow 0.3s;
		box-shadow: 0 4px 20px rgba(196, 92, 62, 0.3);
	}

	.cta-button:hover {
		background: #d4684a;
		transform: translateY(-2px);
		box-shadow: 0 6px 28px rgba(196, 92, 62, 0.4);
	}

	.cta-tamil {
		font-family: var(--font-tamil);
		font-size: 1.1rem;
		font-weight: 600;
	}

	.cta-divider {
		width: 1px;
		height: 18px;
		background: rgba(255, 255, 255, 0.3);
	}

	.cta-english {
		font-size: 0.7rem;
		letter-spacing: 0.12em;
		text-transform: uppercase;
	}

	.site-footer {
		background: #1a1a1a;
		color: #888;
		padding: 3rem 1.5rem 1.5rem;
		position: relative;
		overflow: hidden;
	}

	.footer-grid {
		position: absolute;
		inset: 0;
		opacity: 0.05;
		background-image: 
			linear-gradient(to right, white 1px, transparent 1px),
			linear-gradient(to bottom, white 1px, transparent 1px);
		background-size: 40px 40px;
	}

	.footer-content {
		position: relative;
		z-index: 10;
		text-align: center;
	}

	.footer-brand h3 {
		font-family: var(--font-heading);
		font-size: 1.25rem;
		color: #c27e60;
		margin: 0 0 0.5rem;
		letter-spacing: 0.1em;
	}

	.footer-brand p {
		font-size: 0.85rem;
		line-height: 1.5;
		margin: 0 0 1.5rem;
		opacity: 0.7;
	}

	.footer-nav {
		display: flex;
		justify-content: center;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.footer-nav a {
		color: inherit;
		text-decoration: none;
		font-size: 0.8rem;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		transition: color 0.3s;
	}

	.footer-nav a:hover {
		color: #c27e60;
	}

	.footer-copyright {
		text-align: center;
		font-size: 0.7rem;
		opacity: 0.5;
		position: relative;
		z-index: 10;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	@media (min-width: 768px) {
		.landscapes-section {
			padding: 6rem 2rem;
		}

		.landscapes-list {
			max-width: 700px;
		}

		.landscape-card {
			display: grid;
			grid-template-columns: 200px 1fr;
		}

		.landscape-visual {
			height: auto;
		}

		.landscape-content {
			padding: 2rem;
		}

		.poetry-section {
			padding: 6rem 2rem;
		}

		.cta-section {
			padding: 6rem 2rem;
		}

		.cta-content {
			max-width: 600px;
		}
	}

	:global(::-webkit-scrollbar) {
		width: 6px;
	}

	:global(::-webkit-scrollbar-track) {
		background: #f3f0e9;
	}

	:global(::-webkit-scrollbar-thumb) {
		background: #8c4b31;
		border-radius: 3px;
	}
</style>
