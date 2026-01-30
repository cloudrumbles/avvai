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
	<path d="M7 9h10M7 13h8M7 17h6" stroke={color} stroke-width="1.5" stroke-linecap="round" />
</svg>
