<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<rect x="3" y="3" width="7" height="7" rx="2" stroke={color} stroke-width="2"/>
	<rect x="14" y="3" width="7" height="7" rx="2" stroke={color} stroke-width="2"/>
	<rect x="3" y="14" width="7" height="7" rx="2" stroke={color} stroke-width="2"/>
	<rect x="14" y="14" width="7" height="7" rx="2" stroke={color} stroke-width="2"/>
</svg>
