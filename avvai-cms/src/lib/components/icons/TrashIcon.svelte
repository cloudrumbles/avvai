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
	<path d="M3 6h18M8 6V4a2 2 0 012-2h4a2 2 0 012 2v2" stroke={color} stroke-width="1.5" stroke-linecap="round" />
	<path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
	<path d="M10 11v6M14 11v6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
</svg>
