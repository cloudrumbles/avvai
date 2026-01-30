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
	<circle cx="12" cy="12" r="1.5" fill={color} opacity="0.6" />
	<circle cx="8" cy="8" r="1" fill={color} opacity="0.4" />
	<circle cx="16" cy="8" r="1" fill={color} opacity="0.4" />
	<circle cx="8" cy="16" r="1" fill={color} opacity="0.4" />
	<circle cx="16" cy="16" r="1" fill={color} opacity="0.4" />
</svg>
