<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<path d="M22 12H18L15 21L9 3L6 12H2" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
