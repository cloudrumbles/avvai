<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<circle cx="12" cy="6" r="2" fill={color}/>
	<circle cx="12" cy="12" r="2" fill={color}/>
	<circle cx="12" cy="18" r="2" fill={color}/>
</svg>
