<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<path d="M3 6H21" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M7 12H17" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M10 18H14" stroke={color} stroke-width="2" stroke-linecap="round"/>
</svg>
