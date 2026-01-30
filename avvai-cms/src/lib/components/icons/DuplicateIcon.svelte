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
	<rect x="9" y="9" width="12" height="12" rx="2" stroke={color} stroke-width="1.5" />
	<path d="M5 15V5a2 2 0 012-2h10" stroke={color} stroke-width="1.5" stroke-linecap="round" />
</svg>
