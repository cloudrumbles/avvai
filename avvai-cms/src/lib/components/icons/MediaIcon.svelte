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
	<rect x="3" y="4" width="18" height="16" rx="2" stroke={color} stroke-width="1.5" />
	<circle cx="8" cy="9" r="2" stroke={color} stroke-width="1.5" />
	<path
		d="M3 15l5-3 4 2 5-3 4 2"
		stroke={color}
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	/>
</svg>
