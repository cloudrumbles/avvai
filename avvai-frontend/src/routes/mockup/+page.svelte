<script lang="ts">
	let scrollY = $state(0);
	let innerHeight = $state(0);
	let mounted = $state(false);

	let heroTranslate = $derived(scrollY * 0.4);
	let heroScale = $derived(1 + scrollY * 0.0003);
	let textOpacity = $derived(Math.max(0, 1 - scrollY / 500));
	let textTranslate = $derived(scrollY * 0.2);

	$effect(() => {
		mounted = true;
	});
</script>

<svelte:window bind:scrollY bind:innerHeight />

<svelte:head>
	<title>avvai — Come Home to Tamil</title>
	<meta name="description" content="Reconnect with your Tamil roots through the poetry of your ancestors" />
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,600;1,400&family=Eczar:wght@400;600;700&family=Noto+Serif+Tamil:wght@400;500;600&display=swap" rel="stylesheet" />
</svelte:head>

<div class="page" class:mounted>
	<div class="texture-overlay"></div>

	<header class="site-header">
		<div class="logo">avvai</div>
		<nav class="nav">
			<a href="/home" class="nav-cta">Start Learning</a>
		</nav>
	</header>

	<!-- Hero: The Emotional Hook -->
	<section class="hero">
		<div class="hero-image-container">
			<div 
				class="hero-image-wrapper"
				style:transform="translate3d(0, {heroTranslate}px, 0) scale({heroScale})"
			>
				<img 
					src="/marutham-000.png" 
					alt="A Tamil village - the world your ancestors came from" 
					class="hero-image"
				/>
			</div>
			<div class="hero-gradient"></div>
		</div>

		<div 
			class="hero-content"
			style:opacity={textOpacity}
			style:transform="translate3d(0, {textTranslate}px, 0)"
		>
			<h1 class="hero-title">
				Read Tamil.<br/>
				Finally.
			</h1>
			<p class="hero-subtitle">
				Classical literature, made accessible.
			</p>
			<a href="/home" class="hero-cta">
				<span class="cta-tamil">தொடங்கு</span>
				<span class="cta-divider"></span>
				<span class="cta-english">Start Reading</span>
			</a>
		</div>

		<div class="hero-transition">
			<svg viewBox="0 0 1440 120" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg">
				<path d="M0,120 C360,60 1080,60 1440,120 L1440,120 L0,120 Z" fill="var(--c-cream)"/>
			</svg>
		</div>
	</section>

	<section class="section showcase-section">
		<div class="section-content">
			<div class="interface-preview">
				<div class="poem-card">
					<p class="poem-line">
						கற்றது கைம்மண் <span class="interactive-word">அளவு</span> கல்லாதது
					</p>
					<p class="poem-line">
						உலகளவு என்று உற்று
					</p>
					
					<div class="definition-popover">
						<div class="def-header">
							<span class="def-word">அளவு</span>
							<span class="def-translit">aḷavu</span>
						</div>
						<div class="def-body">
							measure; amount; size
						</div>
						<div class="def-meta">noun</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<footer class="site-footer">
		<div class="footer-content">
			<div class="footer-brand">
				<span class="footer-logo">avvai</span>
				<p>Named after Avvaiyar—the poet who made Tamil wisdom accessible to everyone.</p>
			</div>

			<div class="footer-attribution">
				<div class="attribution-row">
					<img 
						src="https://www.tamilvu.org/sites/default/files/kanithamizh_logo%20%282%29_64_0.png" 
						alt="Tamil Virtual Academy" 
						class="tva-logo-footer"
					/>
					<p class="attribution-text">
						Data from <a href="https://www.tamilvu.org/" target="_blank" rel="noopener noreferrer">Tamil Virtual Academy</a><br/>
						Licensed under <a href="https://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="noopener noreferrer">CC BY-SA 4.0</a>
					</p>
				</div>
			</div>

			<nav class="footer-nav">
				<a href="/home">Start Learning</a>
			</nav>
		</div>
		<div class="footer-bottom">
			<p>Tamil has been spoken for over 2,000 years. Your journey starts today.</p>
		</div>
	</footer>
