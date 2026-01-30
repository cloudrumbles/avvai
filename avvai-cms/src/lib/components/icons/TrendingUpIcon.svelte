<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<path d="M23 6L13.5 15.5L8.5 10.5L1 18" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
	<path d="M17 6H23V12" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
