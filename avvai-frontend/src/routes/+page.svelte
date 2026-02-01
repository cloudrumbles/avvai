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
</svelte:head>

<div class="page" class:mounted>
	<header class="site-header">
		<div class="logo">avvai</div>
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
			<div class="footer-chip">
				<span class="chip-label">Sources</span>
				<div class="footer-attribution">
					<a href="https://www.tamilvu.org/" target="_blank" rel="noopener noreferrer">Tamil Virtual Academy</a>
					<span class="separator">·</span>
					<a href="https://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="noopener noreferrer">CC BY-SA 4.0</a>
				</div>
			</div>
		</div>
	</footer>
</div>

<style>
	:global(body) {
		margin: 0;
		background: var(--color-bg-inverse);
		color: var(--color-text-inverse);
		font-family: var(--font-serif);
		overflow-x: hidden;
	}

	.page {
		min-height: 100dvh;
		display: flex;
		flex-direction: column;
		color: var(--color-text-inverse);
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
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-5-5);
		font-weight: 700;
		color: var(--color-text-inverse);
		letter-spacing: var(--letter-tight);
		opacity: 0;
		transform: translateY(-10px);
		transition: all 0.8s cubic-bezier(0.22, 1, 0.36, 1) 0.1s;
	}

	.mounted .logo {
		opacity: 1;
		transform: translateY(0);
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
			var(--overlay-dark-95) 0%,
			var(--overlay-dark-40) 35%,
			var(--overlay-dark-10) 60%,
			var(--overlay-dark-30) 100%
		);
	}

	.hero-content {
		position: relative;
		z-index: 10;
		padding: 0 2rem 15vh;
		max-width: 600px;
	}

	.hero-title {
		font-family: 'Eczar', Georgia, serif;
		font-size: clamp(var(--font-size-10), 7vw, var(--font-size-12));
		font-weight: 600;
		margin: 0 0 2rem;
		line-height: var(--line-height-0-75);
		letter-spacing: var(--letter-tight);
		opacity: 0;
		transform: translateY(20px);
		transition: all 1s cubic-bezier(0.22, 1, 0.36, 1) 0.3s;
	}

	.mounted .hero-title {
		opacity: 1;
		transform: translateY(0);
	}

	.highlight {
		color: var(--color-accent-secondary);
	}

	.hero-cta {
		display: inline-block;
		padding: 0.9rem 2.5rem;
		background: var(--color-accent-secondary);
		color: var(--color-text-inverse);
		text-decoration: none;
		font-family: 'Eczar', Georgia, serif;
		font-size: var(--font-size-3);
		font-weight: 600;
		letter-spacing: var(--letter-3);
		border-radius: var(--radius-1);
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
		background: var(--color-accent-secondary);
		box-shadow: var(--shadow-accent-secondary);
	}

	/* Footer */
	.site-footer {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		z-index: 50;
		padding: 1rem 2rem;
		background: linear-gradient(to top, var(--overlay-dark-75), transparent);
		backdrop-filter: blur(2px);
	}

	.footer-content {
		display: flex;
		justify-content: flex-end;
		align-items: center;
	}

	.footer-chip {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.35rem 0.6rem;
		border-radius: var(--radius-pill);
		border: 1px solid var(--overlay-cream-12);
		background: linear-gradient(
			120deg,
			var(--overlay-dark-78) 0%,
			var(--overlay-dark-35) 100%
		);
		box-shadow: 0 6px 18px var(--overlay-dark-20);
		backdrop-filter: blur(4px);
	}

	.chip-label {
		text-transform: uppercase;
		letter-spacing: var(--letter-18);
		font-size: var(--font-size-0);
		color: var(--overlay-cream-45);
	}

	.footer-attribution {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		font-size: var(--font-size-0-5);
		font-weight: 400;
	}

	.footer-attribution a {
		color: var(--color-bg);
		text-decoration: none;
		opacity: 0.45;
		transition: opacity 0.2s;
		text-shadow: var(--shadow-text-inverse);
	}

	.footer-attribution a:hover {
		opacity: 0.7;
	}

	.separator {
		opacity: 0.35;
		text-shadow: var(--shadow-text-inverse);
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
			display: flex;
			flex-wrap: wrap;
			justify-content: flex-end;
			gap: 0.25rem;
		}
	}
</style>