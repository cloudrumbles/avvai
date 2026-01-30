<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<circle cx="11" cy="11" r="7" stroke={color} stroke-width="2"/>
	<path d="M20 20L16 16" stroke={color} stroke-width="2" stroke-linecap="round"/>
</svg>
