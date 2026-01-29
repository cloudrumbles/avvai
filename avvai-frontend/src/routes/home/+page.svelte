<script lang="ts">
	// Home page - user dashboard
</script>

<div class="home">
	<header class="home-header">
		<h1>Welcome back</h1>
	</header>

	<section class="section">
		<h2 class="section-title">Continue learning</h2>
		<div class="cards">
			<a href="/lesson" class="card">
				<span class="card-icon">📖</span>
				<div class="card-content">
					<h3>Lessons</h3>
					<p>Read and learn from Tamil classics</p>
				</div>
				<span class="card-arrow">→</span>
			</a>

			<a href="/flashcards" class="card">
				<span class="card-icon">🔄</span>
				<div class="card-content">
					<h3>Review</h3>
					<p>Practice with flashcards</p>
				</div>
				<span class="card-arrow">→</span>
			</a>
		</div>
	</section>

	<section class="section">
		<h2 class="section-title">Your progress</h2>
		<div class="stats">
			<div class="stat">
				<span class="stat-value">0</span>
				<span class="stat-label">Lessons completed</span>
			</div>
			<div class="stat">
				<span class="stat-value">0</span>
				<span class="stat-label">Words learned</span>
			</div>
			<div class="stat">
				<span class="stat-value">0</span>
				<span class="stat-label">Day streak</span>
			</div>
		</div>
	</section>
</div>

<style>
	.home {
		min-height: 100dvh;
		background: var(--cream);
		font-family: 'Catamaran', sans-serif;
		max-width: 640px;
		margin: 0 auto;
		padding: 24px 16px;
	}

	.home-header h1 {
		font-size: 24px;
		font-weight: 700;
		color: var(--red-deep);
		margin: 0 0 32px;
	}

	.section {
		margin-bottom: 32px;
	}

	.section-title {
		font-size: 14px;
		font-weight: 600;
		color: var(--stone);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 12px;
	}

	.cards {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 16px;
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.card:hover {
		border-color: var(--red);
		background: var(--red-faint);
	}

	.card-icon {
		font-size: 28px;
	}

	.card-content {
		flex: 1;
	}

	.card-content h3 {
		font-size: 16px;
		font-weight: 600;
		color: var(--ink);
		margin: 0 0 2px;
	}

	.card-content p {
		font-size: 14px;
		color: var(--ink-soft);
		margin: 0;
	}

	.card-arrow {
		font-size: 18px;
		color: var(--stone);
	}

	.stats {
		display: flex;
		gap: 12px;
	}

	.stat {
		flex: 1;
		background: var(--cream);
		border: 1.5px solid var(--cream-mid);
		border-radius: 12px;
		padding: 16px;
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 28px;
		font-weight: 700;
		color: var(--gold);
	}

	.stat-label {
		display: block;
		font-size: 12px;
		color: var(--stone);
		margin-top: 4px;
	}
</style>
