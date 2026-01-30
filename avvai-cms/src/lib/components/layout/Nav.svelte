<script lang="ts">
	import { page } from '$app/state';
	import {
		DashboardIcon,
		BookOpenIcon,
		MediaIcon,
		DialogueIcon
	} from '$lib/components/icons';

	const links = [
		{
			href: '/',
			label: 'Dashboard',
			icon: DashboardIcon
		},
		{
			href: '/lessons',
			label: 'Lessons',
			icon: BookOpenIcon
		},
		{
			href: '/media',
			label: 'Media',
			icon: MediaIcon
		},
		{
			href: '/dictionary',
			label: 'Dictionary',
			icon: DialogueIcon
		}
	];
	const pathname = $derived(page.url.pathname);
</script>

<nav class="compact-nav">
	<div class="brand">
		<span class="logo-tamil">அவ்வை</span>
		<span class="label">CMS</span>
	</div>

	<div class="nav-links">
		{#each links as link}
			<a
				href={link.href}
				class="nav-link"
				class:active={pathname === link.href ||
					(link.href !== '/' && pathname.startsWith(link.href + '/'))}
			>
				<span class="link-icon">
					<link.icon size={16} color="currentColor" />
				</span>
				<span class="link-label">{link.label}</span>
			</a>
		{/each}
	</div>
</nav>

<style>
	.compact-nav {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.brand {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		padding-bottom: var(--space-3);
		border-bottom: 1px solid rgba(92, 74, 61, 0.1);
	}

	.logo-tamil {
		font-family: var(--font-tamil);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--c-terracotta);
		line-height: 1;
	}

	.label {
		font-family: var(--font-heading);
		font-size: 0.65rem;
		font-weight: 600;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: var(--c-brown-muted);
	}

	.nav-links {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-sm);
		color: var(--c-brown);
		font-size: 0.85rem;
		font-weight: 500;
		transition: all var(--transition-fast);
		text-decoration: none;
	}

	.nav-link:hover {
		background: rgba(92, 74, 61, 0.06);
		color: var(--c-terracotta);
	}

	.nav-link.active {
		background: var(--c-terracotta);
		color: white;
	}

	.link-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
	}

	.link-label {
		font-family: var(--font-heading);
	}
</style>
