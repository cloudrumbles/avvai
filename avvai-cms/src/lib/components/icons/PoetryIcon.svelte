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
	<path d="M6 4v12M12 6v10M18 8v8" stroke={color} stroke-width="1.5" stroke-linecap="round" />
	<circle cx="6" cy="18" r="1.5" fill={color} />
	<circle cx="12" cy="18" r="1.5" fill={color} />
	<circle cx="18" cy="18" r="1.5" fill={color} />
</svg>
