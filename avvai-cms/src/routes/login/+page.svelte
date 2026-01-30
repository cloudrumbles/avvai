<script lang="ts">
	import { createBrowserClient } from '@supabase/ssr';
	import { env } from '$env/dynamic/public';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import KolamPattern from '$lib/components/ui/KolamPattern.svelte';
	import { CheckCircleIcon } from '$lib/components/icons';

	let email = $state('');
	let password = $state('');
	let loading = $state(false);
	let error = $state('');

	const supabase = createBrowserClient(
		env.PUBLIC_SUPABASE_URL!,
		env.PUBLIC_SUPABASE_PUBLISHABLE_DEFAULT_KEY!
	);

	async function handleLogin() {
		if (!email.trim() || !password.trim()) {
			error = 'Please enter both email and password';
			return;
		}

		loading = true;
		error = '';

		const { error: authError } = await supabase.auth.signInWithPassword({
			email: email.trim(),
			password
		});

		if (authError) {
			error = authError.message;
			loading = false;
			return;
		}

		window.location.href = '/';
	}
</script>

<div class="login-page">
	<div class="background-pattern">
		<KolamPattern variant="background" color="terracotta" />
	</div>

	<div class="login-container">
		<div class="brand-section">
			<div class="logo-container">
				<span class="logo-tamil">அவ்வை</span>
				<div class="logo-accent"></div>
			</div>
			<div class="brand-text">
				<span class="brand-label">Content Management</span>
				<p class="brand-tagline">Preserving Tamil literature with care</p>
			</div>
		</div>

		<div class="card-wrapper">
			<div class="card-glow"></div>
			<Card padding="lg">
				<div class="form-header">
					<h1 class="form-title">Welcome Back</h1>
					<p class="form-subtitle">Enter your credentials to continue</p>
				</div>

				<form class="login-form" onsubmit={(e) => { e.preventDefault(); handleLogin(); }}>
					{#if error}
						<div class="error-banner">
							<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
								<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5"/>
								<path d="M8 5v3M8 10v1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
							</svg>
							<span>{error}</span>
						</div>
					{/if}

					<div class="input-group">
						<TextInput
							label="Email Address"
							type="email"
							bind:value={email}
							placeholder="admin@avvai.edu"
						/>
					</div>

					<div class="input-group">
						<TextInput
							label="Password"
							type="password"
							bind:value={password}
							placeholder="••••••••"
						/>
					</div>

					<Button 
						type="submit" 
						fullWidth 
						disabled={loading}
						size="lg"
					>
						{#if loading}
							<span class="btn-spinner"></span>
						{/if}
						{loading ? 'Authenticating...' : 'Sign In'}
					</Button>
				</form>

				<div class="form-footer">
					<div class="security-note">
						<CheckCircleIcon size={14} color="var(--c-sage)" />
						<span>Secure admin access only</span>
					</div>
				</div>
			</Card>
		</div>

		<div class="corner-decorations">
			<KolamPattern variant="corner" size="md" color="gold" />
		</div>
	</div>
</div>

<style>
	.login-page {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, var(--c-cream) 0%, var(--c-parchment) 50%, var(--c-cream-dark) 100%);
		padding: var(--space-6);
		position: relative;
		overflow: hidden;
	}

	.background-pattern {
		position: absolute;
		inset: 0;
		opacity: 0.04;
		pointer-events: none;
	}

	.login-container {
		position: relative;
		width: 100%;
		max-width: 440px;
		display: flex;
		flex-direction: column;
		gap: var(--space-8);
		z-index: 1;
	}

	.brand-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		text-align: center;
	}

	.logo-container {
		position: relative;
		display: inline-block;
	}

	.logo-tamil {
		font-family: var(--font-tamil);
		font-size: 4rem;
		font-weight: 700;
		color: var(--c-terracotta);
		line-height: 1;
		display: block;
		text-shadow: 
			0 2px 4px rgba(184, 83, 61, 0.1),
			0 4px 8px rgba(184, 83, 61, 0.05);
	}

	.logo-accent {
		position: absolute;
		bottom: -8px;
		left: 50%;
		transform: translateX(-50%);
		width: 60px;
		height: 3px;
		background: linear-gradient(90deg, transparent 0%, var(--c-gold) 50%, transparent 100%);
		border-radius: 2px;
	}

	.brand-text {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.brand-label {
		font-family: var(--font-heading);
		font-size: 0.9rem;
		font-weight: 600;
		letter-spacing: 0.15em;
		text-transform: uppercase;
		color: var(--c-brown);
	}

	.brand-tagline {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-muted);
		font-style: italic;
		margin: 0;
	}

	.card-wrapper {
		position: relative;
	}

	.card-glow {
		position: absolute;
		inset: -20px;
		background: radial-gradient(
			ellipse at center,
			rgba(184, 83, 61, 0.08) 0%,
			transparent 70%
		);
		pointer-events: none;
	}

	.form-header {
		text-align: center;
		margin-bottom: var(--space-6);
	}

	.form-title {
		font-family: var(--font-heading);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--c-brown-dark);
		margin: 0 0 var(--space-2);
	}

	.form-subtitle {
		font-family: var(--font-serif);
		font-size: 0.95rem;
		color: var(--c-brown-muted);
		margin: 0;
	}

	.login-form {
		display: flex;
		flex-direction: column;
		gap: var(--space-5);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3);
		background: var(--c-error-bg);
		border: 1px solid rgba(184, 83, 61, 0.15);
		border-radius: var(--radius-sm);
		color: var(--c-terracotta);
		font-size: 0.9rem;
	}

	.input-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.form-footer {
		margin-top: var(--space-6);
		padding-top: var(--space-5);
		border-top: 1px solid rgba(92, 74, 61, 0.08);
	}

	.security-note {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		font-size: 0.8rem;
		color: var(--c-sage);
		font-family: var(--font-body);
	}

	.corner-decorations {
		position: absolute;
		bottom: -40px;
		right: -40px;
		opacity: 0.15;
		transform: rotate(180deg);
		pointer-events: none;
	}
</style>
