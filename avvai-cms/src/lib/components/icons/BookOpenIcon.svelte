<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<path d="M2 3H8C9.65685 3 11 4.34315 11 6V21C11 19.3431 9.65685 18 8 18H2V3Z" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
	<path d="M22 3H16C14.3431 3 13 4.34315 13 6V21C13 19.3431 14.3431 18 16 18H22V3Z" stroke={color} stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
	<path d="M8 7H4" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M20 7H16" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M8 11H4" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M20 11H16" stroke={color} stroke-width="2" stroke-linecap="round"/>
</svg>
