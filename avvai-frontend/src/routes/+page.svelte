<script lang="ts">
	let mounted = $state(false);

	$effect(() => {
		mounted = true;
	});
</script>

<svelte:window />

<svelte:head>
	<title>avvai — Learn Tamil</title>
	<meta name="description" content="Learn Tamil without the pain" />
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,600;1,400&family=Eczar:wght@400;600;700&display=swap" rel="stylesheet" />
</svelte:head>

<div class="page" class:mounted>
	<header class="site-header">
		<div class="logo">avvai</div>
		<nav class="nav">
			<a href="/home" class="nav-link">Start</a>
		</nav>
	</header>

	<section class="hero">
		<div class="hero-image-container">
			<img
				src="/marutham-000.png"
				alt="A Tamil village scene"
				class="hero-image"
			/>
			<div class="hero-gradient"></div>
		</div>

		<div class="hero-content">
			<h1 class="hero-title">
				Learn Tamil<br/>
				<span class="highlight">without the pain.</span>
			</h1>
			<a href="/home" class="hero-cta">
				Begin
			</a>
		</div>
	</section>

	<footer class="site-footer">
		<div class="footer-content">
			<span class="footer-logo">avvai</span>
			<div class="footer-attribution">
				<a href="https://www.tamilvu.org/" target="_blank" rel="noopener noreferrer">Tamil Virtual Academy</a>
				<span class="separator">·</span>
				<a href="https://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="noopener noreferrer">CC BY-SA 4.0</a>
			</div>
		</div>
	</footer>
</div>

<style>
	:global(:root) {
		--font-serif: 'Cormorant Garamond', Georgia, serif;
		--font-heading: 'Eczar', Georgia, serif;

		--c-leaf: #e8e0c8;            /* palm leaf */
		--c-cream: #f2edd8;
		--c-terracotta: #c45c3e;
		--c-terracotta-light: #d4684a;
		--c-brown: #5c4a3d;
		--c-dark: #1c1c1c;
	}

	:global(body) {
		margin: 0;
		background: var(--c-dark);
		color: var(--c-cream);
		font-family: var(--font-serif);
		overflow-x: hidden;
	}

	.page {
		min-height: 100dvh;
		display: flex;
		flex-direction: column;
	}

	/* Header */
	.site-header {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		padding: 1.25rem 2rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.logo {
		font-family: var(--font-heading);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--c-cream);
		letter-spacing: -0.01em;
		opacity: 0;
		transform: translateY(-10px);
		transition: all 0.8s cubic-bezier(0.22, 1, 0.36, 1) 0.1s;
	}

	.mounted .logo {
		opacity: 1;
		transform: translateY(0);
	}

	.nav-link {
		font-family: var(--font-heading);
		font-size: 0.9rem;
		color: var(--c-cream);
		text-decoration: none;
		opacity: 0.7;
		transition: opacity 0.2s;
		letter-spacing: 0.02em;
	}

	.nav-link:hover {
		opacity: 1;
	}

	/* Hero */
	.hero {
		flex: 1;
		position: relative;
		display: flex;
		align-items: flex-end;
		justify-content: flex-start;
		min-height: 100dvh;
	}

	.hero-image-container {
		position: absolute;
		inset: 0;
	}

	.hero-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: center 35%;
		filter: brightness(0.55) saturate(0.85);
	}

	.hero-gradient {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to top,
			rgba(28, 28, 28, 0.95) 0%,
			rgba(28, 28, 28, 0.4) 35%,
			rgba(28, 28, 28, 0.1) 60%,
			rgba(28, 28, 28, 0.3) 100%
		);
	}

	.hero-content {
		position: relative;
		z-index: 10;
		padding: 0 2rem 15vh;
		max-width: 600px;
	}

	.hero-title {
		font-family: var(--font-heading);
		font-size: clamp(2.5rem, 7vw, 4rem);
		font-weight: 600;
		margin: 0 0 2rem;
		line-height: 1.15;
		letter-spacing: -0.01em;
		opacity: 0;
		transform: translateY(20px);
		transition: all 1s cubic-bezier(0.22, 1, 0.36, 1) 0.3s;
	}

	.mounted .hero-title {
		opacity: 1;
		transform: translateY(0);
	}

	.highlight {
		color: var(--c-terracotta);
	}

	.hero-cta {
		display: inline-block;
		padding: 0.9rem 2.5rem;
		background: var(--c-terracotta);
		color: white;
		text-decoration: none;
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 600;
		letter-spacing: 0.03em;
		border-radius: 4px;
		opacity: 0;
		transform: translateY(15px);
		transition: all 0.8s cubic-bezier(0.22, 1, 0.36, 1) 0.5s,
		            background 0.2s ease,
		            box-shadow 0.2s ease;
	}

	.mounted .hero-cta {
		opacity: 1;
		transform: translateY(0);
	}

	.hero-cta:hover {
		background: var(--c-terracotta-light);
		box-shadow: 0 8px 30px rgba(196, 92, 62, 0.35);
	}

	/* Footer */
	.site-footer {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		z-index: 50;
		padding: 1rem 2rem;
	}

	.footer-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.footer-logo {
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 600;
		color: var(--c-cream);
		opacity: 0.3;
	}

	.footer-attribution {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
	}

	.footer-attribution a {
		color: var(--c-cream);
		text-decoration: none;
		opacity: 0.3;
		transition: opacity 0.2s;
	}

	.footer-attribution a:hover {
		opacity: 0.6;
	}

	.separator {
		opacity: 0.2;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.site-header {
			padding: 1rem 1.25rem;
		}

		.hero-content {
			padding: 0 1.25rem 12vh;
		}

		.site-footer {
			padding: 1rem 1.25rem;
		}

		.footer-attribution {
			display: none;
		}
	}
</style>
