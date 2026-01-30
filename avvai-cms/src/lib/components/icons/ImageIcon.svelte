<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<rect x="3" y="3" width="18" height="18" rx="2" stroke={color} stroke-width="2"/>
	<circle cx="8.5" cy="8.5" r="2" stroke={color} stroke-width="2"/>
	<path d="M21 15L16 10L3 21" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
