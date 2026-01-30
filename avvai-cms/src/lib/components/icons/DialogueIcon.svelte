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
	<path
		d="M4 5h12a2 2 0 012 2v6a2 2 0 01-2 2H8l-4 3v-3H4a2 2 0 01-2-2V7a2 2 0 012-2z"
		stroke={color}
		stroke-width="1.5"
	/>
	<path
		d="M20 9v4a2 2 0 01-2 2h-2"
		stroke={color}
		stroke-width="1.5"
		stroke-linecap="round"
	/>
</svg>
