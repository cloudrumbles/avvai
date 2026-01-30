<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
		class?: string;
	}

	let { size = 16, color = 'currentColor', class: className = '' }: Props = $props();
</script>

<svg
	class={className}
	width={size}
	height={size}
	viewBox="0 0 24 24"
	fill="none"
	xmlns="http://www.w3.org/2000/svg"
>
	<circle cx="6" cy="12" r="2" stroke={color} stroke-width="1.5" />
	<circle cx="18" cy="6" r="2" stroke={color} stroke-width="1.5" />
	<circle cx="18" cy="12" r="2" stroke={color} stroke-width="1.5" />
	<circle cx="18" cy="18" r="2" stroke={color} stroke-width="1.5" />
	<path d="M9 12h5M9 6h5M9 18h5" stroke={color} stroke-width="1.5" stroke-linecap="round" />
</svg>