</div>

<style>
	:global(:root) {
		--font-serif: 'Cormorant Garamond', Georgia, serif;
		--font-heading: 'Eczar', Georgia, serif;
		--font-tamil: 'Noto Serif Tamil', serif;
		
		--c-cream: #f5f2eb;
		--c-cream-dark: #e8e4d9;
		--c-terracotta: #c45c3e;
		--c-terracotta-light: #d4684a;
		--c-ochre: #d4a373;
		--c-brown: #5c4a3d;
		--c-brown-light: #8b7355;
		--c-dark: #1c1c1c;
	}

	:global(body) {
		margin: 0;
		background: var(--c-cream);
		color: var(--c-brown);
		font-family: var(--font-serif);
		overflow-x: hidden;
	}

	.page {
		position: relative;
	}

	.texture-overlay {
		position: fixed;
		inset: 0;
		pointer-events: none;
		z-index: 1000;
		opacity: 0.06;
		mix-blend-mode: multiply;
		background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 400 400' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E");
	}

	/* Header */
	.site-header {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		padding: 1.5rem 3rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		transition: all 0.3s ease;
		background: linear-gradient(to bottom, rgba(0,0,0,0.4) 0%, rgba(0,0,0,0) 100%);
	}

	.logo {
		font-family: var(--font-heading);
		font-size: 1.8rem;
		font-weight: 700;
		color: var(--c-cream);
		letter-spacing: -0.01em;
	}

	.nav {
		display: flex;
		gap: 2.5rem;
		align-items: center;
	}

	.nav-cta {
		padding: 0.6rem 1.4rem;
		background: var(--c-terracotta);
		border-radius: 50px;
		color: white !important;
		font-weight: 600;
		box-shadow: 0 4px 15px rgba(0,0,0,0.2);
		text-decoration: none;
		transition: transform 0.2s, background 0.2s;
	}
	
	.nav-cta:hover {
		background: var(--c-terracotta-light);
		transform: translateY(-1px);
		opacity: 1 !important;
	}

	/* Hero */
	.hero {
		position: relative;
		height: 100vh;
		min-height: 700px;
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.hero-image-container {
		position: absolute;
		inset: 0;
	}

	.hero-image-wrapper {
		position: absolute;
		inset: -10%;
		will-change: transform;
	}

	.hero-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: center 40%;
		filter: brightness(0.65) saturate(0.9);
	}

	.hero-gradient {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to bottom,
			rgba(28, 28, 28, 0.4) 0%,
			rgba(28, 28, 28, 0.2) 50%,
			rgba(28, 28, 28, 0.5) 100%
		);
	}

	.hero-content {
		position: relative;
		z-index: 10;
		text-align: center;
		color: var(--c-cream);
		padding: 3rem;
		max-width: 800px;
		will-change: transform, opacity;
	}

	.hero-title {
		font-family: var(--font-heading);
		font-size: clamp(3rem, 8vw, 5.5rem);
		font-weight: 700;
		margin: 0 0 1.5rem;
		line-height: 1.1;
		opacity: 0;
		transform: translateY(30px);
		transition: all 1.2s cubic-bezier(0.22, 1, 0.36, 1) 0.2s;
		text-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	.hero-subtitle {
		font-size: clamp(1.25rem, 2.5vw, 1.6rem);
		line-height: 1.6;
		margin: 0 auto 3rem;
		font-weight: 400;
		opacity: 0;
		transform: translateY(20px);
		transition: all 1s ease 0.5s;
		max-width: 650px;
		text-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
	}

	.hero-cta {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 2rem;
		background: var(--c-terracotta);
		color: white;
		text-decoration: none;
		border-radius: 50px;
		font-family: var(--font-heading);
		transition: all 0.3s;
		box-shadow: 0 8px 30px rgba(196, 92, 62, 0.4);
		opacity: 0;
		transform: translateY(20px);
		transition: all 0.5s ease 0.8s, background 0.3s, box-shadow 0.3s, transform 0.3s;
	}

	.hero-cta:hover {
		background: var(--c-terracotta-light);
		transform: translateY(-3px);
		box-shadow: 0 12px 40px rgba(196, 92, 62, 0.5);
	}

	.mounted .hero-title,
	.mounted .hero-subtitle,
	.mounted .hero-cta {
		opacity: 1;
		transform: translateY(0);
	}

	.cta-tamil {
		font-family: var(--font-tamil);
		font-size: 1.1rem;
	}

	.cta-divider {
		width: 1px;
		height: 18px;
		background: rgba(255, 255, 255, 0.3);
	}

	.cta-english {
		font-size: 0.85rem;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.hero-transition {
		position: absolute;
		bottom: -1px;
		left: 0;
		right: 0;
		z-index: 20;
		line-height: 0;
	}

	.hero-transition svg {
		display: block;
		width: 100%;
		height: 80px;
	}

	/* Showcase Section */
	.showcase-section {
		padding: 4rem 2rem;
		display: flex;
		justify-content: center;
		background: var(--c-cream);
		position: relative;
		z-index: 10;
	}

	.section-content {
		max-width: 900px;
		margin: 0 auto;
		width: 100%;
	}

	.interface-preview {
		position: relative;
		padding: 1rem;
	}

	.poem-card {
		background: white;
		padding: 3rem;
		border-radius: 4px;
		box-shadow: 
			0 2px 4px rgba(0,0,0,0.02),
			0 15px 40px rgba(92, 74, 61, 0.08);
		text-align: center;
		position: relative;
		max-width: 600px;
		margin: 0 auto;
		border: 1px solid rgba(92, 74, 61, 0.05);
	}

	.poem-line {
		font-family: var(--font-tamil);
		font-size: clamp(1.4rem, 4vw, 1.8rem);
		color: var(--c-brown);
		margin: 0.5rem 0;
		line-height: 1.6;
	}

	.interactive-word {
		color: var(--c-terracotta);
		background: rgba(196, 92, 62, 0.1);
		padding: 0 0.25rem;
		border-radius: 4px;
		cursor: pointer;
		position: relative;
		text-decoration: underline;
		text-decoration-color: rgba(196, 92, 62, 0.3);
		text-underline-offset: 4px;
	}

	.definition-popover {
		position: absolute;
		top: 60%;
		left: 50%;
		transform: translateX(-10%);
		background: white;
		text-align: left;
		padding: 1.25rem;
		border-radius: 6px;
		box-shadow: 0 4px 25px rgba(0,0,0,0.1);
		border: 1px solid var(--c-cream-dark);
		min-width: 220px;
		z-index: 20;
		animation: floatIn 0.8s ease-out backwards;
		animation-delay: 0.5s;
	}

	.definition-popover::before {
		content: '';
		position: absolute;
		top: -6px;
		left: 20px;
		width: 12px;
		height: 12px;
		background: white;
		border-top: 1px solid var(--c-cream-dark);
		border-left: 1px solid var(--c-cream-dark);
		transform: rotate(45deg);
	}

	@keyframes floatIn {
		from { opacity: 0; transform: translate(-10%, 15px); }
		to { opacity: 1; transform: translate(-10%, 0); }
	}

	.def-header {
		display: flex;
		align-items: baseline;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid var(--c-cream);
	}

	.def-word {
		font-family: var(--font-tamil);
		font-weight: 600;
		color: var(--c-terracotta);
		font-size: 1.2rem;
	}

	.def-translit {
		font-family: var(--font-serif);
		color: var(--c-brown-light);
		font-style: italic;
		font-size: 0.95rem;
	}

	.def-body {
		font-family: var(--font-serif);
		color: var(--c-brown);
		font-size: 1.1rem;
		margin-bottom: 0.25rem;
		line-height: 1.3;
	}

	.def-meta {
		font-family: var(--font-serif);
		font-size: 0.85rem;
		color: var(--c-brown-light);
		font-style: italic;
		margin-top: 0.5rem;
	}

	/* TVA Section */
	.tva-section {
		text-align: center;
		padding: 2rem;
		background: var(--c-cream);
		padding-bottom: 4rem;
		position: relative;
		z-index: 10;
		border-top: 1px solid rgba(92, 74, 61, 0.1);
	}

	.attribution-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.attribution-text {
		font-size: 0.8rem;
		color: var(--c-brown-light);
		margin: 0;
		line-height: 1.5;
	}

	.attribution-text a {
		color: var(--c-brown);
		text-decoration: none;
		border-bottom: 1px solid rgba(92, 74, 61, 0.3);
		transition: border-color 0.2s;
	}

	.attribution-text a:hover {
		border-bottom-color: var(--c-brown);
	}

	.tva-logo {
		height: 56px;
		margin-bottom: 1rem;
		filter: grayscale(100%);
		opacity: 0.7;
		transition: all 0.3s;
	}

	.tva-logo:hover {
		filter: grayscale(0%);
		opacity: 1;
	}

	/* Footer */
	.site-footer {
		background: var(--c-dark);
		color: var(--c-cream);
		padding: 3rem 2rem 2rem;
		position: relative;
		z-index: 20;
	}

	.footer-content {
		max-width: 900px;
		margin: 0 auto;
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 2rem;
	}

	.footer-logo {
		font-family: var(--font-heading);
		font-size: 1.4rem;
		font-weight: 700;
		color: var(--c-ochre);
	}

	.footer-brand p {
		font-size: 0.9rem;
		max-width: 280px;
		margin: 0.75rem 0 0;
		opacity: 0.6;
		line-height: 1.5;
	}
	
	.footer-attribution {
		flex: 1;
		min-width: 280px;
		display: flex;
		justify-content: center;
	}

	.attribution-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		text-align: left;
		background: rgba(255, 255, 255, 0.03);
		padding: 0.75rem 1rem;
		border-radius: 8px;
		border: 1px solid rgba(255, 255, 255, 0.05);
	}

	.tva-logo-footer {
		height: 36px;
		filter: grayscale(100%);
		opacity: 0.6;
		transition: all 0.3s;
	}
	
	.attribution-row:hover .tva-logo-footer {
		filter: grayscale(0%);
		opacity: 1;
	}

	.attribution-text {
		font-size: 0.75rem;
		color: rgba(245, 242, 235, 0.5);
		margin: 0;
		line-height: 1.4;
	}

	.attribution-text a {
		color: rgba(245, 242, 235, 0.7);
		text-decoration: none;
		border-bottom: 1px solid rgba(245, 242, 235, 0.2);
		transition: all 0.2s;
	}

	.attribution-text a:hover {
		color: var(--c-cream);
		border-bottom-color: var(--c-cream);
	}

	.footer-nav {
		display: flex;
		gap: 1.5rem;
	}

	.footer-nav a {
		color: var(--c-cream);
		text-decoration: none;
		font-size: 0.85rem;
		opacity: 0.6;
		transition: opacity 0.3s;
	}

	.footer-nav a:hover {
		opacity: 1;
	}

	.footer-bottom {
		max-width: 900px;
		margin: 2rem auto 0;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		text-align: center;
	}

	.footer-bottom p {
		font-size: 0.8rem;
		opacity: 0.4;
		margin: 0;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.site-header {
			padding: 1rem;
		}

		.nav {
			gap: 1rem;
		}

		.hero-content {
			padding: 1.5rem;
		}

		.footer-content {
			flex-direction: column;
			text-align: center;
			align-items: center;
			gap: 2.5rem;
		}

		.footer-brand p {
			max-width: none;
		}
		
		.footer-attribution {
			width: 100%;
		}
		
		.attribution-row {
			width: 100%;
			justify-content: center;
		}

		.footer-nav {
			justify-content: center;
		}
	}
</style>