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
	<path d="M4 8h6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
	<path d="M14 8h6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
	<path d="M4 12h16" stroke={color} stroke-width="1.5" stroke-linecap="round" stroke-dasharray="3 2" />
	<path d="M4 16h6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
	<path d="M14 16h6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
</svg>
