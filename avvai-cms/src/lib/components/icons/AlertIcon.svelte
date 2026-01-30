<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<circle cx="12" cy="12" r="10" stroke={color} stroke-width="2"/>
	<path d="M12 8V12" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<circle cx="12" cy="16" r="1" fill={color}/>
</svg>
