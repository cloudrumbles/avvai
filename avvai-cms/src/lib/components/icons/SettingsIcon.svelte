<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<circle cx="12" cy="12" r="3" stroke={color} stroke-width="2"/>
	<path d="M19 12H21" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M3 12H5" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M12 5V3" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M12 21V19" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M16.95 7.05L18.36 5.64" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M5.64 18.36L7.05 16.95" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M16.95 16.95L18.36 18.36" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M5.64 5.64L7.05 7.05" stroke={color} stroke-width="2" stroke-linecap="round"/>
</svg>
