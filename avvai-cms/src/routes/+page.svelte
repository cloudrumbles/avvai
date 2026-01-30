<script lang="ts">
	import { onMount } from 'svelte';
	import { 
		PlusIcon, 
		ClockIcon, 
		ActivityIcon, 
		TrendingUpIcon,
		BookOpenIcon,
		MediaIcon,
		UsersIcon,
		AlertIcon,
		CheckCircleIcon,
		ProseIcon,
		PoetryIcon,
		DialogueIcon
	} from '$lib/components/icons';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';

	let lessonCount = $state(0);
	let mediaCount = $state(0);
	let cacheCount = $state(0);
	let recentLessons = $state<{id: string, title: string, type: string, updatedAt: string}[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			const [lessonsRes, mediaRes, cacheRes] = await Promise.all([
				fetch('/api/admin/lessons'),
				fetch('/api/admin/media'),
				fetch('/api/admin/dictionary-cache')
			]);

			if (lessonsRes.ok) {
				const lessons = await lessonsRes.json();
				lessonCount = Array.isArray(lessons) ? lessons.length : 0;
				// Mock recent activity - in real app, you'd track this
				recentLessons = Array.isArray(lessons) ? lessons.slice(0, 3).map((l: {id: string, title: string}) => ({
					id: l.id,
					title: l.title || l.id,
					type: 'prose',
					updatedAt: '2 hours ago'
				})) : [];
			}

			if (mediaRes.ok) {
				const media = await mediaRes.json();
				mediaCount = Array.isArray(media) ? media.length : 0;
			}

			if (cacheRes.ok) {
				const cache = await cacheRes.json();
				cacheCount = Array.isArray(cache) ? cache.length : 0;
			}
		} catch (e) {
			console.error('Failed to fetch dashboard data', e);
		} finally {
			loading = false;
		}
	});

	const quickActions = [
		{ label: 'Prose Lesson', icon: ProseIcon, href: '/lessons/new?type=prose', color: '--c-prose' },
		{ label: 'Poetry Lesson', icon: PoetryIcon, href: '/lessons/new?type=poetry', color: '--c-poetry' },
		{ label: 'Dialogue', icon: DialogueIcon, href: '/lessons/new?type=dialogue', color: '--c-dialogue' },
	];

	const stats = $derived([
		{ label: 'Lessons', value: lessonCount, icon: BookOpenIcon, trend: '+3 this month' },
		{ label: 'Media Files', value: mediaCount, icon: MediaIcon, trend: '+12 this month' },
		{ label: 'Students', value: 156, icon: UsersIcon, trend: '+24 this week' },
	]);
</script>

