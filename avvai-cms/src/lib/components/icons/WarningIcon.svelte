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
	<circle cx="12" cy="12" r="10" stroke={color} stroke-width="1.5" />
	<path d="M12 8v5M12 16v1" stroke={color} stroke-width="1.5" stroke-linecap="round" />
</svg>
