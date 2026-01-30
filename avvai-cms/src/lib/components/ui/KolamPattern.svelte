<script lang="ts">
	interface Props {
		variant?: 'corner' | 'border' | 'background' | 'accent' | 'dot';
		size?: 'sm' | 'md' | 'lg';
		color?: 'terracotta' | 'gold' | 'sage' | 'brown';
		class?: string;
	}

	let {
		variant = 'accent',
		size = 'md',
		color = 'terracotta',
		class: className = ''
	}: Props = $props();

	const colorMap = {
		terracotta: '#B8533D',
		gold: '#C99A2E',
		sage: '#5F7A61',
		brown: '#5C4A3D'
	};

	const sizeMap = {
		sm: 24,
		md: 48,
		lg: 96
	};

	let fillColor = $derived(colorMap[color]);
	let dimensions = $derived(sizeMap[size]);
</script>

{#if variant === 'corner'}
	<svg
		class="kolam-corner {className}"
		width={dimensions}
		height={dimensions}
		viewBox="0 0 48 48"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<!-- Traditional pulli kolam style corner pattern -->
		<circle cx="8" cy="8" r="2" fill={fillColor} opacity="0.6" />
		<circle cx="24" cy="8" r="2" fill={fillColor} opacity="0.6" />
		<circle cx="8" cy="24" r="2" fill={fillColor} opacity="0.6" />
		<circle cx="40" cy="8" r="2" fill={fillColor} opacity="0.4" />
		<circle cx="8" cy="40" r="2" fill={fillColor} opacity="0.4" />
		<path
			d="M8 8 L24 8 M8 8 L8 24"
			stroke={fillColor}
			stroke-width="1.5"
			opacity="0.8"
			stroke-linecap="round"
		/>
		<path
			d="M24 8 Q32 8 32 16 M8 24 Q8 32 16 32"
			stroke={fillColor}
			stroke-width="1.5"
			opacity="0.6"
			stroke-linecap="round"
			fill="none"
		/>
		<path
			d="M32 16 Q40 16 40 24 M16 32 Q16 40 24 40"
			stroke={fillColor}
			stroke-width="1"
			opacity="0.4"
			stroke-linecap="round"
			fill="none"
		/>
	</svg>

{:else if variant === 'border'}
	<svg
		class="kolam-border {className}"
		width="100%"
		height={dimensions / 2}
		viewBox="0 0 200 12"
		preserveAspectRatio="none"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<!-- Repeating geometric border pattern -->
		<path
			d="M0 6 L20 6 M20 2 L20 10 M25 6 Q30 2 35 6 Q40 10 45 6 M50 6 L70 6"
			stroke={fillColor}
			stroke-width="1"
			opacity="0.5"
			stroke-linecap="round"
		/>
		<circle cx="75" cy="6" r="2" fill={fillColor} opacity="0.6" />
		<path
			d="M80 6 L100 6"
			stroke={fillColor}
			stroke-width="1"
			opacity="0.5"
			stroke-linecap="round"
		/>
		<circle cx="105" cy="6" r="2" fill={fillColor} opacity="0.6" />
		<path
			d="M110 6 L130 6 M130 2 L130 10 M135 6 Q140 2 145 6 Q150 10 155 6 M160 6 L180 6"
			stroke={fillColor}
			stroke-width="1"
			opacity="0.5"
			stroke-linecap="round"
		/>
		<circle cx="185" cy="6" r="2" fill={fillColor} opacity="0.6" />
		<path
			d="M190 6 L200 6"
			stroke={fillColor}
			stroke-width="1"
			opacity="0.5"
			stroke-linecap="round"
		/>
	</svg>

{:else if variant === 'background'}
	<svg
		class="kolam-background {className}"
		width="100%"
		height="100%"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
		preserveAspectRatio="xMidYMid slice"
	>
		<pattern id="kolam-pattern" x="0" y="0" width="60" height="60" patternUnits="userSpaceOnUse">
			<circle cx="30" cy="30" r="1.5" fill={fillColor} opacity="0.15" />
			<circle cx="10" cy="10" r="1" fill={fillColor} opacity="0.1" />
			<circle cx="50" cy="10" r="1" fill={fillColor} opacity="0.1" />
			<circle cx="10" cy="50" r="1" fill={fillColor} opacity="0.1" />
			<circle cx="50" cy="50" r="1" fill={fillColor} opacity="0.1" />
			<path
				d="M10 10 L30 30 L50 10 M10 50 L30 30 L50 50"
				stroke={fillColor}
				stroke-width="0.5"
				opacity="0.08"
				stroke-linecap="round"
				fill="none"
			/>
		</pattern>
		<rect width="100%" height="100%" fill="url(#kolam-pattern)" />
	</svg>

{:else if variant === 'dot'}
	<svg
		class="kolam-dot {className}"
		width={dimensions}
		height={dimensions}
		viewBox="0 0 24 24"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<!-- Simple dot pattern - traditional pulli -->
		<circle cx="6" cy="6" r="1.5" fill={fillColor} opacity="0.8" />
		<circle cx="18" cy="6" r="1.5" fill={fillColor} opacity="0.8" />
		<circle cx="6" cy="18" r="1.5" fill={fillColor} opacity="0.8" />
		<circle cx="18" cy="18" r="1.5" fill={fillColor} opacity="0.8" />
		<circle cx="12" cy="12" r="2" fill={fillColor} opacity="0.6" />
		<path
			d="M6 6 L18 6 L18 18 L6 18 Z"
			stroke={fillColor}
			stroke-width="0.5"
			opacity="0.3"
			stroke-dasharray="2 2"
			fill="none"
		/>
	</svg>

{:else}
	<!-- accent variant - small decorative element -->
	<svg
		class="kolam-accent {className}"
		width={dimensions}
		height={dimensions}
		viewBox="0 0 48 48"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<!-- Geometric floral/kolam accent -->
		<circle cx="24" cy="24" r="16" stroke={fillColor} stroke-width="1" opacity="0.3" fill="none" />
		<circle cx="24" cy="24" r="10" stroke={fillColor} stroke-width="1" opacity="0.5" fill="none" />
		<circle cx="24" cy="24" r="4" fill={fillColor} opacity="0.6" />
		<path
			d="M24 4 L24 12 M24 36 L24 44 M4 24 L12 24 M36 24 L44 24"
			stroke={fillColor}
			stroke-width="1.5"
			opacity="0.4"
			stroke-linecap="round"
		/>
		<path
			d="M10 10 L16 16 M38 10 L32 16 M10 38 L16 32 M38 38 L32 32"
			stroke={fillColor}
			stroke-width="1"
			opacity="0.3"
			stroke-linecap="round"
		/>
	</svg>
{/if}

<style>
	.kolam-corner,
	.kolam-accent,
	.kolam-dot {
		display: block;
	}

	.kolam-border {
		display: block;
		width: 100%;
	}

	.kolam-background {
		position: absolute;
		inset: 0;
		pointer-events: none;
		opacity: 0.5;
	}
</style>
