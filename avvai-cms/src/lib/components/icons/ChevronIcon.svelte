<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
		class?: string;
		expanded?: boolean;
	}

	let { size = 16, color = 'currentColor', class: className = '', expanded = false }: Props = $props();
</script>

<svg
	class={className}
	class:expanded
	width={size}
	height={size}
	viewBox="0 0 24 24"
	fill="none"
	xmlns="http://www.w3.org/2000/svg"
>
	<path
		d={expanded ? "M6 9l6 6 6-6" : "M9 6l6 6-6 6"}
		stroke={color}
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	/>
</svg>

<style>
	svg {
		transition: transform 0.2s ease;
	}
</style>
