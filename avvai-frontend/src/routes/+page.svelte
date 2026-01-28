<script lang="ts">
	// Landing page - marketing/intro for visitors
</script>

<svelte:head>
	<title>ஆவ்வை - Learn Tamil</title>
	<meta name="description" content="Learn Tamil through classical literature" />
</svelte:head>

<div class="landing">
	<header class="hero">
		<h1 class="hero-title">ஆவ்வை</h1>
		<p class="hero-subtitle">Learn Tamil through classical literature</p>
	</header>

	<section class="features">
		<div class="feature">
			<span class="feature-icon">📖</span>
			<h2>Read</h2>
			<p>Immerse yourself in Thirukkural and other Tamil classics with word-by-word definitions</p>
		</div>
		<div class="feature">
			<span class="feature-icon">🎯</span>
			<h2>Learn</h2>
			<p>Structured lessons that build your vocabulary and grammar step by step</p>
		</div>
		<div class="feature">
			<span class="feature-icon">🔄</span>
			<h2>Review</h2>
			<p>Spaced repetition flashcards to make sure you remember what you learn</p>
		</div>
	</section>

	<div class="cta">
		<a href="/home" class="cta-btn">Get started</a>
	</div>
</div>

<style>
	.landing {
		--ink: #1a0e06;
		--ink-soft: #3a2a1a;
		--cream: #faf3e6;
		--cream-mid: #f0e4cc;
		--red: #8b1a1a;
		--red-deep: #5c0e0e;
		--gold: #c5941a;

		min-height: 100dvh;
		background: var(--cream);
		font-family: 'Catamaran', sans-serif;
	}

	.hero {
		text-align: center;
		padding: 80px 24px 48px;
	}

	.hero-title {
		font-size: 48px;
		font-weight: 700;
		color: var(--red-deep);
		margin: 0 0 12px;
	}

	.hero-subtitle {
		font-size: 18px;
		color: var(--ink-soft);
		margin: 0;
	}

	.features {
		display: flex;
		flex-direction: column;
		gap: 24px;
		max-width: 640px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.feature {
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		padding: 24px;
		text-align: center;
	}

	.feature-icon {
		font-size: 32px;
		display: block;
		margin-bottom: 12px;
	}

	.feature h2 {
		font-size: 18px;
		font-weight: 600;
		color: var(--ink);
		margin: 0 0 8px;
	}

	.feature p {
		font-size: 14px;
		color: var(--ink-soft);
		margin: 0;
		line-height: 1.5;
	}

	.cta {
		text-align: center;
		padding: 48px 24px 80px;
	}

	.cta-btn {
		display: inline-block;
		font-family: 'Catamaran', sans-serif;
		font-size: 16px;
		font-weight: 600;
		color: var(--cream);
		background: var(--red-deep);
		border-radius: 12px;
		padding: 14px 32px;
		text-decoration: none;
		transition: background 0.15s ease;
	}

	.cta-btn:hover {
		background: var(--red);
	}

	@media (min-width: 600px) {
		.features {
			flex-direction: row;
		}

		.feature {
			flex: 1;
		}

		.hero-title {
			font-size: 64px;
		}

		.hero-subtitle {
			font-size: 20px;
		}
	}
</style>