<div class="dashboard">
	<div class="dashboard-header">
		<div>
			<h1 class="page-title">Dashboard</h1>
			<p class="page-date">{new Date().toLocaleDateString('en-US', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}</p>
		</div>
	</div>

	{#if loading}
		<div class="loading-state">
			<div class="loader">
				<div class="loader-ring"></div>
				<div class="loader-ring"></div>
				<div class="loader-ring"></div>
			</div>
			<p class="loading-text">Gathering your content...</p>
		</div>
	{:else}
		<div class="dashboard-grid">
			<div class="main-column">
				<section class="section">
					<h2 class="section-title">Quick Actions</h2>
					<Card padding="lg">
						<div class="quick-actions">
							<a href="/lessons/new" class="action-primary">
								<div class="action-icon">
									<PlusIcon size={24} color="white" />
								</div>
								<div class="action-content">
									<span class="action-label">Create New Lesson</span>
									<span class="action-description">Start crafting Tamil literary content</span>
								</div>
							</a>
							<div class="action-divider"></div>
							<div class="action-secondary">
								{#each quickActions as action}
									<a href={action.href} class="action-chip" style="--chip-color: var({action.color})">
										<action.icon size={16} color="currentColor" />
										<span>{action.label}</span>
									</a>
								{/each}
							</div>
						</div>
					</Card>
				</section>

				<section class="section">
					<div class="section-header">
						<h2 class="section-title">Recent Activity</h2>
						<a href="/lessons" class="view-all">View all →</a>
					</div>
					<Card padding="lg">
						{#if recentLessons.length === 0}
							<div class="empty-activity">
								<ActivityIcon size={32} color="var(--c-brown-muted)" />
								<p>No recent activity yet</p>
								<a href="/lessons/new">Create your first lesson</a>
							</div>
						{:else}
							<div class="activity-list">
								{#each recentLessons as lesson}
									<div class="activity-item">
										<div class="activity-icon">
											<BookOpenIcon size={18} color="var(--c-terracotta)" />
										</div>
										<div class="activity-content">
											<span class="activity-title">{lesson.title}</span>
											<span class="activity-meta">
												<span class="activity-type">{lesson.type}</span>
												<span class="activity-time">
													<ClockIcon size={12} color="var(--c-brown-muted)" />
													{lesson.updatedAt}
												</span>
											</span>
										</div>
										<Button 
											variant="ghost" 
											size="sm"
											onclick={() => window.location.href = `/lessons/${lesson.id}`}
										>
											Edit
										</Button>
									</div>
								{/each}
							</div>
						{/if}
					</Card>
				</section>
			</div>

			<div class="side-column">
				<section class="section">
					<h2 class="section-title">Your Stats</h2>
					<div class="stats-grid">
						{#each stats as stat}
							<Card padding="md">
								<div class="stat-card">
									<div class="stat-header">
										<stat.icon size={18} color="var(--c-terracotta)" />
										<span class="stat-trend">
											<TrendingUpIcon size={12} color="var(--c-sage)" />
										</span>
									</div>
									<span class="stat-value">{stat.value}</span>
									<span class="stat-label">{stat.label}</span>
									<span class="stat-trend-text">{stat.trend}</span>
								</div>
							</Card>
						{/each}
					</div>
				</section>

				<section class="section">
					<h2 class="section-title">Needs Attention</h2>
					<div class="attention-list">
						<div class="attention-item">
							<AlertIcon size={16} color="var(--c-warning)" />
							<span>3 lessons have unpublished changes</span>
							<a href="/lessons">Review →</a>
						</div>
						<div class="attention-item resolved">
							<CheckCircleIcon size={16} color="var(--c-sage)" />
							<span>All media files processed</span>
						</div>
					</div>
				</section>
			</div>
		</div>
	{/if}
</div>

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		max-width: 1400px;
	}

	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 700;
		color: var(--c-brown-dark);
		margin: 0;
	}

	.page-date {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-muted);
		margin: 0.25rem 0 0;
		font-style: italic;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 0;
		gap: 1rem;
	}

	.loader {
		position: relative;
		width: 48px;
		height: 48px;
	}

	.loader-ring {
		position: absolute;
		inset: 0;
		border: 2px solid transparent;
		border-top-color: var(--c-terracotta);
		border-radius: 50%;
		animation: spin 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
	}

	.loader-ring:nth-child(2) {
		inset: 6px;
		animation-delay: -0.15s;
		border-top-color: var(--c-gold);
	}

	.loader-ring:nth-child(3) {
		inset: 12px;
		animation-delay: -0.3s;
		border-top-color: var(--c-sage);
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.loading-text {
		font-family: var(--font-serif);
		font-size: 1rem;
		color: var(--c-brown-light);
		font-style: italic;
	}

	.dashboard-grid {
		display: grid;
		grid-template-columns: 1fr 320px;
		gap: 2rem;
	}

	@media (max-width: 1024px) {
		.dashboard-grid {
			grid-template-columns: 1fr;
		}
	}

	.main-column {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.side-column {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.section {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.section-title {
		font-family: var(--font-heading);
		font-size: 0.85rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--c-brown);
		margin: 0;
	}

	.view-all {
		font-size: 0.85rem;
		color: var(--c-terracotta);
		font-weight: 500;
		transition: color 0.15s ease;
	}

	.view-all:hover {
		color: var(--c-terracotta-dark);
	}

	/* Quick Actions */
	.quick-actions {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.action-primary {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: linear-gradient(135deg, var(--c-terracotta) 0%, var(--c-terracotta-dark) 100%);
		border-radius: var(--radius-md);
		color: white;
		transition: all var(--transition-base);
	}

	.action-primary:hover {
		transform: translateY(-2px);
		box-shadow: var(--shadow-md);
	}

	.action-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: rgba(255, 255, 255, 0.2);
		border-radius: var(--radius-sm);
	}

	.action-content {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.action-label {
		font-family: var(--font-heading);
		font-size: 1.1rem;
		font-weight: 600;
	}

	.action-description {
		font-size: 0.85rem;
		opacity: 0.8;
	}

	.action-divider {
		height: 1px;
		background: rgba(92, 74, 61, 0.1);
		margin: 0.5rem 0;
	}

	.action-secondary {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.action-chip {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.875rem;
		background: rgba(var(--chip-color), 0.1);
		border: 1px solid rgba(var(--chip-color), 0.2);
		border-radius: var(--radius-full);
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--c-brown);
		transition: all var(--transition-fast);
	}

	.action-chip:hover {
		background: rgba(var(--chip-color), 0.15);
		transform: translateY(-1px);
	}

	/* Activity List */
	.empty-activity {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 2rem;
		text-align: center;
	}

	.empty-activity p {
		margin: 0;
		color: var(--c-brown-muted);
		font-family: var(--font-serif);
		font-style: italic;
	}

	.empty-activity a {
		color: var(--c-terracotta);
		font-weight: 500;
	}

	.activity-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.activity-item {
		display: flex;
		align-items: center;
		gap: 0.875rem;
		padding: 0.75rem;
		border-radius: var(--radius-sm);
		transition: background var(--transition-fast);
	}

	.activity-item:hover {
		background: var(--c-cream-dark);
	}

	.activity-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: var(--c-exercises-bg);
		border-radius: var(--radius-sm);
		flex-shrink: 0;
	}

	.activity-content {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
		flex: 1;
		min-width: 0;
	}

	.activity-title {
		font-weight: 500;
		color: var(--c-brown-dark);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.activity-meta {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.8rem;
		color: var(--c-brown-muted);
	}

	.activity-type {
		text-transform: capitalize;
	}

	.activity-time {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	/* Stats */
	.stats-grid {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.stat-card {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.stat-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.stat-trend {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: var(--c-success-bg);
		border-radius: var(--radius-full);
	}

	.stat-value {
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 700;
		color: var(--c-brown-dark);
		line-height: 1;
	}

	.stat-label {
		font-size: 0.85rem;
		color: var(--c-brown-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.stat-trend-text {
		font-size: 0.75rem;
		color: var(--c-sage);
		font-weight: 500;
	}

	/* Attention List */
	.attention-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.attention-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: var(--c-warning-bg);
		border-radius: var(--radius-md);
		font-size: 0.9rem;
		color: var(--c-brown);
	}

	.attention-item.resolved {
		background: var(--c-success-bg);
	}

	.attention-item span {
		flex: 1;
	}

	.attention-item a {
		font-size: 0.8rem;
		color: var(--c-terracotta);
		font-weight: 500;
		white-space: nowrap;
	}
</style>
